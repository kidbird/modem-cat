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

fn get_ant_values(t: &mut dyn AtTransport, chip: &QuectelChip) -> Vec<String> {
    let none = vec![String::new(), String::new(), String::new(), String::new()];
    match chip {
        QuectelChip::Qualcomm => match t.send_at("AT+QRSSI") {
            Ok(r) => {
                let ant = parse_qrssi(&r);
                if ant.iter().any(|v| !v.is_empty()) { ant } else { none }
            }
            Err(_) => none,
        },
        QuectelChip::UniSoc => match t.send_at("AT+QANTRSSI?") {
            Ok(r) => {
                let ant = parse_qantrssi(&r);
                if ant.iter().any(|v| !v.is_empty()) { ant } else { none }
            }
            Err(_) => none,
        },
    }
}

fn send_and_check(t: &mut dyn AtTransport, cmd: &str) -> Result<String, String> {
    let resp = send_and_delay(t, cmd)?;
    if is_ok(&resp) {
        Ok(resp)
    } else {
        Err(format!("AT command failed: {} => {}", cmd, resp.trim()))
    }
}

/// Parse `AT+QNWLOCK` or `AT+QNWLOCKFREQ` response.
/// Returns (arfcn, pci) pairs for each active lock entry.
fn parse_qnwlock_response(resp: &str, prefix: &str) -> Vec<(String, String)> {
    let mut items = Vec::new();
    for line in resp.lines() {
        let line = line.trim().trim_start_matches('+');
        let key = format!("{}:", prefix);
        if !line.starts_with(&key) {
            continue;
        }
        let data = line[key.len()..].trim();
        let parts: Vec<&str> = data.split(',').map(|s| s.trim().trim_matches('"')).collect();
        if parts.len() < 2 {
            continue;
        }
        let (arfcn, pci) = if parts.len() >= 3 && (parts[1] == "0" || parts[1] == "1") {
            // format: "common/5g",<enable>,<arfcn>[,<pci>]
            if parts[1] != "1" { continue; }
            let arfcn = parts[2].to_string();
            let pci = parts.get(3).unwrap_or(&"").to_string();
            (arfcn, pci)
        } else {
            // format: "common/5g",<arfcn>[,<pci>]  (presence = locked)
            let arfcn = parts[1].to_string();
            let pci = parts.get(2).unwrap_or(&"").to_string();
            (arfcn, pci)
        };
        if !arfcn.is_empty() && arfcn != "0" {
            items.push((arfcn, pci));
        }
    }
    items
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
        match self.chip {
            QuectelChip::Qualcomm => {
                let resp = send_and_delay(t, "AT+QMAP=\"MPDN_status\"")?;
                if qualcomm::parse_mpdn_connect_status(&resp) {
                    Ok("已连接".to_string())
                } else {
                    Ok("未连接".to_string())
                }
            }
            QuectelChip::UniSoc => {
                let resp = send_and_delay(t, "AT+CGACT?")?;
                let contexts = parse_cgact(&resp);
                if contexts.iter().any(|(_, s)| *s == 1) {
                    Ok("已连接".to_string())
                } else {
                    Ok("未连接".to_string())
                }
            }
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

        let ant_values = get_ant_values(t, &self.chip);

        Ok(SignalInfo {
            rsrp: cell.rsrp,
            rsrq: cell.rsrq,
            sinr: cell.sinr,
            ant_values,
        })
    }

    fn query_neighbor_cells(&mut self, t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let resp = send_and_delay(t, r#"AT+QENG="neighbourcell""#)?;
        log::info!("QENG neighbourcell raw response: {}", resp.replace('\n', "\\n").replace('\r', ""));
        let result = parse_qeng_neighbour_cells(&resp);
        log::info!("QENG neighbourcell parsed: LTE={} NR={}", result.lte.len(), result.nr.len());
        Ok(result)
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
        let active_set: std::collections::HashSet<i32> = parse_cgact(&cgact_resp)
            .into_iter()
            .filter(|(_, status)| *status == 1)
            .map(|(cid, _)| cid)
            .collect();
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
        let rf_resp = send_and_delay(t, r#"AT+QNWPREFCFG="rf_band""#)?;
        let (lte_spec, nr_spec) = parse_qnwprefcfg_rf_band(&rf_resp);
        let lte_resp = send_and_delay(t, r#"AT+QNWPREFCFG="lte_band""#)?;
        let nr_resp = send_and_delay(t, r#"AT+QNWPREFCFG="nr5g_band""#)?;
        Ok(BandConfig {
            lte_locked: parse_qnwprefcfg_bands(&lte_resp, "lte_band"),
            nr_locked: parse_qnwprefcfg_bands(&nr_resp, "nr5g_band"),
            lte_spec,
            nr_spec,
        })
    }

    fn set_lte_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        send_and_check(t, &format!("AT+QNWPREFCFG=\"lte_band\",{}", bands))?;
        Ok(())
    }

    fn set_nr5g_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        send_and_check(t, &format!("AT+QNWPREFCFG=\"nr5g_band\",{}", bands))?;
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
        let rf_resp = match t.send_at(r#"AT+QNWPREFCFG="rf_band""#) {
            Ok(r) => r,
            Err(e) => {
                log::warn!("AT+QNWPREFCFG=\"rf_band\" failed: {}", e);
                String::new()
            }
        };
        let (lte_spec, nr_spec) = parse_qnwprefcfg_rf_band(&rf_resp);

        let lte_resp = send_and_delay(t, r#"AT+QNWPREFCFG="lte_band""#)?;
        let nr_resp = send_and_delay(t, r#"AT+QNWPREFCFG="nr5g_band""#)?;

        Ok(BandConfig {
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
        send_and_check(t, &format!("AT+CFUN={}", mode))?;
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
        let eth_at = match self.chip {
            QuectelChip::Qualcomm => match send_and_delay(t, r#"AT+QCFG="eth_at""#) {
                Ok(r) => parse_qcfg_int(&r, "eth_at").unwrap_or(0) == 1,
                Err(_) => false,
            },
            QuectelChip::UniSoc => false,
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
            // UI-only stubs — AT command to be wired up later
            "armLog" | "cpLog" => {}
            _ => return Err(format!("Unknown feature: {}", feat)),
        }
        Ok(())
    }

    fn query_cell_lock(&mut self, t: &mut dyn AtTransport) -> Result<Vec<CellLockEntry>, String> {
        let mut entries = Vec::new();

        if let Ok(resp) = t.send_at(r#"AT+QNWLOCK="common/5g""#) {
            for (arfcn, pci) in parse_qnwlock_response(&resp, "QNWLOCK") {
                entries.push(CellLockEntry { lock_type: "cell".to_string(), arfcn, pci });
            }
        }
        // QNWLOCKFREQ is UniSoc-only
        if matches!(self.chip, QuectelChip::UniSoc) {
            if let Ok(resp) = t.send_at(r#"AT+QNWLOCKFREQ="common/5g""#) {
                for (arfcn, _) in parse_qnwlock_response(&resp, "QNWLOCKFREQ") {
                    entries.push(CellLockEntry { lock_type: "freq".to_string(), arfcn, pci: String::new() });
                }
            }
        }
        Ok(entries)
    }

    fn set_cell_lock(&mut self, t: &mut dyn AtTransport, arfcn: &str, pci: &str, scs: &str, band: &str) -> Result<(), String> {
        let cmd = match self.chip {
            QuectelChip::Qualcomm => {
                // Qualcomm: AT+QNWLOCK="common/5g",<pci>,<arfcn>,<scs>,<band>
                format!(r#"AT+QNWLOCK="common/5g",{},{},{},{}"#, pci, arfcn, scs, band)
            }
            _ => {
                if !pci.is_empty() {
                    format!(r#"AT+QNWLOCK="common/5g",1,{},{}"#, arfcn, pci)
                } else {
                    format!(r#"AT+QNWLOCKFREQ="common/5g",1,{}"#, arfcn)
                }
            }
        };
        let resp = t.send_at(&cmd)?;
        if !is_ok(&resp) {
            return Err(format!("Cell lock failed: {}", resp.trim()));
        }
        Ok(())
    }

    fn clear_cell_lock(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => {
                let r = t.send_at(r#"AT+QNWLOCK="common/5g",0"#)?;
                if !is_ok(&r) { return Err(format!("Failed to clear cell lock: {}", r.trim())); }
            }
            _ => {
                let r1 = t.send_at(r#"AT+QNWLOCK="common/5g",0"#)?;
                let r2 = t.send_at(r#"AT+QNWLOCKFREQ="common/5g",0"#)?;
                if !is_ok(&r1) || !is_ok(&r2) {
                    return Err(format!("Failed to clear cell lock: {} / {}", r1.trim(), r2.trim()));
                }
            }
        }
        Ok(())
    }

    fn set_plmn_lock(&mut self, t: &mut dyn AtTransport, plmn: &str, password: &str) -> Result<(), String> {
        let cmd = format!(r#"AT+QSIMLOCK="PN","{}",2,"{}""#, password, plmn);
        let resp = t.send_at(&cmd)?;
        if !is_ok(&resp) {
            return Err(format!("PLMN lock failed: {}", resp.trim()));
        }
        Ok(())
    }

    fn clear_plmn_lock(&mut self, t: &mut dyn AtTransport, password: &str) -> Result<(), String> {
        let resp = t.send_at(&format!(r#"AT+QSIMLOCK="PN","{}""#, password))?;
        if !is_ok(&resp) {
            return Err(format!("PLMN unlock failed: {}", resp.trim()));
        }
        Ok(())
    }

    fn query_qos(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<QosInfo, String> {
        let cmd = format!("AT+C5GQOSRDP={}", cid);
        let resp = send_and_delay(t, &cmd)?;
        log::info!("C5GQOSRDP raw response (cid={}): {}", cid, resp.replace('\n', "\\n").replace('\r', ""));
        let (cqi, ul_bw, dl_bw) = parse_c5gqosrdp(&resp);
        log::info!("QosInfo parsed: cqi={}, ul_bw={}, dl_bw={}", cqi, ul_bw, dl_bw);
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
        Ok(get_ant_values(t, &self.chip))
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

    fn query_nat_mode(&mut self, t: &mut dyn AtTransport) -> Result<i32, String> {
        let r = send_and_delay(t, r#"AT+QCFG="nat""#)?;
        Ok(parse_qcfg_int(&r, "nat").unwrap_or(0))
    }

    fn set_nat_mode(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        let resp = send_and_delay(t, &format!(r#"AT+QCFG="nat",{}"#, mode))?;
        if !is_ok(&resp) {
            return Err(format!("Failed to set nat: {}", resp));
        }
        Ok(())
    }

    fn query_qualcomm_config(&mut self, t: &mut dyn AtTransport) -> Result<QualcommConfig, String> {
        if matches!(self.chip, QuectelChip::UniSoc) {
            return Err("query_qualcomm_config not supported on UniSoc platform".to_string());
        }
        let resp = send_and_delay(t, r#"AT+QCFG="usbnet""#)?;
        let usbnet = parse_qcfg_usbnet(&resp).unwrap_or(0);

        let resp = send_and_delay(t, r#"AT+QCFG="data_interface""#)?;
        let data_interface = parse_qcfg_data_interface(&resp).unwrap_or_else(|| "0,0".to_string());

        let resp = send_and_delay(t, r#"AT+QCFG="pcie/mode""#)?;
        let pcie_mode = parse_qcfg_int(&resp, "pcie/mode").unwrap_or(0);

        let resp = send_and_delay(t, r#"AT+QCFG="usbspeed""#)?;
        let usbspeed = parse_qcfg_usbspeed(&resp).unwrap_or_else(|| "2.0".to_string());

        let resp = send_and_delay(t, r#"AT+QETH="eth_driver""#)?;
        let eth_driver = parse_qeth_eth_driver(&resp).unwrap_or_else(|| "none".to_string());

        let mpdn_resp = t.send_at("AT+QMAP=\"MPDN_rule\"").unwrap_or_default();
        let ippt_mode = qualcomm::parse_mpdn_ippt_mode(&mpdn_resp);

        Ok(QualcommConfig {
            usbnet,
            data_interface,
            pcie_mode,
            usbspeed,
            eth_driver,
            ippt_mode,
        })
    }

    fn set_qualcomm_config(&mut self, t: &mut dyn AtTransport, param: &str, value: &str) -> Result<(), String> {
        if matches!(self.chip, QuectelChip::UniSoc) {
            return Err("set_qualcomm_config not supported on UniSoc platform".to_string());
        }
        let cmd = match param {
            "usbnet" => {
                let mode: i32 = value.parse().map_err(|e| format!("Invalid usbnet value: {}", e))?;
                format!(r#"AT+QCFG="usbnet",{}"#, mode)
            }
            "dataInterface" => {
                format!(r#"AT+QCFG="data_interface",{}"#, value)
            }
            "pcieMode" => {
                let mode: i32 = value.parse().map_err(|e| format!("Invalid pcieMode value: {}", e))?;
                format!(r#"AT+QCFG="pcie/mode",{}"#, mode)
            }
            "usbspeed" => {
                format!(r#"AT+QCFG="usbspeed","{}""#, value)
            }
            "ethDriver" => {
                if let Some(stripped) = value.strip_prefix('-') {
                    format!(r#"AT+QETH="eth_driver","{}",0"#, stripped)
                } else {
                    format!(r#"AT+QETH="eth_driver","{}",1"#, value)
                }
            }
            "ippt" => {
                let mode: i32 = value.parse().map_err(|_| format!("Invalid IPPT mode: {}", value))?;
                match mode {
                    0 => {
                        send_and_check(t, r#"AT+QMAP="mPDN_rule",0"#)?;
                    }
                    1 => {
                        // Routing: always disable first, then configure
                        let _ = t.send_at(r#"AT+QMAP="mPDN_rule",0"#);
                        send_and_check(t, r#"AT+QMAP="mPDN_rule",0,1,0,0,1,"FF:FF:FF:FF:FF:FF""#)?;
                    }
                    2 => {
                        // Bridging (IPPT): always disable first, then configure
                        let _ = t.send_at(r#"AT+QMAP="mPDN_rule",0"#);
                        send_and_check(t, r#"AT+QMAP="mPDN_rule",0,1,0,1,1,"FF:FF:FF:FF:FF:FF""#)?;
                    }
                    _ => return Err(format!("Invalid IPPT mode: {}", value)),
                }
                return Ok(());
            }
            _ => return Err(format!("Unsupported Qualcomm parameter: {}", param)),
        };

        send_and_check(t, &cmd)?;
        Ok(())
    }

    fn query_modem_status(
        &mut self,
        t: &mut dyn AtTransport,
    ) -> Result<ModemStatus, String> {
        // ── Phase 1: AT I/O — send all commands back-to-back, no parsing in between ──
        let cpin_raw  = t.send_at("AT+CPIN?")?;
        let imei_raw  = t.send_at("AT+CGSN")?;

        let iccid_cmd = match self.chip { QuectelChip::Qualcomm => "AT+ICCID", QuectelChip::UniSoc => "AT+CCID" };
        let iccid_p   = t.send_at(iccid_cmd)?;
        // ICCID fallback requires the primary parse result to decide whether to send a second command.
        let iccid_raw = if parse_iccid(&iccid_p).is_empty() { t.send_at("AT+QCCID")? } else { iccid_p };

        let qeng_raw  = t.send_at(r#"AT+QENG="servingcell""#)?;
        log::info!("QENG raw response: {}", qeng_raw.replace('\n', "\\n").replace('\r', ""));

        let cops_raw  = t.send_at("AT+COPS?")?;

        let ant_raw = match self.chip {
            QuectelChip::Qualcomm => t.send_at("AT+QRSSI").ok(),
            QuectelChip::UniSoc   => t.send_at("AT+QANTRSSI?").ok(),
        };

        let cgact_raw = t.send_at("AT+CGACT?")?;
        let mpdn_raw = if matches!(self.chip, QuectelChip::Qualcomm) {
            t.send_at("AT+QMAP=\"MPDN_status\"").ok()
        } else {
            None
        };

        // ── Phase 2: parse — AT bus is free, all CPU work happens here ──
        let sim_status  = parse_cpin(&cpin_raw);
        let imei        = parse_cgsn(&imei_raw);
        let iccid       = parse_iccid(&iccid_raw);

        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        let serving_cell = parse_qeng_serving_cell(&qeng_raw, qualcomm_bw);
        log::info!(
            "ServingCell parsed: tech={}, state={}, pci={}, cell_id={}, arfcn={}, rsrp={}, sinr={}",
            serving_cell.tech, serving_cell.mobility_state, serving_cell.pci,
            serving_cell.cell_id, serving_cell.arfcn, serving_cell.rsrp, serving_cell.sinr
        );

        let (cops_name, _) = parse_cops_with_act(&cops_raw);
        let operator = if cops_name.is_empty() && !serving_cell.operator_mcc.is_empty() {
            format!("{}{}", serving_cell.operator_mcc, serving_cell.operator_mnc)
        } else {
            cops_name
        };

        let ant_values = match ant_raw {
            Some(r) => {
                let ant = match self.chip {
                    QuectelChip::Qualcomm => parse_qrssi(&r),
                    QuectelChip::UniSoc   => parse_qantrssi(&r),
                };
                if ant.iter().any(|v| !v.is_empty()) { ant } else { vec![String::new(); 4] }
            }
            None => vec![String::new(); 4],
        };

        let conn_status = {
            let mpdn_ok = mpdn_raw.as_ref().map(|r| qualcomm::parse_mpdn_connect_status(r));
            if mpdn_ok == Some(true) {
                "已连接".to_string()
            } else {
                let contexts = parse_cgact(&cgact_raw);
                if contexts.iter().any(|(_, s)| *s == 1) { "已连接".to_string() } else { "未连接".to_string() }
            }
        };

        let reg_status = serving_cell.mobility_state.clone();

        Ok(ModemStatus {
            sim_status,
            reg_status,
            conn_status,
            imei,
            iccid,
            operator,
            network_type: serving_cell.tech,
            band: serving_cell.band,
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
            chip_vendor: self.vendor().as_str().to_string(),
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

    fn set_5glan(&mut self, t: &mut dyn AtTransport, cid: i32, enabled: bool, vlan_id: i32) -> Result<(), String> {
        let state = if enabled { 1 } else { 0 };
        let resp = send_and_delay(t, &format!(r#"AT+QCFG="5glan",{},{},{}"#, cid, state, vlan_id))?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("Failed to set 5GLAN: {}", resp))
        }
    }

    fn connect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::connect_data(t),
            QuectelChip::UniSoc => unisoc::connect_data(t, cid),
        }
    }

    fn disconnect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::disconnect_data(t),
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