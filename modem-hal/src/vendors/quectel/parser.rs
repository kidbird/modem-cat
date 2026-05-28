use crate::types::{ApnEntry, L5GanEntry, NeighborCell, NeighborCells, ServingCellInfo, TemperatureInfo};

pub fn is_ok(response: &str) -> bool {
    let trimmed = response.trim();
    trimmed.ends_with("OK") || trimmed.contains("OK\n") || trimmed.contains("OK\r\n")
}

pub fn extract_data_lines(response: &str) -> Vec<String> {
    response
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty()
                && t != "OK"
                && !t.starts_with("ERROR")
                && !t.starts_with("+CME ERROR")
                && !t.starts_with("AT+")
                && !t.starts_with("AT^")
                && t != "AT"
        })
        .map(|l| l.trim().to_string())
        .collect()
}

pub fn parse_cpin(response: &str) -> String {
    for line in response.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("+CPIN: ") {
            return rest.trim().to_string();
        }
        if trimmed.starts_with("+CME ERROR") || trimmed.starts_with("ERROR") {
            return "NO SIM".to_string();
        }
    }
    "UNKNOWN".to_string()
}

pub fn parse_cgsn(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') {
            continue;
        }
        if line.chars().all(|c| c.is_ascii_digit()) && line.len() >= 14 {
            return line;
        }
    }
    String::new()
}

pub fn parse_iccid(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line
            .strip_prefix("+CCID: ")
            .or_else(|| line.strip_prefix("+ICCID: "))
            .or_else(|| line.strip_prefix("+QCCID: "))
        {
            return rest.trim().to_string();
        }
    }
    String::new()
}

pub fn parse_cgmm(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') && !line.starts_with("+CGMM:") {
            continue;
        }
        if line.starts_with("+CGMM:") {
            if let Some(rest) = line.strip_prefix("+CGMM:") {
                return rest.trim().trim_matches('"').to_string();
            }
        }
        return line.trim().to_string();
    }
    String::new()
}

pub fn parse_cops(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+COPS:") {
            let parts: Vec<&str> = rest.split(',').collect();
            if parts.len() >= 3 {
                return parts[2].trim().trim_matches('"').to_string();
            }
        }
    }
    String::new()
}

pub fn format_rsrp(val: &str) -> String {
    if val.is_empty() || val == "0" {
        return String::new();
    }
    if let Ok(v) = val.parse::<i32>() {
        if v < 0 {
            format!("{} dBm", v)
        } else {
            format!("-{} dBm", v)
        }
    } else {
        val.to_string()
    }
}

pub fn format_rsrq(val: &str) -> String {
    if val.is_empty() || val == "0" {
        return String::new();
    }
    if let Ok(v) = val.parse::<i32>() {
        if v < 0 {
            format!("{} dB", v)
        } else {
            format!("-{} dB", v)
        }
    } else {
        val.to_string()
    }
}

fn format_bw(val: &str) -> String {
    if val.is_empty() || val == "0" {
        return String::new();
    }
    if let Ok(v) = val.parse::<u32>() {
        format!("{} MHz", v)
    } else {
        val.to_string()
    }
}

fn format_bandwidth_bps(val: &str) -> String {
    if let Ok(v) = val.parse::<u64>() {
        let mbps = v as f64 / 1_000.0;
        if mbps >= 1_000.0 {
            format!("{:.1} Gbps", mbps / 1_000.0)
        } else {
            format!("{:.0} Mbps", mbps)
        }
    } else {
        val.to_string()
    }
}

pub fn decode_qualcomm_bandwidth(idx: u32) -> String {
    match idx {
        0 => "1.4".to_string(),
        1 => "3".to_string(),
        2 => "5".to_string(),
        3 => "10".to_string(),
        4 => "15".to_string(),
        5 => "20".to_string(),
        6 => "100".to_string(),
        n => n.to_string(),
    }
}

fn filter_tx_power(val: &str) -> String {
    let v = val.trim();
    match v.parse::<i32>() {
        Ok(n) if n <= -32760 || n >= 32760 => String::new(),
        Ok(n) => format!("{} dBm", n),
        Err(_) => String::new(),
    }
}

fn filter_rx_level(val: &str) -> String {
    let v = val.trim();
    match v.parse::<i32>() {
        Ok(n) if n >= 255 || n < 0 => String::new(),
        Ok(n) if n == 0 => String::new(),
        Ok(n) => n.to_string(),
        Err(_) => String::new(),
    }
}

fn format_sinr(val: &str) -> String {
    let v = val.trim();
    if v.is_empty() || v == "0" {
        return String::new();
    }
    if let Ok(n) = v.parse::<i32>() {
        format!("{} dB", n)
    } else {
        v.to_string()
    }
}

pub fn parse_qeng_serving_cell(response: &str, qualcomm_bandwidth: bool) -> ServingCellInfo {
    for line in extract_data_lines(response) {
        if !line.starts_with("+QENG:") {
            continue;
        }
        let data = line.strip_prefix("+QENG:").unwrap().trim();
        let parts: Vec<&str> = data.splitn(26, ',').collect();
        if parts.len() < 3 {
            // Short response like: +QENG: "servingcell","SEARCH" or +QENG: "servingcell","CONNECT"
            let state = parts.get(1).map(|s| s.trim().trim_matches('"')).unwrap_or("");
            if !state.is_empty() {
                log::warn!(
                    "QENG short response: state={}, parts_len={}, line={}",
                    state, parts.len(), line
                );
                return ServingCellInfo {
                    mobility_state: state.to_string(),
                    ..Default::default()
                };
            }
            continue;
        }

        let state = parts[1].trim().trim_matches('"');
        let tech = parts[2].trim().trim_matches('"');
        let connected = state == "CONNECT";

        match tech {
            // NR5G-SA format (Quectel RM520N / RM500Q Qualcomm):
            // [0]="servingcell" [1]=state [2]="NR5G-SA" [3]=duplex [4]=MCC [5]=MNC
            // [6]=cellID [7]=PCI [8]=TAC [9]=NR-ARFCN [10]=band [11]=BW(MHz)
            // [12]=RSRP [13]=RSRQ [14]=SINR [15]=TxPwr [16]=RxLev [17]=SCS
            "NR5G-SA" if parts.len() >= 15 => {
                return ServingCellInfo {
                    connected,
                    mobility_state: state.to_string(),
                    tech: tech.to_string(),
                    operator_mcc: parts[4].trim().trim_matches('"').to_string(),
                    operator_mnc: parts[5].trim().trim_matches('"').to_string(),
                    cell_id: parts[6].trim().to_string(),
                    pci: parts[7].trim().to_string(),
                    arfcn: parts[9].trim().to_string(),
                    band: parts[10].trim().to_string(),
                    bandwidth: format_bw(parts[11].trim()),
                    rsrp: format_rsrp(parts[12].trim()),
                    rsrq: format_rsrq(parts[13].trim()),
                    sinr: format_sinr(parts[14].trim()),
                    tx_power: parts.get(15).map_or(String::new(), |v| filter_tx_power(v)),
                    rx_level: parts.get(16).map_or(String::new(), |v| filter_rx_level(v)),
                    scs: parts.get(17).map_or(String::new(), |v| v.trim().to_string()),
                };
            }
            // LTE format (Quectel Qualcomm / UniSoc):
            // [0]="servingcell" [1]=state [2]="LTE" [3]=duplex [4]=MCC [5]=MNC
            // [6]=cellID [7]=PCI [8]=EARFCN [9]=band [10]=DL-BW [11]=UL-BW
            // [12]=TAC [13]=RSRP [14]=RSRQ [15]=RSSI [16]=SINR [17]=CQI [18]=TxPwr [19]=srxlev
            "LTE" if parts.len() >= 17 => {
                let bw_raw = parts.get(10).unwrap_or(&"").trim();
                let bandwidth = if qualcomm_bandwidth {
                    let bw_str = decode_qualcomm_bandwidth(bw_raw.parse::<u32>().unwrap_or(0));
                    if bw_str.is_empty() { String::new() } else { format!("{} MHz", bw_str) }
                } else {
                    format_bw(bw_raw)
                };
                return ServingCellInfo {
                    connected,
                    mobility_state: state.to_string(),
                    tech: tech.to_string(),
                    operator_mcc: parts[4].trim().trim_matches('"').to_string(),
                    operator_mnc: parts[5].trim().trim_matches('"').to_string(),
                    cell_id: parts[6].trim().to_string(),
                    pci: parts[7].trim().to_string(),
                    arfcn: parts[8].trim().to_string(),
                    band: parts[9].trim().to_string(),
                    bandwidth,
                    rsrp: format_rsrp(parts[13].trim()),
                    rsrq: format_rsrq(parts[14].trim()),
                    sinr: format_sinr(parts[16].trim()),
                    tx_power: parts.get(18).map_or(String::new(), |v| filter_tx_power(v)),
                    rx_level: String::new(),
                    scs: String::new(),
                };
            }
            // NR5G-NSA format (dual connectivity: LTE anchor + NR secondary):
            // [0..11] = LTE anchor fields (same layout as LTE)
            // [12]=LTE-TAC [13]=LTE-RSRP [14]=LTE-RSRQ [15]=LTE-RSSI [16]=LTE-SINR
            // [17]=LTE-TxPwr [18]=LTE-RxLev [19]=NR-ARFCN [20]=NR-band [21]=NR-BW
            // [22]=NR-RSRP [23]=NR-RSRQ [24]=NR-SINR
            "NR5G-NSA" if parts.len() >= 20 => {
                let bw_raw = parts.get(10).unwrap_or(&"").trim();
                let _lte_bw = if qualcomm_bandwidth {
                    decode_qualcomm_bandwidth(bw_raw.parse::<u32>().unwrap_or(0))
                } else {
                    bw_raw.to_string()
                };
                return ServingCellInfo {
                    connected,
                    mobility_state: state.to_string(),
                    tech: tech.to_string(),
                    operator_mcc: parts[4].trim().trim_matches('"').to_string(),
                    operator_mnc: parts[5].trim().trim_matches('"').to_string(),
                    cell_id: parts[6].trim().to_string(),
                    pci: parts[7].trim().to_string(),
                    arfcn: parts[19].trim().to_string(),
                    band: parts[20].trim().to_string(),
                    bandwidth: format_bw(parts[21].trim()),
                    rsrp: format_rsrp(parts.get(22).unwrap_or(&"").trim()),
                    rsrq: format_rsrq(parts.get(23).unwrap_or(&"").trim()),
                    sinr: format_sinr(parts.get(24).unwrap_or(&"").trim()),
                    tx_power: parts.get(17).map_or(String::new(), |v| filter_tx_power(v)),
                    rx_level: String::new(),
                    scs: String::new(),
                };
            }
            // NR5G-NSA fallback: fewer fields, use LTE anchor data
            "NR5G-NSA" if parts.len() >= 15 => {
                return ServingCellInfo {
                    connected,
                    mobility_state: state.to_string(),
                    tech: tech.to_string(),
                    operator_mcc: parts[4].trim().trim_matches('"').to_string(),
                    operator_mnc: parts[5].trim().trim_matches('"').to_string(),
                    cell_id: parts[6].trim().to_string(),
                    pci: parts[7].trim().to_string(),
                    arfcn: parts[8].trim().to_string(),
                    band: parts[9].trim().to_string(),
                    bandwidth: format_bw(parts[11].trim()),
                    rsrp: format_rsrp(parts[13].trim()),
                    rsrq: format_rsrq(parts[14].trim()),
                    sinr: String::new(),
                    tx_power: String::new(),
                    rx_level: String::new(),
                    scs: String::new(),
                };
            }
            _ => {
                log::warn!(
                    "QENG unmatched tech={}, parts_len={}, line={}",
                    tech, parts.len(), line
                );
            }
        }
    }
    // No +QENG: line found — check if response contains ERROR
    let trimmed = response.trim();
    if trimmed.contains("ERROR") || trimmed.contains("error") {
        log::warn!("QENG command returned error: {}", trimmed);
    } else {
        log::warn!("QENG no serving cell data found in response: {}", trimmed.replace('\n', "\\n").replace('\r', ""));
    }
    ServingCellInfo::default()
}

pub fn parse_qeng_neighbour_cells(response: &str) -> NeighborCells {
    let mut lte_cells = Vec::new();
    let mut nr_cells = Vec::new();

    for line in extract_data_lines(response) {
        if !line.starts_with("+QENG: \"neighbourcell") {
            continue;
        }

        let rest = line.strip_prefix("+QENG: ").unwrap_or("");
        let bytes = rest.as_bytes();
        let mut pos = 0;

        if pos >= bytes.len() || bytes[pos] != b'"' {
            continue;
        }
        pos += 1;

        let _start = pos;
        while pos < bytes.len() && bytes[pos] != b'"' {
            pos += 1;
        }
        if pos >= bytes.len() {
            continue;
        }
        pos += 1;

        if pos < bytes.len() && bytes[pos] == b',' {
            pos += 1;
        }

        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        if pos >= bytes.len() || bytes[pos] != b'"' {
            continue;
        }
        pos += 1;
        let rat_start = pos;
        while pos < bytes.len() && bytes[pos] != b'"' {
            pos += 1;
        }
        if pos >= bytes.len() {
            continue;
        }
        let rat = std::str::from_utf8(&bytes[rat_start..pos])
            .unwrap_or("")
            .trim();
        pos += 1;

        let remaining = &rest[pos..];
        let remaining = remaining.strip_prefix(',').unwrap_or(remaining);
        let parts: Vec<&str> = remaining.split(',').map(|s| s.trim()).collect();

        match rat {
            "NR" => {
                if parts.len() >= 5 {
                    nr_cells.push(NeighborCell {
                        cell_id: String::new(),
                        pci: parts.get(1).unwrap_or(&"").to_string(),
                        rsrp: parts.get(2).unwrap_or(&"").to_string(),
                        rsrq: parts.get(3).unwrap_or(&"").to_string(),
                        sinr: parts.get(4).unwrap_or(&"").to_string(),
                        earfcn: parts.get(0).unwrap_or(&"").to_string(),
                        arfcn: parts.get(0).unwrap_or(&"").to_string(),
                        offset: String::new(),
                    });
                }
            }
            "LTE" => {
                if parts.len() >= 6 {
                    let earfcn = parts.get(0).unwrap_or(&"").to_string();
                    let pci = parts.get(1).unwrap_or(&"").to_string();
                    let rsrp = parts.get(2).unwrap_or(&"").to_string();
                    let rsrq = parts.get(3).unwrap_or(&"").to_string();
                    let (sinr, srxlev) = if parts.len() >= 10 {
                        (
                            parts.get(5).unwrap_or(&"").to_string(),
                            parts.get(4).unwrap_or(&"").to_string(),
                        )
                    } else {
                        (
                            parts.get(4).unwrap_or(&"").to_string(),
                            parts.get(5).unwrap_or(&"").to_string(),
                        )
                    };
                    lte_cells.push(NeighborCell {
                        cell_id: String::new(),
                        pci,
                        rsrp,
                        rsrq,
                        sinr,
                        earfcn: earfcn.clone(),
                        arfcn: earfcn,
                        offset: srxlev,
                    });
                }
            }
            "WCDMA" => {
                if parts.len() >= 6 {
                    lte_cells.push(NeighborCell {
                        cell_id: String::new(),
                        pci: parts.get(2).unwrap_or(&"").to_string(),
                        rsrp: parts.get(3).unwrap_or(&"").to_string(),
                        rsrq: parts.get(4).unwrap_or(&"").to_string(),
                        sinr: parts.get(5).unwrap_or(&"").to_string(),
                        earfcn: parts.get(0).unwrap_or(&"").to_string(),
                        arfcn: parts.get(0).unwrap_or(&"").to_string(),
                        offset: String::new(),
                    });
                }
            }
            _ => {}
        }
    }

    NeighborCells {
        lte: lte_cells,
        nr: nr_cells,
    }
}

pub fn parse_qtemp_rich(response: &str) -> (String, String) {
    // Collect all (label, raw_value, formatted_value) entries first, then pick.
    let mut entries: Vec<(String, i64, String)> = Vec::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QTEMP:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 2 {
                let label = parts[0].to_lowercase();
                let raw: i64 = parts[1].trim().parse().unwrap_or(0);
                // Convert millidegrees (e.g. 34000 → 34.0°C) if needed
                let value = if raw > 1000 {
                    format!("{:.1}°C", raw as f64 / 1000.0)
                } else {
                    format!("{}°C", parts[1].trim())
                };
                entries.push((label, raw, value));
            }
        }
    }

    // Sentinel readings to ignore (offline/unused sensors).
    let is_valid = |raw: i64| raw > -100 && !(raw == 0);

    // PA temperature: prefer "modem-lte-sub6-pa1"; fall back to first "modem-*-pa*" with a valid reading.
    let pa = entries
        .iter()
        .find(|(l, _, _)| l == "modem-lte-sub6-pa1")
        .or_else(|| {
            entries
                .iter()
                .find(|(l, raw, _)| l.starts_with("modem-") && l.contains("-pa") && is_valid(*raw))
        })
        .map(|(_, _, v)| v.clone())
        .unwrap_or_default();

    // SOC temperature: prefer "aoss-0-usr"; fall back to aoss-*/cpuss-*/mdmss-*/socsensor*.
    let soc = entries
        .iter()
        .find(|(l, _, _)| l == "aoss-0-usr")
        .or_else(|| {
            entries.iter().find(|(l, raw, _)| {
                is_valid(*raw)
                    && (l.starts_with("aoss")
                        || l.starts_with("cpuss")
                        || l.starts_with("mdmss")
                        || l.contains("socsensor")
                        || l.contains("xo-therm"))
            })
        })
        .map(|(_, _, v)| v.clone())
        .unwrap_or_default();

    (soc, pa)
}

pub fn parse_qtemp(response: &str) -> TemperatureInfo {
    let (soc, pa) = parse_qtemp_rich(response);
    TemperatureInfo {
        soc_temp: soc,
        pa_temp: pa,
    }
}

pub fn parse_cgact_cids(response: &str) -> std::collections::HashSet<i32> {
    let mut active = std::collections::HashSet::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CGACT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.get(1).map(|s| s.trim()) == Some("1") {
                if let Ok(cid) = parts.get(0).unwrap_or(&"").trim().parse::<i32>() {
                    active.insert(cid);
                }
            }
        }
    }
    active
}

pub fn parse_cgdcont_apn(
    response: &str,
    active_cids: &std::collections::HashSet<i32>,
) -> Vec<ApnEntry> {
    let mut entries = vec![];
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CGDCONT:") {
            let parts: Vec<&str> = rest
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 3 {
                let cid: i32 = parts[0].parse().unwrap_or(0);
                entries.push(ApnEntry {
                    cid,
                    ip_type: parts.get(1).unwrap_or(&"IP").to_string(),
                    apn_name: parts.get(2).unwrap_or(&"").to_string(),
                    auth_type: 0,
                    username: String::new(),
                    active: active_cids.contains(&cid),
                });
            }
        }
    }
    entries
}

pub fn parse_qicsgp(response: &str, active_cids: &[i32]) -> Vec<ApnEntry> {
    let mut entries = Vec::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QICSGP:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 3 {
                let cid = parts[0].trim().parse::<i32>().unwrap_or(0);
                let ctx_type = parts.get(1).map(|v| v.trim()).unwrap_or("1");
                let ip_type = match ctx_type {
                    "1" => "IPv4",
                    "2" => "IPv6",
                    "3" => "IPv4v6",
                    "4" => "Ethernet",
                    _ => "IPv4",
                };
                let apn_name = parts
                    .get(2)
                    .map(|v| v.trim().trim_matches('"'))
                    .unwrap_or("")
                    .to_string();
                let username = parts
                    .get(3)
                    .map(|v| v.trim().trim_matches('"'))
                    .unwrap_or("")
                    .to_string();
                let auth_type = parts
                    .get(5)
                    .and_then(|v| v.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                let active = active_cids.contains(&cid);

                entries.push(ApnEntry {
                    cid,
                    apn_name,
                    ip_type: ip_type.to_string(),
                    auth_type,
                    username,
                    active,
                });
            }
        }
    }
    entries
}

pub fn parse_band_list(response: &str) -> Vec<String> {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNWPREFCFG:") {
            let parts: Vec<&str> = rest.split(',').collect();
            if parts.len() >= 2 {
                return parts[1]
                    .trim()
                    .trim_matches('"')
                    .split(':')
                    .map(|b| b.trim_start_matches('B').to_string())
                    .collect();
            }
        }
    }
    vec![]
}

pub fn parse_qnwprefcfg_mode(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNWPREFCFG:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                return parts[1].trim().trim_matches('"').to_string();
            }
        }
    }
    String::new()
}

pub fn parse_qnwprefcfg_bands(response: &str, band_type: &str) -> Vec<String> {
    let prefix = format!("+QNWPREFCFG: \"{}\"", band_type);
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix(&prefix) {
            let mut value = rest.trim();
            if value.starts_with(',') {
                value = &value[1..];
            }
            value = value.trim().trim_matches('"');
            if value.is_empty() || value == "0" {
                return Vec::new();
            }
            let prefix_char = if band_type == "lte_band" { "B" } else { "n" };
            return value
                .split(':')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| format!("{}{}", prefix_char, s))
                .collect();
        }
    }
    Vec::new()
}

pub fn parse_qnwprefcfg_supported(response: &str) -> (Vec<String>, Vec<String>) {
    let mut lte = Vec::new();
    let mut nr = Vec::new();

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNWPREFCFG:") {
            let rest = rest.trim();
            if let Some(pos) = rest.find("\"lte_band\"") {
                let after = &rest[pos + "\"lte_band\"".len()..];
                let value = if let Some(start) = after.find('(') {
                    if let Some(end) = after.find(')') {
                        Some(&after[start + 1..end])
                    } else {
                        None
                    }
                } else if let Some(start) = after.find(',') {
                    Some(after[start + 1..].trim())
                } else {
                    None
                };
                if let Some(v) = value {
                    lte = v
                        .split(':')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| format!("B{}", s))
                        .collect();
                }
            } else if let Some(pos) = rest.find("\"nr5g_band\"") {
                let after = &rest[pos + "\"nr5g_band\"".len()..];
                let value = if let Some(start) = after.find('(') {
                    if let Some(end) = after.find(')') {
                        Some(&after[start + 1..end])
                    } else {
                        None
                    }
                } else if let Some(start) = after.find(',') {
                    Some(after[start + 1..].trim())
                } else {
                    None
                };
                if let Some(v) = value {
                    nr = v
                        .split(':')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| format!("n{}", s))
                        .collect();
                }
            }
        }
    }

    (lte, nr)
}

pub fn parse_qnetdevstatus(response: &str) -> (String, String, String, String, String) {
    let mut ipv4 = String::new();
    let mut mask = String::new();
    let mut gw = String::new();
    let mut dns = String::new();
    let ipv6 = String::new();

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNETDEVSTATUS:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
            if !parts.is_empty() {
                ipv4 = parts[0].to_string();
            }
            if parts.len() > 1 {
                mask = parts[1].to_string();
            }
            if parts.len() > 2 {
                gw = parts[2].to_string();
            }
            if parts.len() > 4 {
                dns = parts[4].to_string();
            }
            if parts.len() > 5 {
                let dns2 = parts[5].to_string();
                if !dns.is_empty() && !dns2.is_empty() {
                    dns = format!("{}, {}", dns, dns2);
                }
            }
        }
    }

    (ipv4, mask, gw, dns, ipv6)
}

pub fn parse_c5gqosrdp(response: &str) -> (String, String, String) {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+C5GQOSRDP:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
            if parts.len() >= 8 {
                let cqi = parts[1].to_string();
                let dl_bw = format_bandwidth_bps(parts[6]);
                let ul_bw = format_bandwidth_bps(parts[7]);
                return (cqi, ul_bw, dl_bw);
            }
            if parts.len() >= 2 {
                return (parts[1].to_string(), String::new(), String::new());
            }
        }
    }
    (String::new(), String::new(), String::new())
}

pub fn parse_qantrssi(response: &str) -> Vec<String> {
    let mut ant = vec![String::new(), String::new(), String::new(), String::new()];

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QANTRSSI:") {
            let vals: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
            if vals.len() >= 5 {
                for (i, v) in vals.iter().skip(1).take(4).enumerate() {
                    ant[i] = format_rsrp(v);
                }
            } else if vals.len() >= 4 {
                for (i, v) in vals.iter().take(4).enumerate() {
                    ant[i] = format_rsrp(v);
                }
            }
        }
    }

    ant
}

pub fn parse_qbaseline(response: &str) -> (String, String) {
    let mut ap = String::new();
    let mut cp = String::new();
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("AP:") {
            ap = rest.trim().to_string();
        } else if let Some(rest) = t.strip_prefix("CP:") {
            cp = rest.trim().to_string();
        }
    }
    (ap, cp)
}

pub fn parse_cops_with_act(response: &str) -> (String, String) {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+COPS:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 4 {
                let name = parts[2].trim().trim_matches('"').to_string();
                let act = match parts[3].trim() {
                    "7" => "LTE",
                    "9" => "5G NR",
                    "11" => "5G NR",
                    "2" => "WCDMA",
                    other => other,
                };
                return (name, act.to_string());
            }
            if parts.len() >= 3 {
                let name = parts[2].trim().trim_matches('"').to_string();
                return (name, String::new());
            }
        }
    }
    (String::new(), String::new())
}

pub fn parse_qcfg_int(response: &str, key: &str) -> Option<i32> {
    let prefix = format!("+QCFG: \"{}\",", key);
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix(&prefix) {
            let first = rest.trim().split(',').next().unwrap_or("").trim();
            return first.parse().ok();
        }
    }
    None
}

pub fn parse_qcfg_usbcfg_adb(response: &str) -> bool {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QCFG: \"usbcfg\",") {
            let parts: Vec<&str> = rest.split(',').collect();
            // ADB flag is the second-to-last parameter
            if parts.len() >= 2 {
                return parts[parts.len() - 2].trim() == "1";
            }
        }
    }
    false
}

pub fn parse_qcfg_usbnet(response: &str) -> Option<i32> {
    parse_qcfg_int(response, "usbnet")
}

pub fn parse_qgdcnt(response: &str) -> (u64, u64) {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QGDCNT:") {
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

pub fn parse_cereg(response: &str) -> (String, Option<String>, Option<String>) {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CEREG:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                let stat = parts[1].trim();
                let status_str = match stat {
                    "0" => "NOCONN",
                    "1" => "CONNECT",
                    "2" => "SEARCH",
                    "3" => "DENIED",
                    "4" => "UNKNOWN",
                    "5" => "CONNECT",
                    _ => stat,
                };
                let tac = parts.get(2).map(|v| v.trim().to_string());
                let ci = parts.get(3).map(|v| v.trim().to_string());
                return (status_str.to_string(), tac, ci);
            }
        }
    }
    ("NOCONN".to_string(), None, None)
}

pub fn parse_gmr(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+GMR:") {
            return rest.trim().to_string();
        }
    }
    for line in response.lines() {
        let t = line.trim();
        if t.is_empty() || t == "OK" || t.starts_with("AT+") || t.starts_with('+') {
            continue;
        }
        return t.to_string();
    }
    String::new()
}

pub fn parse_cgact(response: &str) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CGACT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                if let (Ok(cid), Ok(status)) = (
                    parts[0].trim().parse::<i32>(),
                    parts[1].trim().parse::<i32>(),
                ) {
                    result.push((cid, status));
                }
            }
        }
    }
    result
}

pub fn parse_5glan(response: &str) -> Vec<L5GanEntry> {
    let mut entries = Vec::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QCFG:") {
            let rest = rest.trim();
            if let Some(params) = rest.strip_prefix("\"5glan\",") {
                let parts: Vec<&str> = params.split(',').collect();
                if parts.len() >= 2 {
                    if let (Ok(cid), Ok(state)) = (
                        parts[0].trim().parse::<i32>(),
                        parts[1].trim().parse::<i32>(),
                    ) {
                        let vlan_id = parts
                            .get(2)
                            .and_then(|s| s.trim().parse::<i32>().ok())
                            .unwrap_or(1);
                        entries.push(L5GanEntry {
                            cid,
                            enabled: state == 1,
                            vlan_id,
                        });
                    }
                }
            }
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_qeng_lte_qualcomm_bandwidth_and_signal() {
        // Fields: [10]=DL-BW-idx=5(→20MHz) [11]=UL-BW [12]=TAC=5AE [13]=RSRP=-95 [14]=RSRQ=-10
        // [15]=RSSI=-65 [16]=SINR=18 [17]=CQI=10 [18]=TxPwr=20
        let raw = r#"+QENG: "servingcell","CONNECT","LTE","FDD",460,11,1A2B3C4,100,2650,3,5,5,5AE,-95,-10,-65,18,10,20,0"#;
        let info = parse_qeng_serving_cell(raw, true);
        assert_eq!(info.bandwidth, "20 MHz");
        assert_eq!(info.arfcn, "2650");
        assert_eq!(info.rsrp, "-95 dBm");
        assert_eq!(info.rsrq, "-10 dB");
        assert_eq!(info.sinr, "18 dB");
        assert_eq!(info.tx_power, "20 dBm");
        assert_eq!(info.tech, "LTE");
        assert!(info.connected);
    }

    #[test]
    fn parse_qeng_lte_unisoc_bandwidth_direct() {
        let raw = r#"+QENG: "servingcell","CONNECT","LTE","FDD",460,11,1A2B3C4,100,2650,3,100,5,5AE,-95,-10,-65,18,10,20,0"#;
        let info = parse_qeng_serving_cell(raw, false);
        assert_eq!(info.bandwidth, "100 MHz");
    }

    #[test]
    fn parse_qeng_nr5g_sa_full() {
        // Fields: [3]=SA [4]=MCC [5]=MNC [6]=cellID [7]=PCI [8]=TAC [9]=ARFCN
        // [10]=band [11]=BW=100MHz [12]=RSRP=-97 [13]=RSRQ=-11 [14]=SINR=21
        // [15]=TxPwr=-32767(n/a) [16]=RxLev=255(n/a) [17]=SCS=1
        let raw = r#"+QENG: "servingcell","CONNECT","NR5G-SA","SA",460,11,0B46E280,0,120,504990,41,100,-97,-11,21,-32767,255,1"#;
        let info = parse_qeng_serving_cell(raw, true);
        assert_eq!(info.tech, "NR5G-SA");
        assert!(info.connected);
        assert_eq!(info.pci, "0");
        assert_eq!(info.arfcn, "504990");
        assert_eq!(info.band, "41");
        assert_eq!(info.bandwidth, "100 MHz");
        assert_eq!(info.rsrp, "-97 dBm");
        assert_eq!(info.rsrq, "-11 dB");
        assert_eq!(info.sinr, "21 dB");
        assert_eq!(info.tx_power, "");   // -32767 filtered out
        assert_eq!(info.rx_level, "");   // 255 filtered out
        assert_eq!(info.scs, "1");
    }

    #[test]
    fn parse_qeng_nr5g_sa_17_fields_no_scs() {
        // Some firmware omits the SCS field (17 fields instead of 18)
        let raw = r#"+QENG: "servingcell","CONNECT","NR5G-SA","SA",460,11,0B46E280,0,120,632928,78,100,-97,-11,21,-32767,255"#;
        let info = parse_qeng_serving_cell(raw, true);
        assert_eq!(info.tech, "NR5G-SA");
        assert_eq!(info.arfcn, "632928");
        assert_eq!(info.rsrp, "-97 dBm");
        assert_eq!(info.scs, "");
    }

    #[test]
    fn parse_qeng_nr5g_nsa_uses_nr_fields() {
        // NSA format: LTE anchor [3..18] + NR secondary [19..24]
        let raw = r#"+QENG: "servingcell","CONNECT","NR5G-NSA","FDD",460,11,1A2B3C4D,100,1300,3,3,5,5AE,-102,-12,-45,24,22,0,504990,78,106,-82,-10,28"#;
        let info = parse_qeng_serving_cell(raw, true);
        assert_eq!(info.tech, "NR5G-NSA");
        assert_eq!(info.arfcn, "504990");   // NR-ARFCN, not LTE EARFCN
        assert_eq!(info.band, "78");         // NR band, not LTE band
        assert_eq!(info.rsrp, "-82 dBm");   // NR RSRP, not LTE RSRP
        assert_eq!(info.rsrq, "-10 dB");    // NR RSRQ
        assert_eq!(info.sinr, "28 dB");     // NR SINR
    }

    #[test]
    fn parse_qeng_returns_default_on_empty() {
        let info = parse_qeng_serving_cell("OK", true);
        assert!(!info.connected);
        assert!(info.tech.is_empty());
    }

    #[test]
    fn parse_cereg_returns_english_state() {
        assert_eq!(parse_cereg("+CEREG: 2,1\r\nOK").0, "CONNECT");
        assert_eq!(parse_cereg("+CEREG: 2,2\r\nOK").0, "SEARCH");
        assert_eq!(parse_cereg("+CEREG: 2,5\r\nOK").0, "CONNECT");
    }

    #[test]
    fn parse_cpin_ready() {
        assert_eq!(parse_cpin("+CPIN: READY\r\nOK"), "READY");
    }

    #[test]
    fn parse_cpin_cme_error_returns_no_sim() {
        assert_eq!(parse_cpin("+CME ERROR: 10\r\n"), "NO SIM");
    }

    #[test]
    fn parse_cpin_error_returns_no_sim() {
        assert_eq!(parse_cpin("ERROR\r\n"), "NO SIM");
    }

    #[test]
    fn parse_cpin_unknown_when_no_data() {
        assert_eq!(parse_cpin("OK"), "UNKNOWN");
    }

    #[test]
    fn parse_qtemp_qualcomm_picks_pa1_and_aoss() {
        let raw = r#"+QTEMP:"modem-lte-sub6-pa1","35"
+QTEMP:"modem-sdr0-pa0","0"
+QTEMP:"modem-sdr0-pa1","0"
+QTEMP:"modem-mmw0","-273"
+QTEMP:"aoss-0-usr","38"
+QTEMP:"cpuss-0-usr","36"
+QTEMP:"mdmss-0-usr","37"
+QTEMP:"modem-lte-sub6-pa2","35"
+QTEMP:"modem-ambient-usr","36"
OK"#;
        let info = parse_qtemp(raw);
        assert_eq!(info.pa_temp, "35°C");
        assert_eq!(info.soc_temp, "38°C");
    }
}