use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;
use parser::*;

pub mod band_db;
pub mod parser;
pub mod qualcomm;
pub mod unisoc;

fn send_and_delay(t: &mut dyn AtTransport, cmd: &str) -> Result<String, String> {
    let result = t.send_at(cmd)?;
    std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(result)
}

fn get_ant_values(t: &mut dyn AtTransport, chip: &QuectelChip) -> Result<Vec<String>, String> {
    let none = vec![String::new(), String::new(), String::new(), String::new()];
    let response = match chip {
        QuectelChip::Qualcomm => t.send_at("AT+QRSRP")?,
        QuectelChip::UniSoc => t.send_at("AT+QANTRSSI?")?,
    };
    let ant = match chip {
        QuectelChip::Qualcomm => parse_qrsrp(&response),
        QuectelChip::UniSoc => parse_qantrssi(&response),
    };
    if ant.iter().any(|v| !v.is_empty()) {
        Ok(ant)
    } else {
        Ok(none)
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

fn query_required_qcfg_bool(
    t: &mut dyn AtTransport,
    command: &str,
    key: &str,
) -> Result<bool, String> {
    let response = send_and_delay(t, command)?;
    parse_qcfg_int(&response, key)
        .map(|value| value == 1)
        .ok_or_else(|| format!("Failed to parse live {} state from {}", key, response.trim()))
}

fn query_required_qcfg_usbcfg_adb(t: &mut dyn AtTransport) -> Result<bool, String> {
    let response = t.send_at(r#"AT+QCFG="usbcfg""#)?;
    parse_qcfg_usbcfg_adb(&response)
        .ok_or_else(|| format!("Failed to parse live usbcfg state from {}", response.trim()))
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
        let parts: Vec<&str> = data
            .split(',')
            .map(|s| s.trim().trim_matches('"'))
            .collect();
        if parts.len() < 2 {
            continue;
        }
        let (arfcn, pci) = if parts.len() >= 3 && (parts[1] == "0" || parts[1] == "1") {
            // format: "common/5g",<enable>,<arfcn>[,<pci>]
            if parts[1] != "1" {
                continue;
            }
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

/// Quectel chip variant. `#[non_exhaustive]` blocks external crates from
/// constructing/exhaustively-matching this enum, so adding a new variant
/// (e.g. an as-yet-unreleased Mediatek-based Quectel SKU) won't silently
/// break downstream consumers. Internal `match` arms in this crate remain
/// exhaustive at compile time — when a new variant is added, the compiler
/// will surface every `match self.chip { ... }` that needs updating.
#[non_exhaustive]
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
        // ASR 平台在 UI/序列化层独立标识，AT 分发仍走 self.chip = UniSoc。
        if self.model.to_uppercase().contains("RG255") {
            return ChipsetVendor::Asr;
        }
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
        if iccid.is_empty() {
            return Err(format!("{} returned no parsable ICCID", cmd));
        }
        Ok(iccid)
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

        let ant_values = get_ant_values(t, &self.chip)?;

        Ok(SignalInfo {
            rsrp: cell.rsrp,
            rsrq: cell.rsrq,
            sinr: cell.sinr,
            ant_values,
        })
    }

    fn query_neighbor_cells(&mut self, t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let resp = send_and_delay(t, r#"AT+QENG="neighbourcell""#)?;
        log::info!(
            "QENG neighbourcell raw response: {}",
            resp.replace('\n', "\\n").replace('\r', "")
        );
        let result = parse_qeng_neighbour_cells(&resp);
        log::info!(
            "QENG neighbourcell parsed: LTE={} NR={}",
            result.lte.len(),
            result.nr.len()
        );
        Ok(result)
    }

    fn query_hardware_info(&mut self, t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let cgmm_resp = send_and_delay(t, "AT+CGMM")?;
        let model = parse_cgmm(&cgmm_resp);

        let cgmi_resp = send_and_delay(t, "AT+CGMI")?;
        let manufacturer = parse_cgmm(&cgmi_resp);

        let gmr_resp = t.send_at("AT+GMR")?;
        let firmware = parse_gmr(&gmr_resp);

        let baseline_resp = t.send_at("AT+QBASELINE")?;
        let (ap_baseline, cp_baseline) = parse_qbaseline(&baseline_resp);

        let temp_resp = t.send_at("AT+QTEMP")?;
        let is_asr = self.model.to_uppercase().contains("RG255");
        let (soc_temp, pa_temp) = if is_asr {
            let info = parse_qtemp_asr(&temp_resp);
            (info.soc_temp, info.pa_temp)
        } else {
            match self.chip {
                QuectelChip::UniSoc => {
                    let info = parse_qtemp_unisoc(&temp_resp);
                    (info.soc_temp, info.pa_temp)
                }
                QuectelChip::Qualcomm => parse_qtemp_rich(&temp_resp),
            }
        };

        // SN read via AT+EGMR=0,5. Like ICCID, this is a modem-adjacent
        // identifier that may be unsupported on some platforms/firmware.
        // Failure must NOT abort the hardware-info query — IMEI, model,
        // firmware remain valid even if this command fails.
        let serial_number = t
            .send_at("AT+EGMR=0,5")
            .map(|resp| parse_egmr_sn(&resp))
            .unwrap_or_default();

        Ok(HardwareInfo {
            model,
            manufacturer,
            firmware,
            ap_baseline,
            cp_baseline,
            soc_temp,
            pa_temp,
            serial_number,
        })
    }

    fn query_temperature(&mut self, t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        let resp = send_and_delay(t, "AT+QTEMP")?;
        let is_asr = self.model.to_uppercase().contains("RG255");
        if is_asr {
            Ok(parse_qtemp_asr(&resp))
        } else {
            match self.chip {
                QuectelChip::UniSoc => Ok(parse_qtemp_unisoc(&resp)),
                QuectelChip::Qualcomm => Ok(parse_qtemp(&resp)),
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
            return Ok(entries
                .into_iter()
                .filter(|e| e.cid >= 1 && e.cid <= 8)
                .collect());
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

    fn set_bands(&mut self, t: &mut dyn AtTransport, lte: &str, nr: &str) -> Result<(), String> {
        if !lte.is_empty() {
            send_and_check(t, &format!(r#"AT+QNWPREFCFG="lte_band",{}"#, lte))?;
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
        if !nr.is_empty() {
            send_and_check(t, &format!(r#"AT+QNWPREFCFG="nr5g_band",{}"#, nr))?;
        }
        Ok(())
    }

    fn query_bands_with_spec(&mut self, t: &mut dyn AtTransport) -> Result<BandConfig, String> {
        // ── Spec bands: Qualcomm queries modem, UniSoc uses preset database ──
        let (lte_spec, nr_spec) = match self.chip {
            QuectelChip::Qualcomm => {
                let rf_resp = send_and_delay(t, r#"AT+QNWPREFCFG="rf_band""#)?;
                parse_qnwprefcfg_rf_band(&rf_resp)
            }
            QuectelChip::UniSoc => match band_db::get_supported_bands(&self.model) {
                Some(bands) => {
                    log::info!("UniSoc model {}, using band_db preset", self.model);
                    (
                        band_db::format_bands(bands.lte, "B"),
                        band_db::format_bands(bands.nr, "n"),
                    )
                }
                None => {
                    log::warn!(
                        "UniSoc model {} not in band_db, spec bands empty",
                        self.model
                    );
                    (vec![], vec![])
                }
            },
        };

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

    fn query_feature_toggles(&mut self, t: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
        let pcie_mode = query_required_qcfg_bool(t, r#"AT+QCFG="pcie/mode""#, "pcie/mode")?;
        let ethernet = query_required_qcfg_bool(t, r#"AT+QCFG="ethernet""#, "ethernet")?;
        let proxyarp = query_required_qcfg_bool(t, r#"AT+QCFG="proxyarp""#, "proxyarp")?;
        let uart_at = query_required_qcfg_bool(t, r#"AT+QCFG="uartat""#, "uartat")?;
        let eth_at = match self.chip {
            QuectelChip::Qualcomm => {
                query_required_qcfg_bool(t, r#"AT+QCFG="eth_at""#, "eth_at")?
            }
            QuectelChip::UniSoc => false,
        };
        let adb = query_required_qcfg_usbcfg_adb(t)?;
        let napt = query_required_qcfg_bool(t, r#"AT+QCFG="napt""#, "napt")?;
        let netmask = query_required_qcfg_bool(t, r#"AT+QCFG="netmask""#, "netmask")?;

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
                        let mut parts: Vec<String> = rest.split(',').map(|s| s.to_string()).collect();
                        if parts.len() >= 2 {
                            // Ensure correct VID/PID for this model
                            let (vid, pid) = ChipsetVendor::usb_id_for_model(&self.model);
                            parts[0] = format!("0x{:04X}", vid);
                            parts[1] = format!("0x{:04X}", pid);
                            // ADB flag is the second-to-last parameter
                            let adb_idx = parts.len() - 2;
                            parts[adb_idx] = if on { "1".to_string() } else { "0".to_string() };
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
                entries.push(CellLockEntry {
                    lock_type: "cell".to_string(),
                    arfcn,
                    pci,
                });
            }
        }
        // QNWLOCKFREQ is UniSoc-only
        if matches!(self.chip, QuectelChip::UniSoc) {
            if let Ok(resp) = t.send_at(r#"AT+QNWLOCKFREQ="common/5g""#) {
                for (arfcn, _) in parse_qnwlock_response(&resp, "QNWLOCKFREQ") {
                    entries.push(CellLockEntry {
                        lock_type: "freq".to_string(),
                        arfcn,
                        pci: String::new(),
                    });
                }
            }
        }
        Ok(entries)
    }

    fn set_cell_lock(
        &mut self,
        t: &mut dyn AtTransport,
        arfcn: &str,
        pci: &str,
        scs: &str,
        band: &str,
    ) -> Result<(), String> {
        let cmd = match self.chip {
            QuectelChip::Qualcomm => {
                // Qualcomm: AT+QNWLOCK="common/5g",<pci>,<arfcn>,<scs>,<band>
                format!(
                    r#"AT+QNWLOCK="common/5g",{},{},{},{}"#,
                    pci, arfcn, scs, band
                )
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
                if !is_ok(&r) {
                    return Err(format!("Failed to clear cell lock: {}", r.trim()));
                }
            }
            _ => {
                let r1 = t.send_at(r#"AT+QNWLOCK="common/5g",0"#)?;
                let r2 = t.send_at(r#"AT+QNWLOCKFREQ="common/5g",0"#)?;
                if !is_ok(&r1) || !is_ok(&r2) {
                    return Err(format!(
                        "Failed to clear cell lock: {} / {}",
                        r1.trim(),
                        r2.trim()
                    ));
                }
            }
        }
        Ok(())
    }

    fn set_plmn_lock(
        &mut self,
        t: &mut dyn AtTransport,
        plmn: &str,
        password: &str,
    ) -> Result<(), String> {
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
        log::info!(
            "C5GQOSRDP raw response (cid={}): {}",
            cid,
            resp.replace('\n', "\\n").replace('\r', "")
        );
        let (cqi, ul_bw, dl_bw) = parse_c5gqosrdp(&resp);
        log::info!(
            "QosInfo parsed: cqi={}, ul_bw={}, dl_bw={}",
            cqi,
            ul_bw,
            dl_bw
        );
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
        get_ant_values(t, &self.chip)
    }

    fn factory_reset(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        let cmd = match self.chip {
            QuectelChip::Qualcomm => "AT&F",
            QuectelChip::UniSoc => "AT+QPRTPARA=3",
        };
        let resp = t.send_at(cmd)?;
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

    fn query_vlan(&mut self, t: &mut dyn AtTransport) -> Result<Vec<i32>, String> {
        if matches!(self.chip, QuectelChip::UniSoc) {
            return Err("VLAN not supported on UniSoc platform".to_string());
        }
        let resp = send_and_delay(t, r#"AT+QMAP="VLAN""#)?;
        // Response lines:
        //   +QMAP: "VLAN",0          ← physical default LAN, always present, skip
        //   +QMAP: "VLAN",<vid>,<type>  ← enabled VLAN (type: 1=ETH, 2=ECM, 3=RNDIS)
        let mut vids = Vec::new();
        for line in resp.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("+QMAP:") {
                let parts: Vec<&str> = rest
                    .trim()
                    .split(',')
                    .map(|s| s.trim().trim_matches('"'))
                    .collect();
                if parts.first().map(|s| s.eq_ignore_ascii_case("vlan")) == Some(true) {
                    let vid: i32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                    // vid=0 is the physical default LAN entry, skip it
                    // vid>0 with a type field means this VLAN is enabled
                    if vid > 0 && parts.get(2).is_some() {
                        vids.push(vid);
                    }
                }
            }
        }
        Ok(vids)
    }

    fn set_vlan(
        &mut self,
        t: &mut dyn AtTransport,
        vlan_id: i32,
        enabled: bool,
    ) -> Result<(), String> {
        if matches!(self.chip, QuectelChip::UniSoc) {
            return Err("VLAN not supported on UniSoc platform".to_string());
        }
        // Enable:  AT+QMAP="VLAN",<vid>,"enable",1  (1=ETH)
        // Disable: AT+QMAP="VLAN",<vid>,"disable"
        let cmd = if enabled {
            format!(r#"AT+QMAP="VLAN",{},"enable",1"#, vlan_id)
        } else {
            format!(r#"AT+QMAP="VLAN",{},"disable""#, vlan_id)
        };
        let resp = send_and_delay(t, &cmd)?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("VLAN set failed: {}", resp))
        }
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
        parse_qcfg_int(&r, "nat")
            .ok_or_else(|| format!("Failed to parse live nat state from {}", r.trim()))
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
        let usbnet = parse_qcfg_usbnet(&resp)
            .ok_or_else(|| "Failed to parse AT+QCFG=\"usbnet\" response".to_string())?;

        let resp = send_and_delay(t, r#"AT+QCFG="data_interface""#)?;
        let data_interface = parse_qcfg_data_interface(&resp)
            .ok_or_else(|| "Failed to parse AT+QCFG=\"data_interface\" response".to_string())?;

        let resp = send_and_delay(t, r#"AT+QCFG="pcie/mode""#)?;
        let pcie_mode = parse_qcfg_int(&resp, "pcie/mode")
            .ok_or_else(|| "Failed to parse AT+QCFG=\"pcie/mode\" response".to_string())?;

        let resp = send_and_delay(t, r#"AT+QCFG="usbspeed""#)?;
        let usbspeed = parse_qcfg_usbspeed(&resp)
            .ok_or_else(|| "Failed to parse AT+QCFG=\"usbspeed\" response".to_string())?;

        let resp = send_and_delay(t, r#"AT+QETH="eth_driver""#)?;
        let eth_driver = parse_qeth_eth_driver(&resp)
            .ok_or_else(|| "Failed to parse AT+QETH=\"eth_driver\" response".to_string())?;

        let mpdn_resp = t.send_at("AT+QMAP=\"MPDN_rule\"")?;
        let ippt_mode = qualcomm::parse_mpdn_ippt_mode(&mpdn_resp);

        let auto_resp = t.send_at("AT+QMAP=\"auto_connect\",0")?;
        let auto_connect = qualcomm::parse_auto_connect(&auto_resp);

        Ok(QualcommConfig {
            usbnet,
            data_interface,
            pcie_mode,
            usbspeed,
            eth_driver,
            ippt_mode,
            auto_connect,
        })
    }

    fn set_qualcomm_config(
        &mut self,
        t: &mut dyn AtTransport,
        param: &str,
        value: &str,
    ) -> Result<(), String> {
        if matches!(self.chip, QuectelChip::UniSoc) {
            return Err("set_qualcomm_config not supported on UniSoc platform".to_string());
        }
        let cmd = match param {
            "usbnet" => {
                let mode: i32 = value
                    .parse()
                    .map_err(|e| format!("Invalid usbnet value: {}", e))?;
                format!(r#"AT+QCFG="usbnet",{}"#, mode)
            }
            "dataInterface" => {
                format!(r#"AT+QCFG="data_interface",{}"#, value)
            }
            "pcieMode" => {
                let mode: i32 = value
                    .parse()
                    .map_err(|e| format!("Invalid pcieMode value: {}", e))?;
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
                let mode: i32 = value
                    .parse()
                    .map_err(|_| format!("Invalid IPPT mode: {}", value))?;

                // Clear rule 0 first. We ignore the output and sleep 20ms as it might not respond.
                let _ = t.send_at(r#"AT+QMAP="MPDN_rule",0"#);
                std::thread::sleep(std::time::Duration::from_millis(20));

                match mode {
                    0 => {
                        qualcomm::set_auto_connect(t, 0, 0, None)?;
                    }
                    1 => {
                        // IPPT ETH (IPPT Mode = 1)
                        send_and_check(t, r#"AT+QMAP="MPDN_rule",0,1,0,1,1,"FF:FF:FF:FF:FF:FF""#)?;
                        qualcomm::set_auto_connect(t, 0, 1, Some(1))?;
                    }
                    3 => {
                        // IPPT USB (IPPT Mode = 3)
                        send_and_check(t, r#"AT+QMAP="MPDN_rule",0,1,0,3,1,"FF:FF:FF:FF:FF:FF""#)?;
                        qualcomm::set_auto_connect(t, 0, 1, Some(1))?;
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

    fn query_modem_status(&mut self, t: &mut dyn AtTransport) -> Result<ModemStatus, String> {
        let cpin_raw = t.send_at("AT+CPIN?")?;
        let imei_raw = t.send_at("AT+CGSN")?;

        let sim_status = parse_cpin(&cpin_raw);
        let iccid_cmd = match self.chip {
            QuectelChip::Qualcomm => "AT+ICCID",
            QuectelChip::UniSoc => "AT+CCID",
        };
        // ICCID only makes sense with a usable SIM. Gate on READY so we don't
        // send AT+ICCID/AT+CCID when there is no card (NO SIM, "SIM not
        // inserted", SIM PIN, etc. — the latter can come back as a plain
        // +CPIN data line that parse_cpin returns verbatim, not "NO SIM").
        // An ICCID read failure must NOT abort the status query: IMEI,
        // operator and registration are still valid and the frontend shows
        // '--' for iccid when SIM is not READY. The strict contract lives in
        // the dedicated query_iccid() command instead.
        let iccid = if sim_status == "READY" {
            match t.send_at(iccid_cmd) {
                Ok(iccid_raw) => {
                    let parsed = parse_iccid(&iccid_raw);
                    if parsed.is_empty() {
                        log::warn!(
                            "SIM is READY but {} returned no parsable ICCID: {:?}",
                            iccid_cmd,
                            iccid_raw.trim()
                        );
                    }
                    parsed
                }
                Err(e) => {
                    log::warn!("{} failed, leaving ICCID empty: {}", iccid_cmd, e);
                    String::new()
                }
            }
        } else {
            String::new()
        };

        let qeng_raw = t.send_at(r#"AT+QENG="servingcell""#)?;
        log::info!(
            "QENG raw response: {}",
            qeng_raw.replace('\n', "\\n").replace('\r', "")
        );

        let cops_raw = t.send_at("AT+COPS?")?;

        let ant_raw = match self.chip {
            QuectelChip::Qualcomm => t.send_at("AT+QRSRP")?,
            QuectelChip::UniSoc => t.send_at("AT+QANTRSSI?")?,
        };

        let cgact_raw = if matches!(self.chip, QuectelChip::UniSoc) {
            Some(t.send_at("AT+CGACT?")?)
        } else {
            None
        };
        let mpdn_raw = if matches!(self.chip, QuectelChip::Qualcomm) {
            Some(t.send_at("AT+QMAP=\"MPDN_status\"")?)
        } else {
            None
        };

        // ── Phase 2: parse — AT bus is free, all CPU work happens here ──
        let imei = parse_cgsn(&imei_raw);

        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        let serving_cell = parse_qeng_serving_cell(&qeng_raw, qualcomm_bw);
        log::info!(
            "ServingCell parsed: tech={}, state={}, pci={}, cell_id={}, arfcn={}, rsrp={}, sinr={}",
            serving_cell.tech,
            serving_cell.mobility_state,
            serving_cell.pci,
            serving_cell.cell_id,
            serving_cell.arfcn,
            serving_cell.rsrp,
            serving_cell.sinr
        );

        let (cops_name, _) = parse_cops_with_act(&cops_raw);
        let operator = if cops_name.is_empty() && !serving_cell.operator_mcc.is_empty() {
            format!("{}{}", serving_cell.operator_mcc, serving_cell.operator_mnc)
        } else {
            cops_name
        };

        let ant_values = {
            let ant = match self.chip {
                QuectelChip::Qualcomm => parse_qrsrp(&ant_raw),
                QuectelChip::UniSoc => parse_qantrssi(&ant_raw),
            };
            if ant.iter().any(|v| !v.is_empty()) {
                ant
            } else {
                vec![String::new(); 4]
            }
        };

        let conn_status = match self.chip {
            QuectelChip::Qualcomm => {
                let mpdn_raw = mpdn_raw
                    .as_deref()
                    .ok_or("Missing Qualcomm MPDN status response")?;
                if qualcomm::parse_mpdn_connect_status(mpdn_raw) {
                    "已连接".to_string()
                } else {
                    "未连接".to_string()
                }
            }
            QuectelChip::UniSoc => {
                let cgact_raw = cgact_raw
                    .as_deref()
                    .ok_or("Missing UniSoc CGACT response")?;
                let contexts = parse_cgact(cgact_raw);
                if contexts.iter().any(|(_, s)| *s == 1) {
                    "已连接".to_string()
                } else {
                    "未连接".to_string()
                }
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
        send_and_check(
            t,
            &format!(
                "AT+QICSGP={},{},\"{}\",\"{}\",\"{}\",{}",
                cid, ctx, apn, user, pass, auth
            ),
        )?;
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

    fn set_apn_active(
        &mut self,
        t: &mut dyn AtTransport,
        cid: i32,
        active: bool,
    ) -> Result<(), String> {
        let state = if active { 1 } else { 0 };
        let resp = send_and_delay(t, &format!("AT+CGACT={},{}", state, cid))?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!(
                "Failed to {} APN: {}",
                if active { "activate" } else { "deactivate" },
                resp
            ))
        }
    }

    fn query_5glan(&mut self, t: &mut dyn AtTransport) -> Result<Vec<L5GanEntry>, String> {
        if matches!(self.chip, QuectelChip::Qualcomm) {
            return Err("Qualcomm: use query_qualcomm_5glan_status".to_string());
        }
        let resp = send_and_delay(t, r#"AT+QCFG="5glan""#)?;
        Ok(parse_5glan(&resp))
    }

    fn set_5glan(
        &mut self,
        t: &mut dyn AtTransport,
        cid: i32,
        enabled: bool,
        vlan_id: i32,
    ) -> Result<(), String> {
        if matches!(self.chip, QuectelChip::Qualcomm) {
            return Err("Qualcomm: use configure_qualcomm_5glan".to_string());
        }
        let state = if enabled { 1 } else { 0 };
        let resp = send_and_delay(
            t,
            &format!(r#"AT+QCFG="5glan",{},{},{}"#, cid, state, vlan_id),
        )?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("5GLAN set failed: {}", resp.trim()))
        }
    }

    fn configure_qualcomm_5glan(
        &mut self,
        t: &mut dyn AtTransport,
        cid: i32,
        apn: &str,
        snssai: &str,
        profile_id: i32,
        vlan_start: i32,
        vlan_end: i32,
    ) -> Result<(), String> {
        if !matches!(self.chip, QuectelChip::Qualcomm) {
            return Err("Qualcomm 5GLAN L2 only supported on Qualcomm chip".to_string());
        }
        // eth_cfg mode: 1 = data with VLAN ID, 2 = without VLAN ID
        let eth_mode = if vlan_start < 65535 { 1 } else { 2 };
        let resp = send_and_delay(
            t,
            &format!("AT+QNWCFG=\"eth_cfg\",{},{}", profile_id, eth_mode),
        )?;
        if !is_ok(&resp) {
            return Err(format!("eth_cfg failed: {}", resp.trim()));
        }
        // Configure PDP context with S-NSSAI (13 empty fields between APN and SNSSAIs_ind)
        let cmd = format!(
            "AT+CGDCONT={},\"IPV4V6\",\"{}\",,,,,,,,,,,,,,1,\"{}\",",
            cid, apn, snssai
        );
        let resp = send_and_delay(t, &cmd)?;
        if !is_ok(&resp) {
            return Err(format!("CGDCONT failed: {}", resp.trim()));
        }
        // Configure WDS Ethernet profile
        let cmd = format!(
            "AT+QWDSCFG=\"profile\",{},\"Ethernet\",\"{}\",{},{}",
            cid, apn, vlan_start, vlan_end
        );
        let resp = send_and_delay(t, &cmd)?;
        if !is_ok(&resp) {
            return Err(format!("QWDSCFG failed: {}", resp.trim()));
        }
        Ok(())
    }

    fn enable_eth_pdu(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        if !matches!(self.chip, QuectelChip::Qualcomm) {
            return Err("ETH PDU only supported on Qualcomm".to_string());
        }
        let resp = send_and_delay(t, r#"AT+QMAP="ETH_PDU","enable""#)?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("ETH_PDU enable failed: {}", resp.trim()))
        }
    }

    fn connect_qualcomm_5glan(
        &mut self,
        t: &mut dyn AtTransport,
        rule_id: i32,
        cid: i32,
    ) -> Result<(), String> {
        if !matches!(self.chip, QuectelChip::Qualcomm) {
            return Err("Qualcomm 5GLAN L2 only supported on Qualcomm chip".to_string());
        }
        let resp = send_and_delay(
            t,
            &format!("AT+QMAP=\"MPDN_rule\",{},{},0,0,0", rule_id, cid),
        )?;
        if !is_ok(&resp) {
            return Err(format!("mpdn_rule failed: {}", resp.trim()));
        }
        let resp = send_and_delay(t, &format!("AT+QMAP=\"connect\",{},1", rule_id))?;
        if is_ok(&resp) {
            Ok(())
        } else {
            Err(format!("connect failed: {}", resp.trim()))
        }
    }

    fn query_qualcomm_5glan_status(
        &mut self,
        t: &mut dyn AtTransport,
    ) -> Result<Qualcomm5GlanStatus, String> {
        if !matches!(self.chip, QuectelChip::Qualcomm) {
            return Err("Qualcomm 5GLAN status only supported on Qualcomm chip".to_string());
        }
        let eth_resp = t.send_at(r#"AT+QMAP="ETH_PDU""#)?;
        let eth_pdu_enabled = qualcomm::parse_eth_pdu_enabled(&eth_resp);
        let mpdn_resp = t.send_at(r#"AT+QMAP="MPDN_rule""#)?;
        let mpdn_cid = qualcomm::parse_mpdn_rule_cid(&mpdn_resp, 1);
        let status_resp = t.send_at(r#"AT+QMAP="MPDN_status""#)?;
        let connected = qualcomm::parse_mpdn_connect_status_by_rule(&status_resp, 1);
        Ok(Qualcomm5GlanStatus {
            eth_pdu_enabled,
            mpdn_cid,
            connected,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    #[test]
    fn query_iccid_uses_single_primary_command_on_qualcomm() {
        let mut modem = QuectelModem::qualcomm("RM500Q".to_string());
        let mut transport = MockTransport::new(vec!["ERROR"]);

        assert!(modem.query_iccid(&mut transport).is_err());
        assert_eq!(transport.sent, vec!["AT+ICCID"]);
    }

    #[test]
    fn query_iccid_uses_single_primary_command_on_unisoc() {
        let mut modem = QuectelModem::unisoc("RM500U".to_string());
        let mut transport = MockTransport::new(vec!["ERROR"]);

        assert!(modem.query_iccid(&mut transport).is_err());
        assert_eq!(transport.sent, vec!["AT+CCID"]);
    }

    /// UniSoc status query sequence responses (in send order), excluding the
    /// optional ICCID slot which each test fills in or omits as needed.
    ///
    /// Order (mod.rs query_modem_status, UniSoc/ASR):
    ///   AT+CPIN?  AT+CGSN  [AT+CCID]  AT+QENG="servingcell"
    ///   AT+COPS?  AT+QANTRSSI?  AT+CGACT?
    const NO_SIM_PREFIX: &[&str] = &["+CME ERROR: 10", "865123456789011"];
    const NO_SIM_SUFFIX: &[&str] = &[
        "+QENG: \"servingcell\",\"NOCONN\"",
        "+COPS: 0",
        "+QANTRSSI: 0,0,0,0",
        "+CGACT: 1,0",
    ];
    const READY_PREFIX: &[&str] = &["+CPIN: READY", "865123456789011"];
    const READY_SUFFIX: &[&str] = &[
        "+QENG: \"servingcell\",\"NOCONN\"",
        "+COPS: 0,2,\"46001\"",
        "+QANTRSSI: 70,60,50,40",
        "+CGACT: 1,1",
    ];

    #[test]
    fn query_modem_status_skips_iccid_when_no_sim() {
        // CPIN returns +CME ERROR → parse_cpin() => "NO SIM".
        // ICCID must NOT be queried; IMEI / rest still returned.
        let mut modem = QuectelModem::unisoc("RM500U".to_string());
        let mut responses: Vec<&str> = NO_SIM_PREFIX.to_vec();
        responses.extend_from_slice(NO_SIM_SUFFIX);
        let mut transport = MockTransport::new(responses);

        let status = modem.query_modem_status(&mut transport).expect("ok");

        assert_eq!(status.sim_status, "NO SIM");
        assert!(!transport.sent.iter().any(|c| c == "AT+CCID"));
        assert_eq!(status.imei, "865123456789011");
        assert_eq!(status.iccid, "");
    }

    #[test]
    fn query_modem_status_skips_iccid_when_sim_not_ready() {
        // ASR/UniSoc report "no SIM" via a DATA line (not an ERROR line):
        //   +CPIN: SIM not inserted
        // parse_cpin() returns the literal string (not "NO SIM"), so the old
        // `!= "NO SIM"` gate still attempted AT+CCID and failed. The new
        // gate keys off READY, so ICCID is skipped.
        let mut modem = QuectelModem::unisoc("RG255AA".to_string());
        let mut responses: Vec<&str> = vec!["+CPIN: SIM not inserted", "865123456789011"];
        responses.extend_from_slice(NO_SIM_SUFFIX);
        let mut transport = MockTransport::new(responses);

        let status = modem.query_modem_status(&mut transport).expect("ok");

        assert_eq!(status.sim_status, "SIM not inserted");
        assert!(!transport.sent.iter().any(|c| c == "AT+CCID"));
        assert_eq!(status.imei, "865123456789011");
        assert_eq!(status.iccid, "");
    }

    #[test]
    fn query_modem_status_continues_when_iccid_fails_on_ready() {
        // SIM READY but AT+CCID returns +CME ERROR (e.g. SIM rejected / not
        // provisioned yet): ICCID failure must NOT abort the whole status
        // query — IMEI/operator/registration are still returned.
        let mut modem = QuectelModem::unisoc("RM500U".to_string());
        let mut responses: Vec<&str> = READY_PREFIX.to_vec();
        responses.push("+CME ERROR: 10");
        responses.extend_from_slice(READY_SUFFIX);
        let mut transport = MockTransport::new(responses);

        let status = modem.query_modem_status(&mut transport).expect("ok");

        assert_eq!(status.sim_status, "READY");
        assert_eq!(status.imei, "865123456789011");
        assert_eq!(status.iccid, "");
    }

    #[test]
    fn query_feature_toggles_errors_when_qcfg_parse_fails() {
        let mut modem = QuectelModem::qualcomm("RM500Q".to_string());
        let mut transport = MockTransport::new(vec![
            "+QCFG: \"pcie/mode\",oops\nOK",
        ]);

        let err = modem
            .query_feature_toggles(&mut transport)
            .expect_err("invalid live toggle response must not become false");

        assert!(err.contains("pcie/mode"), "unexpected error: {err}");
    }

    #[test]
    fn query_feature_toggles_errors_when_live_read_fails() {
        let mut modem = QuectelModem::qualcomm("RM500Q".to_string());
        let mut transport = MockTransport::new(vec!["ERROR"]);

        let err = modem
            .query_feature_toggles(&mut transport)
            .expect_err("live toggle query must not fall back to defaults");

        assert!(
            err.contains("pcie/mode") || err.contains("AT command failed"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn query_nat_mode_errors_when_qcfg_parse_fails() {
        let mut modem = QuectelModem::qualcomm("RM500Q".to_string());
        let mut transport = MockTransport::new(vec![
            "+QCFG: \"nat\",oops\nOK",
        ]);

        let err = modem
            .query_nat_mode(&mut transport)
            .expect_err("invalid NAT response must not become 0");

        assert!(err.contains("nat"), "unexpected error: {err}");
    }
}
