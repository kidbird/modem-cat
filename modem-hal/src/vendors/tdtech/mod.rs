pub mod dial;
pub mod parser;

use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;
use parser::*;

pub struct TdTechModem {
    model: String,
}

impl TdTechModem {
    pub fn new(model: String) -> Self {
        Self { model }
    }
}

fn send_and_check(t: &mut dyn AtTransport, cmd: &str) -> Result<String, String> {
    let resp = t.send_at(cmd)?;
    if resp.lines().any(|l| l.trim() == "OK") && !resp.contains("ERROR") {
        Ok(resp)
    } else {
        Err(format!("Command failed: {}", resp))
    }
}

fn cmd_delay() {
    std::thread::sleep(std::time::Duration::from_millis(5));
}

impl ModemVendor for TdTechModem {
    fn vendor(&self) -> ChipsetVendor {
        ChipsetVendor::TdTech
    }
    fn model(&self) -> &str {
        &self.model
    }

    fn query_sim_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^CARDMODE")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("^CARDMODE:") {
                return Ok(match rest.trim() {
                    "0" => "NO SIM".to_string(),
                    "1" => "SIM".to_string(),
                    "2" => "USIM".to_string(),
                    v => v.to_string(),
                });
            }
        }
        Ok("UNKNOWN".to_string())
    }

    fn query_imei(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CGSN")?;
        for line in resp.lines() {
            let ln = line.trim();
            if ln.chars().all(|c| c.is_ascii_digit()) && ln.len() >= 14 {
                return Ok(ln.to_string());
            }
        }
        Ok(String::new())
    }

    fn query_iccid(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^ICCID?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("^ICCID:") {
                return Ok(rest.trim().to_string());
            }
        }
        Ok(String::new())
    }

    fn query_hardware_info(&mut self, t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let model_resp = t.send_at("AT+CGMM")?;
        cmd_delay();
        let mfr_resp = t.send_at("AT+CGMI")?;
        cmd_delay();
        let fw_resp = t.send_at("AT+GMR")?;
        let extract = |r: &str| -> String {
            for line in r.lines() {
                let ln = line.trim();
                if ln.is_empty() || ln == "OK" || ln.starts_with("AT") || ln.starts_with('+') {
                    continue;
                }
                return ln.to_string();
            }
            String::new()
        };
        Ok(HardwareInfo {
            model: extract(&model_resp),
            manufacturer: extract(&mfr_resp),
            firmware: extract(&fw_resp),
            ap_baseline: String::new(),
            cp_baseline: String::new(),
            soc_temp: String::new(),
            pa_temp: String::new(),
        })
    }

    fn query_temperature(&mut self, _t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        Ok(TemperatureInfo::default())
    }

    fn query_serving_cell(&mut self, t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> {
        let resp = t.send_at("AT^MONSC")?;
        Ok(parse_monsc(&resp))
    }

    fn query_signal_strength(&mut self, t: &mut dyn AtTransport) -> Result<SignalInfo, String> {
        let resp = t.send_at("AT^HCSQ?")?;
        Ok(parse_hcsq(&resp))
    }

    fn query_neighbor_cells(&mut self, _t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        Ok(NeighborCells {
            lte: vec![],
            nr: vec![],
        })
    }

    fn query_operator(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+COPS?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+COPS:") {
                let parts: Vec<&str> = rest.split(',').collect();
                if let Some(name) = parts.get(2) {
                    return Ok(name.trim().trim_matches('"').to_string());
                }
            }
        }
        Ok(String::new())
    }

    fn query_registration_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CEREG?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+CEREG:") {
                let parts: Vec<&str> = rest.trim().split(',').collect();
                let stat = parts.get(1).unwrap_or(parts.first().unwrap_or(&"0")).trim();
                let mapped = match stat {
                    "0" => "NOCONN",
                    "1" => "CONNECT",
                    "2" => "SEARCH",
                    "3" => "DENIED",
                    "4" => "UNKNOWN",
                    "5" => "CONNECT",
                    other => other,
                };
                return Ok(mapped.to_string());
            }
        }
        Ok("NOCONN".to_string())
    }

    fn query_connection_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^DCONNSTAT?")?;
        Ok(if parse_dconnstat(&resp) {
            "\u{5df2}\u{8fde}\u{63a5}".to_string()
        } else {
            "\u{672a}\u{8fde}\u{63a5}".to_string()
        })
    }

    fn query_apn_list(&mut self, t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> {
        let resp = t.send_at("AT+CGDCONT?")?;

        let mut active_cids = std::collections::HashSet::new();
        if let Ok(dconn_resp) = t.send_at("AT^DCONNSTAT?") {
            for line in dconn_resp.lines() {
                if let Some(rest) = line.trim().strip_prefix("^DCONNSTAT:") {
                    let parts: Vec<&str> = rest.split(',').collect();
                    if parts.len() >= 3 {
                        if let Ok(cid) = parts[0].trim().parse::<i32>() {
                            let ipv4_stat = parts[2].trim();
                            let ipv6_stat = parts.get(3).map(|s| s.trim()).unwrap_or("0");
                            if ipv4_stat == "1" || ipv6_stat == "1" {
                                active_cids.insert(cid);
                            }
                        }
                    }
                }
            }
        }

        // Query CGAUTH for authentication info
        let cgauth_resp = t.send_at("AT+CGAUTH?").unwrap_or_default();
        let mut auth_map = std::collections::HashMap::new();
        for line in cgauth_resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+CGAUTH:") {
                let parts: Vec<&str> = rest
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .collect();
                if parts.len() >= 3 {
                    if let Ok(cid) = parts[0].parse::<i32>() {
                        let auth_proto = parts[1].parse::<i32>().unwrap_or(0);
                        let user = parts[2].to_string();
                        auth_map.insert(cid, (auth_proto, user));
                    }
                }
            }
        }

        let mut entries = vec![];
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+CGDCONT:") {
                let parts: Vec<&str> = rest
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .collect();
                if parts.len() >= 3 {
                    if let Ok(cid) = parts[0].parse::<i32>() {
                        let (auth_type, username) =
                            auth_map.get(&cid).cloned().unwrap_or((0, String::new()));
                        let active = active_cids.contains(&cid);
                        entries.push(ApnEntry {
                            cid,
                            apn_name: parts.get(2).unwrap_or(&"").to_string(),
                            ip_type: parts.get(1).unwrap_or(&"IP").to_string(),
                            auth_type,
                            username,
                            active,
                        });
                    }
                }
            }
        }
        Ok(entries)
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
        let pdp = match ctx {
            2 => "IPV6",
            3 => "IPV4V6",
            _ => "IP",
        };
        send_and_check(t, &format!("AT+CGDCONT={},\"{}\",\"{}\"", cid, pdp, apn))?;
        if !user.is_empty() || !pass.is_empty() || auth > 0 {
            let _ = t.send_at(&format!(
                "AT+CGAUTH={},{},\"{}\",\"{}\"",
                cid, auth, user, pass
            ));
        }
        Ok(())
    }

    fn delete_apn(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        send_and_check(t, &format!("AT+CGDCONT={}", cid))?;
        Ok(())
    }

    fn set_apn_active(
        &mut self,
        t: &mut dyn AtTransport,
        cid: i32,
        active: bool,
    ) -> Result<(), String> {
        let state = if active { 1 } else { 0 };
        let resp = t.send_at(&format!("AT+CGACT={},{}", state, cid))?;
        if resp.contains("OK") {
            Ok(())
        } else {
            Err(format!(
                "Failed to {} APN: {}",
                if active { "activate" } else { "deactivate" },
                resp
            ))
        }
    }

    fn connect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        dial::connect(t, cid, "", "", "", 0)
    }

    fn disconnect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        dial::disconnect(t, cid)
    }

    fn query_ip_info(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
        dial::query_ip(t, cid)
    }

    fn query_band_config(&mut self, t: &mut dyn AtTransport) -> Result<BandConfig, String> {
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (_acqorder, lteband_hex) = parse_syscfgex(&resp);
        let lte_bands = decode_syscfgex_lteband(&lteband_hex);
        Ok(BandConfig {
            lte_locked: lte_bands,
            nr_locked: vec![],
            lte_spec: vec![],
            nr_spec: vec![],
        })
    }

    fn set_lte_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (acqorder, _) = parse_syscfgex(&resp);
        let resp2 = t.send_at(&format!(
            "AT^SYSCFGEX=\"{}\",3FFFFFFF,1,2,{},,",
            acqorder, bands
        ))?;
        if resp2.contains("ERROR") {
            return Err(format!("Failed to set LTE bands: {}", resp2.trim()));
        }
        Ok(())
    }

    fn set_nr5g_bands(&mut self, _t: &mut dyn AtTransport, _bands: &str) -> Result<(), String> {
        Err("NR band configuration not supported on MT5700 via this interface".to_string())
    }

    fn set_network_mode(&mut self, t: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        let acqorder = match mode {
            "AUTO" => "030802",
            "LTE" => "03",
            "NR5G" | "NR5G-SA" | "NR" => "08",
            "NR5G-NSA" | "LTE:NR5G" | "LTE:NR" => "0308",
            "WCDMA" => "02",
            other => other,
        };
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (_, lteband) = parse_syscfgex(&resp);
        let resp2 = t.send_at(&format!(
            "AT^SYSCFGEX=\"{}\",3FFFFFFF,1,2,{},,",
            acqorder, lteband
        ))?;
        if resp2.contains("ERROR") {
            return Err(format!("Failed to set network mode: {}", resp2.trim()));
        }
        Ok(())
    }

    fn query_network_mode(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (acqorder, _) = parse_syscfgex(&resp);
        let mode = match acqorder.as_str() {
            "03" => "LTE",
            "08" => "NR5G-SA",
            "0308" => "NR5G-NSA",
            "02" => "WCDMA",
            "030802" => "AUTO",
            s if s.contains("08") && s.contains("03") && s.contains("02") => "AUTO",
            s if s.contains("08") && s.contains("03") => "NR5G-NSA",
            s if s.contains("08") && s.contains("02") => "NR5G-SA",
            s if s.contains("03") && s.contains("02") => "LTE",
            s => s,
        };
        Ok(mode.to_string())
    }

    fn query_traffic(&mut self, t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        let resp = t.send_at("AT^DSFLOWQRY")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("^DSFLOWQRY:") {
                // ^DSFLOWQRY: last_time,last_tx,last_rx,total_time,total_tx,total_rx
                let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim()).collect();
                let tx = u64::from_str_radix(
                    parts.get(4).unwrap_or(&"0x0").trim_start_matches("0x"),
                    16,
                )
                .unwrap_or(0);
                let rx = u64::from_str_radix(
                    parts.get(5).unwrap_or(&"0x0").trim_start_matches("0x"),
                    16,
                )
                .unwrap_or(0);
                return Ok(TrafficInfo {
                    ul_bytes: tx,
                    dl_bytes: rx,
                });
            }
        }
        Ok(TrafficInfo {
            ul_bytes: 0,
            dl_bytes: 0,
        })
    }

    fn reset_traffic(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT^DSFLOWCLR")?;
        Ok(())
    }

    fn reboot(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT^RESET")?;
        Ok(())
    }

    fn set_cfun(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        let resp = t.send_at(&format!("AT+CFUN={}", mode))?;
        if resp.contains("ERROR") {
            return Err(format!("AT+CFUN={} failed: {}", mode, resp.trim()));
        }
        Ok(())
    }

    fn query_feature_toggles(
        &mut self,
        _t: &mut dyn AtTransport,
    ) -> Result<FeatureToggles, String> {
        Ok(FeatureToggles::default())
    }

    fn set_feature_toggle(
        &mut self,
        _t: &mut dyn AtTransport,
        _feat: &str,
        _on: bool,
    ) -> Result<(), String> {
        Ok(())
    }

    fn query_nat_mode(&mut self, _t: &mut dyn AtTransport) -> Result<i32, String> {
        Ok(0)
    }

    fn set_nat_mode(&mut self, _t: &mut dyn AtTransport, _mode: i32) -> Result<(), String> {
        Ok(())
    }

    fn query_qos(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<QosInfo, String> {
        Ok(QosInfo {
            cqi: String::new(),
            ul_bandwidth: String::new(),
            dl_bandwidth: String::new(),
        })
    }
}

/// Decode SYSCFGEX lteband hex bitmask to band number strings
fn decode_syscfgex_lteband(hex: &str) -> Vec<String> {
    let val = u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
    (0..64u64)
        .filter(|&bit| val & (1 << bit) != 0)
        .map(|bit| (bit + 1).to_string())
        .collect()
}
