use crate::at_parser::{
    self, extract_data_lines, is_ok, parse_cgact, parse_cgsn, parse_cgmi,
    parse_cgmm, parse_cops, parse_ccid, parse_cpin, parse_gmr,
    parse_qeng_neighbourcell, parse_qeng_servingcell,
    parse_qnwprefcfg_bands, parse_qnwprefcfg_supported,
};
use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;

fn cmd_delay() {
    std::thread::sleep(std::time::Duration::from_millis(5));
}

/// Qualcomm modem adapter for Quectel RG520N/RM520N/RG525F series
///
/// Key differences from UniSoc:
/// - APN: uses standard AT+CGDCONT instead of AT+QICSGP
/// - ICCID: uses AT+ICCID instead of AT+QCCID
/// - Data connect: uses AT+QMAP="connect" instead of AT+QNETDEVCTL
/// - IP info: uses AT+QMAP="WWAN" instead of AT+QNETDEVSTATUS
/// - Traffic: uses AT+QGDNRCNT instead of AT+QGDCNT
/// - Signal: has dedicated AT+QRSRP/AT+QRSRQ/AT+QSINR commands
/// - Temperature: no AT+QTEMP (uses AT+QCFG="device_thermal" or N/A)
/// - Baseline: no AT+QBASELINE
/// - Band: has extra AT+QNWPREFCFG="nsa_nr5g_band" for NSA bands
/// - Band: has extra AT+QNWPREFCFG="rat_acq_order" for RAT priority
pub struct QualcommModem {
    model: String,
}

impl QualcommModem {
    pub fn new(model: String) -> Self {
        Self { model }
    }

    /// Parse AT+QGDNRCNT? response
    /// Format: +QGDNRCNT: <bytes_sent>,<bytes_recv>
    fn parse_qgdnrcnt(response: &str) -> (u64, u64) {
        for line in extract_data_lines(response) {
            if let Some(rest) = line.strip_prefix("+QGDNRCNT:") {
                let parts: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
                if parts.len() >= 2 {
                    let ul = parts[0].parse::<u64>().unwrap_or(0);
                    let dl = parts[1].parse::<u64>().unwrap_or(0);
                    return (ul, dl);
                }
            }
        }
        (0, 0)
    }

    /// Parse AT+QMAP="WWAN" response
    /// Format: +QMAP: "WWAN",<status>,<profileID>,<IP_family>,<IP_address>
    fn parse_qmap_wwan(response: &str) -> IpInfo {
        let mut info = IpInfo {
            ipv4_addr: String::new(),
            ipv4_mask: String::new(),
            ipv4_gw: String::new(),
            ipv4_dns: String::new(),
            ipv6_addr: String::new(),
            ipv6_gw: String::new(),
            ipv6_dns: String::new(),
        };

        for line in extract_data_lines(response) {
            if let Some(rest) = line.strip_prefix("+QMAP: \"WWAN\",") {
                let parts: Vec<&str> = rest.split(',').map(|v| v.trim().trim_matches('"')).collect();
                if parts.len() >= 5 {
                    let ip_family = parts[3];
                    let ip_addr = parts[4];
                    if ip_addr == "0.0.0.0" || ip_addr == "0:0:0:0:0:0:0:0" {
                        continue;
                    }
                    if ip_family == "IPV4" {
                        info.ipv4_addr = ip_addr.to_string();
                    } else if ip_family == "IPV6" {
                        info.ipv6_addr = ip_addr.to_string();
                    }
                }
            }
        }

        info
    }

    /// Parse AT+CGDCONT? response for APN list
    /// Format: +CGDCONT: <cid>,<PDN_type>,<APN>,<IP_addr>,...
    fn parse_cgdcont_apn(response: &str, active_cids: &[i32]) -> Vec<ApnEntry> {
        let mut entries = Vec::new();
        for line in extract_data_lines(response) {
            if let Some(rest) = line.strip_prefix("+CGDCONT:") {
                let parts: Vec<&str> = rest.split(',').map(|v| v.trim().trim_matches('"')).collect();
                if parts.len() >= 3 {
                    let cid = parts[0].parse::<i32>().unwrap_or(0);
                    let ip_type = match parts.get(1).map(|v| v.trim()) {
                        Some("IP") => "IPv4",
                        Some("IPV6") => "IPv6",
                        Some("IPV4V6") => "IPv4v6",
                        _ => "IPv4",
                    };
                    let apn_name = parts.get(2).map(|v| v.to_string()).unwrap_or_default();
                    let active = active_cids.contains(&cid);
                    entries.push(ApnEntry {
                        cid,
                        apn_name,
                        ip_type: ip_type.to_string(),
                        auth_type: 0,
                        username: String::new(),
                        active,
                    });
                }
            }
        }
        entries
    }
}

impl ModemVendor for QualcommModem {
    fn vendor(&self) -> ChipsetVendor {
        ChipsetVendor::Qualcomm
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
        let resp = match transport.send_at("AT+ICCID") {
            Ok(r) => r,
            Err(_) => {
                cmd_delay();
                let r = transport.send_at("AT+CCID")?;
                let ccid = parse_ccid(&r);
                if !ccid.is_empty() {
                    return Ok(ccid);
                }
                return Err("Failed to query ICCID".to_string());
            }
        };
        let ccid = parse_ccid(&resp);
        if !ccid.is_empty() {
            Ok(ccid)
        } else {
            Err("Failed to parse ICCID".to_string())
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

        Ok(HardwareInfo {
            model,
            manufacturer,
            firmware,
            ap_baseline: String::new(),
            cp_baseline: String::new(),
            soc_temp: String::new(),
            pa_temp: String::new(),
        })
    }

    fn query_temperature(&mut self, _transport: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        Ok(TemperatureInfo::default())
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
        let signal = SignalInfo {
            rsrp: serving.rsrp.clone(),
            rsrq: serving.rsrq.clone(),
            sinr: serving.sinr.clone(),
            ant_values: Default::default(),
        };
        cmd_delay();

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

        let resp = transport.send_at("AT+CGDCONT?")?;
        Ok(Self::parse_cgdcont_apn(&resp, &active_cids))
    }

    fn set_apn(
        &mut self,
        transport: &mut dyn AtTransport,
        cid: i32,
        context_type: i32,
        apn: &str,
        _username: &str,
        _password: &str,
        _auth_type: i32,
    ) -> Result<(), String> {
        let pdn_type = match context_type {
            1 => "IP",
            2 => "IPV6",
            3 => "IPV4V6",
            _ => "IP",
        };
        let cmd = format!("AT+CGDCONT={},\"{}\",\"{}\"", cid, pdn_type, apn);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set APN: {}", resp)) }
    }

    fn delete_apn(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let cmd = format!("AT+CGDCONT={}", cid);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to delete APN: {}", resp)) }
    }

    fn connect_data(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let rule_num = cid.max(0).min(3);
        let cmd = format!("AT+QMAP=\"connect\",{},1", rule_num);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to connect: {}", resp)) }
    }

    fn disconnect_data(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        let rule_num = cid.max(0).min(3);
        let cmd = format!("AT+QMAP=\"connect\",{},0", rule_num);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to disconnect: {}", resp)) }
    }

    fn query_ip_info(&mut self, transport: &mut dyn AtTransport, _cid: i32) -> Result<IpInfo, String> {
        let resp = transport.send_at(r#"AT+QMAP="WWAN""#)?;
        Ok(Self::parse_qmap_wwan(&resp))
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

    fn set_nsa_nr5g_bands(&mut self, transport: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        let cmd = format!(r#"AT+QNWPREFCFG="nsa_nr5g_band","{}""#, bands);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set NSA NR5G bands: {}", resp)) }
    }

    fn set_network_mode(&mut self, transport: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        let cmd = format!(r#"AT+QNWPREFCFG="mode_pref","{}""#, mode);
        let resp = transport.send_at(&cmd)?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to set network mode: {}", resp)) }
    }

    // ==================== Traffic Statistics ====================

    fn query_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        let resp = transport.send_at("AT+QGDNRCNT?")?;
        let (ul, dl) = Self::parse_qgdnrcnt(&resp);
        Ok(TrafficInfo { ul_bytes: ul, dl_bytes: dl })
    }

    fn reset_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<(), String> {
        let resp = transport.send_at("AT+QGDNRCNT=0")?;
        if is_ok(&resp) { Ok(()) } else { Err(format!("Failed to reset traffic: {}", resp)) }
    }

    // ==================== Feature Toggles ====================

    fn query_feature_toggles(&mut self, transport: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
        let pcie_mode = match transport.send_at(r#"AT+QCFG="pcie/mode""#) {
            Ok(r) => at_parser::parse_qcfg_int(&r, "pcie/mode").unwrap_or(0) == 1,
            Err(_) => false,
        };
        cmd_delay();

        let _usbnet = match transport.send_at(r#"AT+QCFG="usbnet""#) {
            Ok(r) => at_parser::parse_qcfg_int(&r, "usbnet").unwrap_or(0),
            Err(_) => 0,
        };

        Ok(FeatureToggles {
            pcie_mode,
            ethernet: false,
            proxyarp: false,
            uartat: false,
            eth_at: false,
            adb: false,
        })
    }

    fn set_feature_toggle(
        &mut self,
        transport: &mut dyn AtTransport,
        feature: &str,
        enabled: bool,
    ) -> Result<(), String> {
        let value = if enabled { 1 } else { 0 };
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
        let (cqi, ul_bw, dl_bw) = at_parser::parse_c5gqosrdp(&resp);
        Ok(QosInfo { cqi, ul_bandwidth: ul_bw, dl_bandwidth: dl_bw })
    }
}
