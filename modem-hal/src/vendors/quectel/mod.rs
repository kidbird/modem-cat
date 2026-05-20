use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;
use parser::*;

pub mod parser;
pub mod qualcomm;
pub mod unisoc;

fn send_and_delay(t: &mut dyn AtTransport, cmd: &str) -> Result<String, String> {
    let result = t.send_at(cmd)?;
    std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(result)
}

fn send_and_check(t: &mut dyn AtTransport, cmd: &str) -> Result<String, String> {
    let resp = send_and_delay(t, cmd)?;
    if is_ok(&resp) {
        Ok(resp)
    } else {
        Err(format!("AT command failed: {} => {}", cmd, resp.trim()))
    }
}



pub enum QuectelChip {
    Qualcomm,
    UniSoc,
}

pub struct QuectelModem {
    pub chip: QuectelChip,
    pub model: String,
}

impl QuectelModem {
    pub fn qualcomm(model: String) -> Self {
        Self {
            chip: QuectelChip::Qualcomm,
            model,
        }
    }
    pub fn unisoc(model: String) -> Self {
        Self {
            chip: QuectelChip::UniSoc,
            model,
        }
    }
}

impl ModemVendor for QuectelModem {
    fn vendor(&self) -> ChipsetVendor {
        match self.chip {
            QuectelChip::Qualcomm => ChipsetVendor::Qualcomm,
            QuectelChip::UniSoc => ChipsetVendor::UniSoc,
        }
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn query_sim_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = send_and_delay(t, "AT+CPIN?")?;
        Ok(parse_cpin(&resp))
    }

    fn query_imei(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = send_and_delay(t, "AT+CGSN")?;
        Ok(parse_cgsn(&resp))
    }

    fn query_iccid(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let cmd = match self.chip {
            QuectelChip::Qualcomm => "AT+ICCID",
            QuectelChip::UniSoc => "AT+CCID",
        };
        let resp = t.send_at(cmd)?;
        let iccid = parse_iccid(&resp);
        if !iccid.is_empty() {
            return Ok(iccid);
        }
        let resp2 = send_and_delay(t, "AT+QCCID")?;
        Ok(parse_iccid(&resp2))
    }

    fn query_operator(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = send_and_delay(t, "AT+COPS?")?;
        Ok(parse_cops_with_act(&resp).0)
    }

    fn query_registration_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = send_and_delay(t, "AT+CEREG?")?;
        let (status, _tac, _ci) = parse_cereg(&resp);
        Ok(status)
    }

    fn query_connection_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = send_and_delay(t, "AT+CGACT?")?;
        let contexts = parse_cgact(&resp);
        if contexts.iter().any(|(_, s)| *s == 1) {
            Ok("\u{5df2}\u{8fde}\u{63a5}".to_string())
        } else {
            Ok("\u{672a}\u{8fde}\u{63a5}".to_string())
        }
    }

    fn query_serving_cell(&mut self, t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> {
        let resp = send_and_delay(t, r#"AT+QENG="servingcell""#)?;
        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        Ok(parse_qeng_serving_cell(&resp, qualcomm_bw))
    }

    fn query_signal_strength(&mut self, t: &mut dyn AtTransport) -> Result<SignalInfo, String> {
        let qeng_resp = send_and_delay(t, r#"AT+QENG="servingcell""#)?;
        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        let cell = parse_qeng_serving_cell(&qeng_resp, qualcomm_bw);

        let mut ant_values = vec![String::new(), String::new(), String::new(), String::new()];
        if let Ok(antrssi_resp) = t.send_at("AT+QANTRSSI?") {
            let ant = parse_qantrssi(&antrssi_resp);
            if ant.iter().any(|v| !v.is_empty()) {
                ant_values = ant;
            }
        }

        Ok(SignalInfo {
            rsrp: cell.rsrp,
            rsrq: cell.rsrq,
            sinr: cell.sinr,
            ant_values,
        })
    }

    fn query_neighbor_cells(&mut self, t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let resp = send_and_delay(t, r#"AT+QENG="neighbourcell""#)?;
        Ok(parse_qeng_neighbour_cells(&resp))
    }

    fn query_hardware_info(&mut self, t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let cgmm_resp = send_and_delay(t, "AT+CGMM")?;
        let model = parse_cgmm(&cgmm_resp);

        let cgmi_resp = send_and_delay(t, "AT+CGMI")?;
        let manufacturer = parse_cgmm(&cgmi_resp);

        let gmr_resp = match t.send_at("AT+GMR") {
            Ok(r) => r,
            Err(_) => String::new(),
        };
        let firmware = parse_gmr(&gmr_resp);

        let (ap_baseline, cp_baseline) = match t.send_at("AT+QBASELINE") {
            Ok(resp) => parse_qbaseline(&resp),
            Err(_) => (String::new(), String::new()),
        };

        let (soc_temp, pa_temp) = match t.send_at("AT+QTEMP") {
            Ok(resp) => parse_qtemp_rich(&resp),
            Err(_) => (String::new(), String::new()),
        };

        Ok(HardwareInfo {
            model,
            manufacturer,
            firmware,
            ap_baseline,
            cp_baseline,
            soc_temp,
            pa_temp,
        })
    }

    fn query_temperature(&mut self, t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        match self.chip {
            QuectelChip::UniSoc | QuectelChip::Qualcomm => {
                let resp = send_and_delay(t, "AT+QTEMP")?;
                Ok(parse_qtemp(&resp))
            }
        }
    }

    fn query_apn_list(&mut self, t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> {
        let cgact_resp = send_and_delay(t, "AT+CGACT?")?;
        let active_cids: Vec<i32> = parse_cgact(&cgact_resp)
            .into_iter()
            .filter(|(_, status)| *status == 1)
            .map(|(cid, _)| cid)
            .collect();

        let qicsgp_resp = send_and_delay(t, "AT+QICSGP?")?;
        let entries = parse_qicsgp(&qicsgp_resp, &active_cids);
        if !entries.is_empty() {
            return Ok(entries.into_iter().filter(|e| e.cid >= 1 && e.cid <= 8).collect());
        }

        let cgdcont_resp = send_and_delay(t, "AT+CGDCONT?")?;
        let cgact2 = send_and_delay(t, "AT+CGACT?")?;
        let active_set = parse_cgact_cids(&cgact2);
        Ok(parse_cgdcont_apn(&cgdcont_resp, &active_set)
            .into_iter()
            .filter(|e| e.cid >= 1 && e.cid <= 8)
            .collect())
    }

    fn query_ip_info(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::query_ip_info(t, cid),
            QuectelChip::UniSoc => unisoc::query_ip_info(t, cid),
        }
    }

    fn query_band_config(&mut self, t: &mut dyn AtTransport) -> Result<BandConfig, String> {
        let lte_resp = send_and_delay(t, r#"AT+QNWPREFCFG="lte_band""#)?;
        let nr_resp = send_and_delay(t, r#"AT+QNWPREFCFG="nr5g_band""#)?;
        let (lte_spec, nr_spec) = crate::types::spec_bands_for_model(&self.model);
        Ok(BandConfig {
            lte_supported: vec![],
            nr_supported: vec![],
            lte_locked: parse_qnwprefcfg_bands(&lte_resp, "lte_band"),
            nr_locked: parse_qnwprefcfg_bands(&nr_resp, "nr5g_band"),
            lte_spec,
            nr_spec,
        })
    }

    fn set_lte_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"lte_band\",{}", bands))?;
        Ok(())
    }

    fn set_nr5g_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"nr5g_band\",{}", bands))?;
        Ok(())
    }

    fn set_network_mode(&mut self, t: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        send_and_check(t, &format!(r#"AT+QNWPREFCFG="mode_pref",{}"#, mode))?;
        Ok(())
    }

    fn set_bands(
        &mut self,
        t: &mut dyn AtTransport,
        lte: &str,
        nr: &str,
    ) -> Result<(), String> {
        if !lte.is_empty() {
            send_and_check(t, &format!(r#"AT+QNWPREFCFG="lte_band",{}"#, lte))?;
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
        if !nr.is_empty() {
            send_and_check(t, &format!(r#"AT+QNWPREFCFG="nr5g_band",{}"#, nr))?;
        }
        Ok(())
    }

    fn query_bands_with_spec(
        &mut self,
        t: &mut dyn AtTransport,
    ) -> Result<BandConfig, String> {
        let supported_resp = match t.send_at("AT+QNWPREFCFG=?") {
            Ok(r) => r,
            Err(e) => {
                log::warn!("AT+QNWPREFCFG=? failed: {}", e);
                String::new()
            }
        };
        let (lte_supported, nr_supported) = parse_qnwprefcfg_supported(&supported_resp);

        let lte_resp = send_and_delay(t, r#"AT+QNWPREFCFG="lte_band""#)?;
        let nr_resp = send_and_delay(t, r#"AT+QNWPREFCFG="nr5g_band""#)?;
        let (lte_spec, nr_spec) = crate::types::spec_bands_for_model(&self.model);

        Ok(BandConfig {
            lte_supported,
            nr_supported,
            lte_locked: parse_qnwprefcfg_bands(&lte_resp, "lte_band"),
            nr_locked: parse_qnwprefcfg_bands(&nr_resp, "nr5g_band"),
            lte_spec,
            nr_spec,
        })
    }

    fn reset_all_bands(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        let resp = send_and_delay(t, r#"AT+QNWPREFCFG="all_band_reset""#)?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to reset bands: {}", resp))
        }
    }

    fn query_network_mode(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = send_and_delay(t, "AT+QNWPREFCFG=\"mode_pref\"")?;
        Ok(parse_qnwprefcfg_mode(&resp))
    }

    fn query_traffic(&mut self, t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::query_traffic(t),
            QuectelChip::UniSoc => unisoc::query_traffic(t),
        }
    }

    fn reset_traffic(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => {
                t.send_at("AT+QGDNRCNT=0")?;
            }
            QuectelChip::UniSoc => {
                t.send_at("AT+QGDCNT=0")?;
            }
        }
        Ok(())
    }

    fn reboot(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        let resp = t.send_at("AT+CFUN=1,1")?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to reboot: {}", resp))
        }
    }

    fn set_cfun(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        let resp = send_and_check(t, &format!("AT+CFUN={}", mode))?;
        if !is_ok(&resp) {
            return Err(format!("Failed to set CFUN: {}", resp));
        }
        Ok(())
    }

    fn query_feature_toggles(
        &mut self,
        t: &mut dyn AtTransport,
    ) -> Result<FeatureToggles, String> {
        let pcie_mode = match send_and_delay(t, r#"AT+QCFG="pcie/mode""#) {
            Ok(r) => parse_qcfg_int(&r, "pcie/mode").unwrap_or(0) == 1,
            Err(_) => false,
        };
        let ethernet = match send_and_delay(t, r#"AT+QCFG="ethernet""#) {
            Ok(r) => parse_qcfg_int(&r, "ethernet").unwrap_or(0) == 1,
            Err(_) => false,
        };
        let proxyarp = match send_and_delay(t, r#"AT+QCFG="proxyarp""#) {
            Ok(r) => parse_qcfg_int(&r, "proxyarp").unwrap_or(0) == 1,
            Err(_) => false,
        };
        let uart_at = match send_and_delay(t, r#"AT+QCFG="uartat""#) {
            Ok(r) => parse_qcfg_int(&r, "uartat").unwrap_or(0) == 1,
            Err(_) => false,
        };
        let eth_at = match send_and_delay(t, r#"AT+QCFG="eth_at""#) {
            Ok(r) => parse_qcfg_int(&r, "eth_at").unwrap_or(0) == 1,
            Err(_) => false,
        };
        let adb = match t.send_at(r#"AT+QCFG="usbcfg""#) {
            Ok(r) => parse_qcfg_usbcfg_adb(&r),
            Err(_) => false,
        };
        let napt = match send_and_delay(t, r#"AT+QCFG="napt""#) {
            Ok(r) => parse_qcfg_int(&r, "napt").unwrap_or(0) == 1,
            Err(_) => false,
        };
        let netmask = match send_and_delay(t, r#"AT+QCFG="netmask""#) {
            Ok(r) => parse_qcfg_int(&r, "netmask").unwrap_or(0) == 1,
            Err(_) => false,
        };

        Ok(FeatureToggles {
            pcie_mode,
            ethernet,
            proxyarp,
            uart_at,
            eth_at,
            adb,
            napt,
            netmask,
        })
    }

    fn set_feature_toggle(
        &mut self,
        t: &mut dyn AtTransport,
        feat: &str,
        on: bool,
    ) -> Result<(), String> {
        let val = if on { 1 } else { 0 };
        match feat {
            "pcieMode" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="pcie/mode",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set pcie/mode: {}", resp));
                }
            }
            "ethernet" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="ethernet",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set ethernet: {}", resp));
                }
            }
            "proxyArp" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="proxyarp",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set proxyarp: {}", resp));
                }
            }
            "uartAt" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="uartat",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set uartat: {}", resp));
                }
            }
            "ethAt" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="eth_at",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set eth_at: {}", resp));
                }
            }
            "adb" => {
                let resp = t.send_at(r#"AT+QCFG="usbcfg""#)?;
                for line in extract_data_lines(&resp) {
                    if let Some(rest) = line.strip_prefix("+QCFG: \"usbcfg\",") {
                        let mut parts: Vec<&str> = rest.split(',').collect();
                        if parts.len() >= 2 {
                            // ADB flag is the second-to-last parameter
                            let adb_idx = parts.len() - 2;
                            parts[adb_idx] = if on { "1" } else { "0" };
                            let cmd = format!(r#"AT+QCFG="usbcfg",{}"#, parts.join(","));
                            let resp2 = t.send_at(&cmd)?;
                            if !is_ok(&resp2) {
                                return Err(format!("Failed to set usbcfg: {}", resp2));
                            }
                            return Ok(());
                        }
                    }
                }
                return Err("Could not parse current usbcfg".to_string());
            }
            "napt" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="napt",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set napt: {}", resp));
                }
            }
            "netmask" => {
                let resp = send_and_delay(t, &format!(r#"AT+QCFG="netmask",{}"#, val))?;
                if !is_ok(&resp) {
                    return Err(format!("Failed to set netmask: {}", resp));
                }
            }
            _ => return Err(format!("Unknown feature: {}", feat)),
        }
        Ok(())
    }

    fn query_qos(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<QosInfo, String> {
        let cmd = format!("AT+C5GQOSRDP={}", cid);
        let resp = send_and_delay(t, &cmd)?;
        let (cqi, ul_bw, dl_bw) = parse_c5gqosrdp(&resp);
        Ok(QosInfo {
            cqi,
            ul_bandwidth: ul_bw,
            dl_bandwidth: dl_bw,
        })
    }

    fn query_baseline(&mut self, t: &mut dyn AtTransport) -> Result<BaselineInfo, String> {
        let resp = send_and_delay(t, "AT+QBASELINE")?;
        let (ap, cp) = parse_qbaseline(&resp);
        Ok(BaselineInfo {
            ap_baseline: ap,
            cp_baseline: cp,
        })
    }

    fn query_ant_rssi(&mut self, t: &mut dyn AtTransport) -> Result<Vec<String>, String> {
        let resp = send_and_delay(t, "AT+QANTRSSI?")?;
        Ok(parse_qantrssi(&resp))
    }

    fn factory_reset(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        let resp = t.send_at("AT+QFACT=0")?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to factory reset: {}", resp))
        }
    }

    fn query_sim_slot(&mut self, t: &mut dyn AtTransport) -> Result<i32, String> {
        let resp = send_and_delay(t, "AT+QUIMSLOT?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+QUIMSLOT:") {
                if let Ok(slot) = rest.trim().parse::<i32>() {
                    return Ok(slot);
                }
            }
        }
        Ok(1)
    }

    fn switch_sim_slot(&mut self, t: &mut dyn AtTransport, slot: i32) -> Result<(), String> {
        let resp = send_and_delay(t, &format!("AT+QUIMSLOT={}", slot))?;
        if is_ok(&resp) {
            std::thread::sleep(std::time::Duration::from_millis(500));
            return Ok(());
        }
        if resp.contains("+CME ERROR") || resp.contains("ERROR") {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            let resp2 = send_and_delay(t, &format!("AT+QUIMSLOT={}", slot))?;
            if is_ok(&resp2) {
                return Ok(());
            }
            return Err(format!("Failed to switch SIM slot: {}", resp2));
        }
        Err(format!("Failed to switch SIM slot: {}", resp))
    }

    fn query_usbnet_mode(&mut self, t: &mut dyn AtTransport) -> Result<i32, String> {
        let resp = send_and_delay(t, r#"AT+QCFG="usbnet""#)?;
        parse_qcfg_usbnet(&resp).ok_or_else(|| format!("Failed to parse usbnet: {}", resp))
    }

    fn set_usbnet_mode(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        let cmd = format!(r#"AT+QCFG="usbnet",{}"#, mode);
        let resp = send_and_delay(t, &cmd)?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to set usbnet: {}", resp))
        }
    }

    fn query_modem_status(
        &mut self,
        t: &mut dyn AtTransport,
    ) -> Result<ModemStatus, String> {
        let sim_status = self.query_sim_status(t)?;
        let imei = self.query_imei(t)?;
        let iccid = self.query_iccid(t).unwrap_or_default();

        let qeng_resp = send_and_delay(t, r#"AT+QENG="servingcell""#)?;
        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        let serving_cell = parse_qeng_serving_cell(&qeng_resp, qualcomm_bw);

        let cops_resp = send_and_delay(t, "AT+COPS?")?;
        let (operator, _) = parse_cops_with_act(&cops_resp);

        let ant_values = match t.send_at("AT+QANTRSSI?") {
            Ok(r) => {
                let ant = parse_qantrssi(&r);
                if ant.iter().any(|v| !v.is_empty()) { ant } else { vec![String::new(), String::new(), String::new(), String::new()] }
            }
            Err(_) => vec![String::new(), String::new(), String::new(), String::new()],
        };

        let conn_status = self.query_connection_status(t).unwrap_or_else(|_| "\u{672a}\u{8fde}\u{63a5}".to_string());

        let reg_status = match serving_cell.mobility_state.as_str() {
            "CONNECT" => "\u{5df2}\u{6ce8}\u{518c}".to_string(),
            "LIMSRV" => "\u{9650}\u{5236}\u{670d}\u{52a1}".to_string(),
            "SEARCH" => "\u{641c}\u{7d22}\u{4e2d}".to_string(),
            "IDLE" => "\u{7a7a}\u{95f2}".to_string(),
            other => other.to_string(),
        };

        Ok(ModemStatus {
            sim_status,
            reg_status,
            conn_status,
            imei,
            iccid,
            operator,
            network_type: serving_cell.tech,
            pci: serving_cell.pci,
            cell_id: serving_cell.cell_id,
            arfcn: serving_cell.arfcn,
            bandwidth: serving_cell.bandwidth,
            rsrp: serving_cell.rsrp,
            rsrq: serving_cell.rsrq,
            sinr: serving_cell.sinr,
            tx_power: serving_cell.tx_power,
            rx_level: serving_cell.rx_level,
            ant_values,
            scs: serving_cell.scs,
        })
    }

    fn set_apn(
        &mut self,
        t: &mut dyn AtTransport,
        cid: i32,
        ctx: i32,
        apn: &str,
        user: &str,
        pass: &str,
        auth: i32,
    ) -> Result<(), String> {
        send_and_check(t, &format!(
            "AT+QICSGP={},{},\"{}\",\"{}\",\"{}\",{}",
            cid, ctx, apn, user, pass, auth
        ))?;
        Ok(())
    }

    fn delete_apn(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let resp = send_and_delay(t, &format!("AT+CGDCONT={}", cid))?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to delete APN: {}", resp))
        }
    }

    fn set_apn_active(&mut self, t: &mut dyn AtTransport, cid: i32, active: bool) -> Result<(), String> {
        let state = if active { 1 } else { 0 };
        let resp = send_and_delay(t, &format!("AT+CGACT={},{}", state, cid))?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to {} APN: {}", if active { "activate" } else { "deactivate" }, resp))
        }
    }

    fn query_5glan(&mut self, t: &mut dyn AtTransport) -> Result<Vec<L5GanEntry>, String> {
        let resp = send_and_delay(t, r#"AT+QCFG="5glan""#)?;
        Ok(parse_5glan(&resp))
    }

    fn set_5glan(&mut self, t: &mut dyn AtTransport, cid: i32, enabled: bool) -> Result<(), String> {
        let state = if enabled { 1 } else { 0 };
        let resp = send_and_delay(t, &format!(r#"AT+QCFG="5glan",{},{},1"#, cid, state))?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to set 5GLAN: {}", resp))
        }
    }

    fn connect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::connect_data(t, cid),
            QuectelChip::UniSoc => unisoc::connect_data(t, cid),
        }
    }

    fn disconnect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::disconnect_data(t, cid),
            QuectelChip::UniSoc => unisoc::disconnect_data(t, cid),
        }
    }

    fn send_raw_at(
        &mut self,
        transport: &mut dyn AtTransport,
        command: &str,
    ) -> Result<String, String> {
        transport.send_at(command)
    }
}