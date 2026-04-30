use crate::at_parser::{
    self, extract_data_lines, is_ok, parse_c5gqosrdp, parse_cgact, parse_cgsn, parse_cgmi,
    parse_cgmm, parse_cops, parse_ccid, parse_cpin, parse_gmr, parse_qantrssi, parse_qbaseline,
    parse_qeng_neighbourcell, parse_qeng_servingcell, parse_qgdcnt, parse_qicsgp, parse_qnwprefcfg_bands,
    parse_qnwprefcfg_supported, parse_qtemp,
};
use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;

fn cmd_delay() {
    std::thread::sleep(std::time::Duration::from_millis(5));
}

/// UniSoc (展锐) modem adapter for Quectel RG200U/RM500U series
pub struct UniSocModem {
    model: String,
}

impl UniSocModem {
    pub fn new(model: String) -> Self {
        Self { model }
    }
}

impl ModemVendor for UniSocModem {
    fn vendor(&self) -> ChipsetVendor {
        ChipsetVendor::UniSoc
    }

    fn model(&self) -> &str {
        &self.model
    }

    // ==================== Basic Information ====================

    fn query_sim_status(&mut self, transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+CPIN?")?;
        Ok(parse_cpin(&resp))
    }

    fn query_imei(&mut self, transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+CGSN")?;
        Ok(parse_cgsn(&resp))
    }

    fn query_iccid(&mut self, transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = match transport.send_at("AT+CCID") {
            Ok(r) => r,
            Err(_) => return Err("Failed to query CCID".to_string()),
        };
        let ccid = parse_ccid(&resp);
        if !ccid.is_empty() {
            return Ok(ccid);
        }
        cmd_delay();
        match transport.send_at("AT+QCCID") {
            Ok(r) => Ok(parse_ccid(&r)),
            Err(e) => Err(format!("Failed to query ICCID: {}", e)),
        }
    }

    fn query_hardware_info(&mut self, transport: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let cgmm_resp = transport.send_at("AT+CGMM")?;
        let model = parse_cgmm(&cgmm_resp);
        cmd_delay();

        let cgmi_resp = transport.send_at("AT+CGMI")?;
        let manufacturer = parse_cgmi(&cgmi_resp);
        cmd_delay();

        let gmr_resp = match transport.send_at("AT+GMR") {
            Ok(r) => r,
            Err(e) => { log::warn!("AT+GMR failed: {}", e); String::new() }
        };
        let firmware = parse_gmr(&gmr_resp);
        cmd_delay();

        let (ap_baseline, cp_baseline) = match transport.send_at("AT+QBASELINE") {
            Ok(resp) => parse_qbaseline(&resp),
            Err(e) => { log::warn!("AT+QBASELINE failed: {}", e); (String::new(), String::new()) }
        };
        cmd_delay();

        let temp = self.query_temperature(transport)?;

        Ok(HardwareInfo {
            model,
            manufacturer,
            firmware,
            ap_baseline,
            cp_baseline,
            soc_temp: temp.soc_temp,
            pa_temp: temp.pa_temp,
        })
    }

    fn query_temperature(&mut self, transport: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        let (soc_temp, pa_temp) = match transport.send_at("AT+QTEMP") {
            Ok(resp) => parse_qtemp(&resp),
            Err(e) => { log::warn!("AT+QTEMP failed: {}", e); (String::new(), String::new()) }
        };
        Ok(TemperatureInfo { soc_temp, pa_temp })
    }

    // ==================== Network Information ====================

    fn query_serving_cell(&mut self, transport: &mut dyn AtTransport) -> Result<ServingCellInfo, String> {
        let resp = transport.send_at(r#"AT+QENG="servingcell""#)?;
        match parse_qeng_servingcell(&resp) {
            Some(info) => Ok(ServingCellInfo {
                connected: info.connected,
                tech: info.tech,
                operator_mcc: info.operator_mcc,
                operator_mnc: info.operator_mnc,
                cell_id: info.cell_id,
                pci: info.pci,
                arfcn: info.arfcn,
                band: info.band,
                bandwidth: info.bandwidth,
                rsrp: info.rsrp,
                rsrq: info.rsrq,
                sinr: info.sinr,
                tx_power: info.tx_power,
                scs: info.scs,
            }),
            None => Ok(ServingCellInfo::default()),
        }
    }

    fn query_neighbor_cells(&mut self, transport: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let resp = transport.send_at(r#"AT+QENG="neighbourcell""#)?;
        let (lte, nr) = parse_qeng_neighbourcell(&resp);
        Ok(NeighborCells { lte, nr })
    }

    fn query_signal_strength(&mut self, transport: &mut dyn AtTransport) -> Result<SignalInfo, String> {
        let serving = self.query_serving_cell(transport)?;
        let mut signal = SignalInfo {
            rsrp: serving.rsrp.clone(),
            rsrq: serving.rsrq.clone(),
            sinr: serving.sinr.clone(),
            ant_values: Default::default(),
        };
        cmd_delay();

        match transport.send_at("AT+QANTRSSI?") {
            Ok(resp) => {
                let ant = parse_qantrssi(&resp);
                if ant.iter().any(|v| !v.is_empty()) {
                    signal.ant_values = ant;
                }
            }
            Err(e) => log::warn!("AT+QANTRSSI? failed: {}", e),
        }

        Ok(signal)
    }

    fn query_operator(&mut self, transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+COPS?")?;
        let (name, _) = parse_cops(&resp);
        Ok(name)
    }

    fn query_registration_status(&mut self, transport: &mut dyn AtTransport) -> Result<String, String> {
        let serving = self.query_serving_cell(transport)?;
        if serving.connected {
            Ok("已注册".to_string())
        } else {
            Ok("未注册".to_string())
        }
    }

    fn query_connection_status(&mut self, transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+CGACT?")?;
        let contexts = parse_cgact(&resp);
        if contexts.iter().any(|(_, s)| *s == 1) {
            Ok("已连接".to_string())
        } else {
            Ok("未连接".to_string())
        }
    }

    // ==================== APN and Data ====================

    fn query_apn_list(&mut self, transport: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> {
        let cgact_resp = transport.send_at("AT+CGACT?")?;
        let active_cids: Vec<i32> = parse_cgact(&cgact_resp)
            .into_iter()
            .filter(|(_, status)| *status == 1)
            .map(|(cid, _)| cid)
            .collect();
        cmd_delay();

        let resp = transport.send_at("AT+QICSGP?")?;
        Ok(parse_qicsgp(&resp, &active_cids))
    }

    fn set_apn(
        &mut self,
        transport: &mut dyn AtTransport,
        cid: i32,
        context_type: i32,
        apn: &str,
        username: &str,
        password: &str,
        auth_type: i32,
    ) -> Result<(), String> {
        let cmd = format!(
            "AT+QICSGP={},{},\"{}\",\"{}\",\"{}\",{}",
            cid, context_type, apn, username, password, auth_type
        );
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set APN: {}", resp)) }
    }

    fn delete_apn(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let cmd = format!("AT+CGDCONT={}", cid);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to delete APN: {}", resp)) }
    }

    fn connect_data(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let cmd = format!("AT+QNETDEVCTL={},3,1", cid);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to connect: {}", resp)) }
    }

    fn disconnect_data(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let cmd = format!("AT+QNETDEVCTL={},2,0", cid);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to disconnect: {}", resp)) }
    }

    fn query_ip_info(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
        let cmd = format!("AT+QNETDEVSTATUS={}", cid);
        let resp = transport.send_at(&cmd)?;
        let (ipv4, mask, gw, dns, ipv6) = at_parser::parse_qnetdevstatus(&resp);
        Ok(IpInfo {
            ipv4_addr: ipv4,
            ipv4_mask: mask,
            ipv4_gw: gw,
            ipv4_dns: dns,
            ipv6_addr: ipv6,
            ipv6_gw: String::new(),
            ipv6_dns: String::new(),
        })
    }

    // ==================== Band Configuration ====================

    fn query_band_config(&mut self, transport: &mut dyn AtTransport) -> Result<BandConfig, String> {
        let supported_resp = match transport.send_at("AT+QNWPREFCFG=?") {
            Ok(r) => r,
            Err(e) => { log::warn!("AT+QNWPREFCFG=? failed: {}", e); String::new() }
        };
        let (lte_supported, nr_supported) = parse_qnwprefcfg_supported(&supported_resp);
        cmd_delay();

        let lte_resp = transport.send_at(r#"AT+QNWPREFCFG="lte_band""#).unwrap_or_default();
        let lte_locked = parse_qnwprefcfg_bands(&lte_resp, "lte_band");
        cmd_delay();

        let nr_resp = transport.send_at(r#"AT+QNWPREFCFG="nr5g_band""#).unwrap_or_default();
        let nr_locked = parse_qnwprefcfg_bands(&nr_resp, "nr5g_band");

        Ok(BandConfig { lte_supported, nr_supported, lte_locked, nr_locked })
    }

    fn set_lte_bands(&mut self, transport: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        let cmd = format!(r#"AT+QNWPREFCFG="lte_band","{}""#, bands);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set LTE bands: {}", resp)) }
    }

    fn set_nr5g_bands(&mut self, transport: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        let cmd = format!(r#"AT+QNWPREFCFG="nr5g_band","{}""#, bands);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set NR5G bands: {}", resp)) }
    }

    fn set_network_mode(&mut self, transport: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        let cmd = format!(r#"AT+QNWPREFCFG="mode_pref","{}""#, mode);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set network mode: {}", resp)) }
    }

    // ==================== Traffic Statistics ====================

    fn query_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        let resp = transport.send_at("AT+QGDCNT?")?;
        let (ul, dl) = parse_qgdcnt(&resp);
        Ok(TrafficInfo { ul_bytes: ul, dl_bytes: dl })
    }

    fn reset_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<(), String> {
        let resp = transport.send_at("AT+QGDCNT=0")?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to reset traffic: {}", resp)) }
    }

    // ==================== Feature Toggles ====================

    fn query_feature_toggles(&mut self, transport: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
        let pcie_mode = match transport.send_at(r#"AT+QCFG="pcie/mode""#) {
            Ok(r) => at_parser::parse_qcfg_int(&r, "pcie/mode").unwrap_or(0) == 1,
            Err(_) => false,
        };
        cmd_delay();

        let ethernet = match transport.send_at(r#"AT+QCFG="ethernet""#) {
            Ok(r) => at_parser::parse_qcfg_int(&r, "ethernet").unwrap_or(0) == 1,
            Err(_) => false,
        };
        cmd_delay();

        let proxyarp = match transport.send_at(r#"AT+QCFG="proxyarp""#) {
            Ok(r) => at_parser::parse_qcfg_int(&r, "proxyarp").unwrap_or(0) == 1,
            Err(_) => false,
        };
        cmd_delay();

        let uartat = match transport.send_at(r#"AT+QCFG="uartat""#) {
            Ok(r) => at_parser::parse_qcfg_int(&r, "uartat").unwrap_or(0) == 1,
            Err(_) => false,
        };
        cmd_delay();

        let adb = match transport.send_at(r#"AT+QCFG="usbcfg""#) {
            Ok(r) => at_parser::parse_qcfg_usbcfg_adb(&r),
            Err(_) => false,
        };

        Ok(FeatureToggles { pcie_mode, ethernet, proxyarp, uartat, eth_at: false, adb })
    }

    fn set_feature_toggle(
        &mut self,
        transport: &mut dyn AtTransport,
        feature: &str,
        enabled: bool,
    ) -> Result<(), String> {
        let value = if enabled { 1 } else { 0 };
        if feature == "adb" {
            let resp = transport.send_at(r#"AT+QCFG="usbcfg""#)?;
            for line in extract_data_lines(&resp) {
                if let Some(rest) = line.strip_prefix("+QCFG: \"usbcfg\",") {
                    let mut parts: Vec<&str> = rest.split(',').collect();
                    if !parts.is_empty() {
                        let last = parts.len() - 1;
                        parts[last] = if enabled { "1" } else { "0" };
                        let cmd = format!(r#"AT+QCFG="usbcfg",{}"#, parts.join(","));
                        let resp = transport.send_at(&cmd)?;
                        if is_ok(&resp) { return Ok(()); }
                        else { return Err(format!("Failed to set usbcfg: {}", resp)); }
                    }
                }
            }
            return Err("Could not parse current usbcfg".to_string());
        }
        let cmd = format!(r#"AT+QCFG="{}",{}"#, feature, value);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set {}: {}", feature, resp)) }
    }

    // ==================== Power Management ====================

    fn reboot(&mut self, transport: &mut dyn AtTransport) -> Result<(), String> {
        let resp = transport.send_at("AT+CFUN=1,1")?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to reboot: {}", resp)) }
    }

    fn set_cfun(&mut self, transport: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        let cmd = format!("AT+CFUN={}", mode);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set CFUN: {}", resp)) }
    }

    // ==================== QoS ====================

    fn query_qos(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<QosInfo, String> {
        let cmd = format!("AT+C5GQOSRDP={}", cid);
        let resp = transport.send_at(&cmd)?;
        let (cqi, ul_bw, dl_bw) = parse_c5gqosrdp(&resp);
        Ok(QosInfo { cqi, ul_bandwidth: ul_bw, dl_bandwidth: dl_bw })
    }
}
