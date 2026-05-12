use crate::at_parser::*;
use modem_hal::transport::AtTransport;
use modem_hal::types::*;

/// Small delay between AT commands to let the modem process
fn cmd_delay() {
    std::thread::sleep(std::time::Duration::from_millis(2));
}

/// Query full modem status: SIM, registration, signal, cell info
/// Also returns active CIDs from CGACT for reuse by query_apn_list.
pub fn query_modem_status(t: &mut dyn AtTransport) -> Result<(ModemStatus, Vec<i32>), String> {
    log::info!("query_modem_status: starting queries...");

    // SIM status
    let cpin_resp = t.send_at("AT+CPIN?")?;
    log::info!("AT+CPIN? => {}", cpin_resp);
    let sim_status = parse_cpin(&cpin_resp);

    cmd_delay();

    // IMEI
    let cgsn_resp = match t.send_at("AT+CGSN") {
        Ok(r) => {
            log::info!("AT+CGSN => {}", r);
            r
        }
        Err(e) => {
            log::warn!("AT+CGSN failed: {}", e);
            String::new()
        }
    };
    let imei = parse_cgsn(&cgsn_resp);

    cmd_delay();

    // ICCID (try CCID first, fallback to QCCID)
    let ccid_resp = match t.send_at("AT+CCID") {
        Ok(r) => {
            log::info!("AT+CCID => {}", r);
            r
        }
        Err(e) => {
            log::warn!("AT+CCID failed: {}", e);
            String::new()
        }
    };
    let iccid = if !parse_ccid(&ccid_resp).is_empty() {
        parse_ccid(&ccid_resp)
    } else {
        // Fallback to QCCID
        match t.send_at("AT+QCCID") {
            Ok(r) => {
                log::info!("AT+QCCID => {}", r);
                parse_ccid(&r)
            }
            Err(e) => {
                log::warn!("AT+QCCID failed: {}", e);
                String::new()
            }
        }
    };

    cmd_delay();

    // Serving cell info (covers: reg status, operator, cell, signal, tx power)
    let mut reg_status = "未注册".to_string();
    let mut operator = String::new();
    let mut network_type = String::new();
    let mut pci = String::new();
    let mut cell_id = String::new();
    let mut arfcn = String::new();
    let mut bandwidth = String::new();
    let mut rsrp = String::new();
    let mut rsrq = String::new();
    let mut sinr = String::new();
    let mut tx_power = String::new();
    let mut scs = String::new();

    match t.send_at("AT+QENG=\"servingcell\"") {
        Ok(qeng_resp) => {
            log::info!("AT+QENG=\"servingcell\" => {}", qeng_resp);
            if let Some(info) = parse_qeng_servingcell(&qeng_resp) {
                reg_status = if info.connected {
                    "已注册".to_string()
                } else {
                    "未注册".to_string()
                };
                operator = format!("{}{}", info.operator_mcc, info.operator_mnc);
                network_type = info.tech;
                pci = info.pci;
                cell_id = info.cell_id;
                arfcn = info.arfcn;
                bandwidth = info.bandwidth;
                rsrp = info.rsrp;
                rsrq = info.rsrq;
                sinr = info.sinr;
                tx_power = info.tx_power;
                scs = info.scs;
            } else {
                log::warn!("Failed to parse QENG servingcell response");
            }
        }
        Err(e) => log::warn!("AT+QENG=\"servingcell\" failed: {}", e),
    }

    cmd_delay();

    // Also try COPS for operator name
    match t.send_at("AT+COPS?") {
        Ok(cops_resp) => {
            log::info!("AT+COPS? => {}", cops_resp);
            let (op_name, _) = parse_cops(&cops_resp);
            if !op_name.is_empty() {
                operator = op_name;
            }
        }
        Err(e) => log::warn!("AT+COPS? failed: {}", e),
    }

    cmd_delay();

    // ANT from AT+QANTRSSI?
    let mut ant_values = vec![String::new(), String::new(), String::new(), String::new()];
    match t.send_at("AT+QANTRSSI?") {
        Ok(antrssi_resp) => {
            log::info!("AT+QANTRSSI? => {}", antrssi_resp);
            let ant = parse_qantrssi(&antrssi_resp);
            let has_data = ant.iter().any(|v| !v.is_empty());
            if has_data {
                ant_values = ant;
            }
        }
        Err(e) => log::warn!("AT+QANTRSSI? failed: {}", e),
    }

    cmd_delay();

    // Connection status from CGACT (also collect active CIDs for reuse)
    let mut conn_status = "未连接".to_string();
    let mut active_cids: Vec<i32> = Vec::new();
    match t.send_at("AT+CGACT?") {
        Ok(cgact_resp) => {
            log::info!("AT+CGACT? => {}", cgact_resp);
            let contexts = parse_cgact(&cgact_resp);
            active_cids = contexts.iter()
                .filter(|(_, s)| *s == 1)
                .map(|(cid, _)| *cid)
                .collect();
            if !active_cids.is_empty() {
                conn_status = "已连接".to_string();
            }
        }
        Err(e) => log::warn!("AT+CGACT? failed: {}", e),
    }

    let result = ModemStatus {
        sim_status,
        reg_status,
        conn_status,
        imei,
        iccid,
        operator,
        network_type,
        pci,
        cell_id,
        arfcn,
        bandwidth,
        rsrp,
        rsrq,
        sinr,
        tx_power,
        ant_values,
        scs,
    };
    log::info!(
        "query_modem_status: done - SIM={}, REG={}, CONN={}, NET={}, OP={}",
        result.sim_status,
        result.reg_status,
        result.conn_status,
        result.network_type,
        result.operator
    );
    Ok((result, active_cids))
}

/// Query hardware info: model, manufacturer, firmware, baseline, temperature
pub fn query_hardware_info(t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
    let ati_resp = match t.send_at("ATI") {
        Ok(r) => { log::info!("ATI => {}", r); r }
        Err(e) => { log::warn!("ATI failed: {}", e); String::new() }
    };
    let (manufacturer, model, firmware) = if !ati_resp.is_empty() {
        parse_ati(&ati_resp)
    } else {
        // Fallback to individual commands if ATI fails
        let cgmm_resp = t.send_at("AT+CGMM").unwrap_or_default();
        let cgmi_resp = t.send_at("AT+CGMI").unwrap_or_default();
        let gmr_resp = t.send_at("AT+GMR").unwrap_or_default();
        (parse_cgmi(&cgmi_resp), parse_cgmm(&cgmm_resp), parse_gmr(&gmr_resp))
    };

    cmd_delay();

    // AP/CP baseline
    let (ap_baseline, cp_baseline) = match t.send_at("AT+QBASELINE") {
        Ok(resp) => {
            log::info!("AT+QBASELINE => {}", resp);
            parse_qbaseline(&resp)
        }
        Err(e) => {
            log::warn!("AT+QBASELINE failed: {}", e);
            (String::new(), String::new())
        }
    };

    cmd_delay();

    // Temperature
    let (soc_temp, pa_temp) = match t.send_at("AT+QTEMP") {
        Ok(resp) => {
            log::info!("AT+QTEMP => {}", resp);
            parse_qtemp(&resp)
        }
        Err(e) => {
            log::warn!("AT+QTEMP failed: {}", e);
            (String::new(), String::new())
        }
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

/// Query IP info via AT+QNETDEVSTATUS
pub fn query_ip_info(t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
    let cmd = format!("AT+QNETDEVSTATUS={}", cid);
    let resp = t.send_at(&cmd)?;
    let (ipv4, mask, gw, dns, ipv6) = parse_qnetdevstatus(&resp);
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

/// Query APN list via AT+CGDCONT? (standard 3GPP, works on Qualcomm & Unisoc)
/// If `active_cids` is provided, skip AT+CGACT? query.
pub fn query_apn_list(t: &mut dyn AtTransport, active_cids: Option<Vec<i32>>) -> Result<Vec<ApnEntry>, String> {
    let active_cids = match active_cids {
        Some(cids) => cids,
        None => {
            let cgact_resp = t.send_at("AT+CGACT?")?;
            parse_cgact(&cgact_resp)
                .into_iter()
                .filter(|(_, status)| *status == 1)
                .map(|(cid, _)| cid)
                .collect()
        }
    };

    let resp = t.send_at("AT+CGDCONT?")?;
    Ok(parse_cgdcont(&resp, &active_cids))
}

/// Query neighbor cells via AT+QENG="neighbourcell"
/// Returns (lte_cells, nr_cells).
pub fn query_neighbor_cells(
    t: &mut dyn AtTransport,
) -> Result<(Vec<NeighborCell>, Vec<NeighborCell>), String> {
    log::info!("AT+QENG=\"neighbourcell\" => ...");
    let resp = t.send_at(r#"AT+QENG="neighbourcell""#)?;
    log::info!("AT+QENG=\"neighbourcell\" <= {}", resp);
    Ok(parse_qeng_neighbourcell(&resp))
}

/// Query QoS info via AT+C5GQOSRDP=<cid>
pub fn query_qos(t: &mut dyn AtTransport, cid: i32) -> Result<QosInfo, String> {
    let cmd = format!("AT+C5GQOSRDP={}", cid);
    log::info!("{} => ...", cmd);
    let resp = t.send_at(&cmd)?;
    log::info!("{} <= {}", cmd, resp);
    let (cqi, ul_bw, dl_bw) = parse_c5gqosrdp(&resp);
    Ok(QosInfo {
        cqi,
        ul_bandwidth: ul_bw,
        dl_bandwidth: dl_bw,
    })
}

/// Set APN via AT+CGDCONT (standard 3GPP, works on Qualcomm & Unisoc)
/// `context_type`: 1=IPv4, 2=IPv6, 3=IPv4v6
/// `username`/`password`/`auth_type` are accepted for API compat but not sent (CGDCONT has no auth fields)
pub fn set_apn(
    t: &mut dyn AtTransport,
    cid: i32,
    context_type: i32,
    apn: &str,
    _username: &str,
    _password: &str,
    _auth_type: i32,
) -> Result<(), String> {
    let pdp_type = match context_type {
        1 => "IP",
        2 => "IPV6",
        3 => "IPV4V6",
        _ => "IPV4V6",
    };
    let cmd = format!("AT+CGDCONT={},\"{}\",\"{}\"", cid, pdp_type, apn);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to set APN: {}", resp))
    }
}

/// Delete APN via AT+CGDCONT=<cid>
pub fn delete_apn(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let cmd = format!("AT+CGDCONT={}", cid);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to delete APN: {}", resp))
    }
}

/// Activate data connection via AT+QNETDEVCTL=<cid>,3,<flag>
/// op=3 means connect, flag=1 disables auto-connect on next boot
pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let cmd = format!("AT+QNETDEVCTL={},3,1", cid);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to connect: {}", resp))
    }
}

/// Deactivate data connection via AT+QNETDEVCTL=<cid>,2,0
/// op=2 means disconnect
pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let cmd = format!("AT+QNETDEVCTL={},2,0", cid);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to disconnect: {}", resp))
    }
}

/// Set preferred network mode via AT+QNWPREFCFG="mode_pref",<mode>
pub fn set_network_mode(t: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
    let cmd = format!("AT+QNWPREFCFG=\"mode_pref\",\"{}\"", mode);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to set network mode: {}", resp))
    }
}

/// Query preferred network mode
pub fn query_network_mode(t: &mut dyn AtTransport) -> Result<String, String> {
    let resp = t.send_at("AT+QNWPREFCFG=\"mode_pref\"")?;
    Ok(parse_qnwprefcfg_mode(&resp))
}

/// Set NR5G band via AT+QNWPREFCFG="nr5g_band",<band>
pub fn set_nr5g_band(t: &mut dyn AtTransport, band: &str) -> Result<(), String> {
    let cmd = format!("AT+QNWPREFCFG=\"nr5g_band\",\"{}\"", band);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to set NR5G band: {}", resp))
    }
}

/// Look up the hardware-supported bands for a given modem model.
/// Returns (lte_bands, nr_bands) as display names (e.g. "B1", "n78").
fn supported_bands_for_model(model: &str) -> (Vec<String>, Vec<String>) {
    let m = model.to_uppercase();

    // RM520N-CN — Qualcomm SDX62 / 5G Sub-6
    if m.contains("RM520N") {
        return (
            vec!["B1","B3","B5","B8","B34","B38","B39","B40","B41"]
                .into_iter().map(String::from).collect(),
            vec!["n1","n3","n5","n8","n28","n41","n78","n79"]
                .into_iter().map(String::from).collect(),
        );
    }

    // Fallback: empty — caller will query the modem directly
    (Vec::new(), Vec::new())
}

/// Query all band configuration: supported bands and currently locked bands
pub fn query_bands(t: &mut dyn AtTransport) -> Result<BandConfig, String> {
    let (mut lte_supported, mut nr_supported) = (Vec::new(), Vec::new());

    // Detect model via ATI and look up known band tables
    match t.send_at("ATI") {
        Ok(resp) => {
            log::info!("ATI => {}", resp);
            let (_, model, _) = parse_ati(&resp);
            if !model.is_empty() {
                (lte_supported, nr_supported) = supported_bands_for_model(&model);
                if !lte_supported.is_empty() || !nr_supported.is_empty() {
                    log::info!("Using hardcoded band table for model: {}", model);
                }
            }
        }
        Err(e) => log::warn!("ATI failed: {}", e),
    }

    // Fallback: query modem for its supported band list
    if lte_supported.is_empty() && nr_supported.is_empty() {
        log::info!("No hardcoded band table — querying AT+QNWPREFCFG=?");
        match t.send_at("AT+QNWPREFCFG=?") {
            Ok(r) => {
                log::info!("AT+QNWPREFCFG=? => {}", r);
                let parsed = parse_qnwprefcfg_supported(&r);
                lte_supported = parsed.0;
                nr_supported = parsed.1;
            }
            Err(e) => log::warn!("AT+QNWPREFCFG=? failed: {}", e),
        }
    }

    cmd_delay();

    // Query current locked LTE bands
    let lte_locked_resp = match t.send_at(r#"AT+QNWPREFCFG="lte_band""#) {
        Ok(r) => {
            log::info!("AT+QNWPREFCFG=\"lte_band\" => {}", r);
            r
        }
        Err(e) => {
            log::warn!("AT+QNWPREFCFG=\"lte_band\" failed: {}", e);
            String::new()
        }
    };
    let lte_locked = parse_qnwprefcfg_bands(&lte_locked_resp, "lte_band");

    cmd_delay();

    // Query current locked NR5G bands
    let nr_locked_resp = match t.send_at(r#"AT+QNWPREFCFG="nr5g_band""#) {
        Ok(r) => {
            log::info!("AT+QNWPREFCFG=\"nr5g_band\" => {}", r);
            r
        }
        Err(e) => {
            log::warn!("AT+QNWPREFCFG=\"nr5g_band\" failed: {}", e);
            String::new()
        }
    };
    let nr_locked = parse_qnwprefcfg_bands(&nr_locked_resp, "nr5g_band");

    Ok(BandConfig {
        lte_supported,
        nr_supported,
        lte_locked,
        nr_locked,
    })
}

/// Reset all bands via AT+QNWPREFCFG="all_band_reset"
pub fn reset_all_bands(t: &mut dyn AtTransport) -> Result<(), String> {
    let resp = t.send_at(r#"AT+QNWPREFCFG="all_band_reset""#)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to reset bands: {}", resp))
    }
}

/// Set bands via AT+QNWPREFCFG
/// `lte` and `nr` are colon-separated band numbers, e.g. "1:3:5" or "1:3:5:77:78:79"
pub fn set_bands(t: &mut dyn AtTransport, lte: &str, nr: &str) -> Result<(), String> {
    if !lte.is_empty() {
        let cmd = format!(r#"AT+QNWPREFCFG="lte_band","{}""#, lte);
        let resp = t.send_at(&cmd)?;
        if !is_ok(&resp) {
            return Err(format!("Failed to set LTE bands: {}", resp));
        }
        cmd_delay();
    }

    if !nr.is_empty() {
        let cmd = format!(r#"AT+QNWPREFCFG="nr5g_band","{}""#, nr);
        let resp = t.send_at(&cmd)?;
        if !is_ok(&resp) {
            return Err(format!("Failed to set NR5G bands: {}", resp));
        }
    }

    Ok(())
}

/// Reboot modem via AT+CRESET
pub fn reboot_modem(t: &mut dyn AtTransport) -> Result<(), String> {
    let resp = t.send_at("AT+CRESET")?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to reboot: {}", resp))
    }
}

/// Factory reset via AT+QFACT
pub fn factory_reset(t: &mut dyn AtTransport) -> Result<(), String> {
    let resp = t.send_at("AT+QFACT=0")?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to factory reset: {}", resp))
    }
}

/// Send raw AT command and return the response
pub fn send_raw_at(t: &mut dyn AtTransport, command: &str) -> Result<String, String> {
    t.send_at(command)
}

/// Query all feature toggles from modem
pub fn query_feature_toggles(t: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
    let pcie_mode = match t.send_at(r#"AT+QCFG="pcie/mode""#) {
        Ok(r) => parse_qcfg_int(&r, "pcie/mode").unwrap_or(0) == 1,
        Err(e) => {
            log::warn!("AT+QCFG pcie/mode failed: {}", e);
            false
        }
    };
    cmd_delay();

    let ethernet = match t.send_at(r#"AT+QCFG="ethernet""#) {
        Ok(r) => parse_qcfg_int(&r, "ethernet").unwrap_or(0) == 1,
        Err(e) => {
            log::warn!("AT+QCFG ethernet failed: {}", e);
            false
        }
    };
    cmd_delay();

    let proxyarp = match t.send_at(r#"AT+QCFG="proxyarp""#) {
        Ok(r) => parse_qcfg_int(&r, "proxyarp").unwrap_or(0) == 1,
        Err(e) => {
            log::warn!("AT+QCFG proxyarp failed: {}", e);
            false
        }
    };
    cmd_delay();

    let uartat = match t.send_at(r#"AT+QCFG="uartat""#) {
        Ok(r) => parse_qcfg_int(&r, "uartat").unwrap_or(0) == 1,
        Err(e) => {
            log::warn!("AT+QCFG uartat failed: {}", e);
            false
        }
    };
    cmd_delay();

    let eth_at = match t.send_at(r#"AT+QCFG="eth_at""#) {
        Ok(r) => parse_qcfg_int(&r, "eth_at").unwrap_or(0) == 1,
        Err(e) => {
            log::warn!("AT+QCFG eth_at failed: {}", e);
            false
        }
    };
    cmd_delay();

    let adb = match t.send_at(r#"AT+QCFG="usbcfg""#) {
        Ok(r) => parse_qcfg_usbcfg_adb(&r),
        Err(e) => {
            log::warn!("AT+QCFG usbcfg failed: {}", e);
            false
        }
    };

    Ok(FeatureToggles {
        pcie_mode,
        ethernet,
        proxyarp,
        uartat,
        eth_at,
        adb,
    })
}

/// Set a single AT+QCFG integer toggle
pub fn set_qcfg_toggle(t: &mut dyn AtTransport, key: &str, value: i32) -> Result<(), String> {
    let cmd = format!(r#"AT+QCFG="{}",{}"#, key, value);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to set {}: {}", key, resp))
    }
}

/// Set ADB via AT+QCFG="usbcfg" — need to preserve other params, only change last one
pub fn set_adb(t: &mut dyn AtTransport, enabled: bool) -> Result<(), String> {
    // First read current usbcfg
    let resp = t.send_at(r#"AT+QCFG="usbcfg""#)?;

    // Parse existing params
    for line in extract_data_lines(&resp) {
        if let Some(rest) = line.strip_prefix("+QCFG: \"usbcfg\",") {
            let mut parts: Vec<&str> = rest.split(',').collect();
            if !parts.is_empty() {
                let last = parts.len() - 1;
                parts[last] = if enabled { "1" } else { "0" };
                let cmd = format!(r#"AT+QCFG="usbcfg",{}"#, parts.join(","));
                let resp = t.send_at(&cmd)?;
                if is_ok(&resp) {
                    return Ok(());
                } else {
                    return Err(format!("Failed to set usbcfg: {}", resp));
                }
            }
        }
    }
    Err("Could not parse current usbcfg".to_string())
}

/// Query USB net mode via AT+QCFG="usbnet"
pub fn query_usbnet(t: &mut dyn AtTransport) -> Result<i32, String> {
    let resp = t.send_at(r#"AT+QCFG="usbnet""#)?;
    parse_qcfg_usbnet(&resp).ok_or_else(|| format!("Failed to parse usbnet: {}", resp))
}

/// Set USB net mode via AT+QCFG="usbnet",<mode>
pub fn set_usbnet(t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
    let cmd = format!(r#"AT+QCFG="usbnet",{}"#, mode);
    let resp = t.send_at(&cmd)?;
    if is_ok(&resp) {
        Ok(())
    } else {
        Err(format!("Failed to set usbnet: {}", resp))
    }
}

/// Query traffic statistics via AT+QGDCNT?
pub fn query_traffic(t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
    log::info!("AT+QGDCNT? => ...");
    let resp = t.send_at("AT+QGDCNT?")?;
    log::info!("AT+QGDCNT? <= {}", resp);
    let (ul, dl) = parse_qgdcnt(&resp);
    Ok(TrafficInfo {
        ul_bytes: ul,
        dl_bytes: dl,
    })
}
