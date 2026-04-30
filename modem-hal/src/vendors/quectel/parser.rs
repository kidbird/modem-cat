use crate::types::{ServingCellInfo, NeighborCells, TemperatureInfo, ApnEntry};

/// Parse AT+QENG="servingcell" response.
/// `qualcomm_bandwidth`: true = index lookup, false = direct MHz value (UniSoc)
pub fn parse_qeng_serving_cell(response: &str, qualcomm_bandwidth: bool) -> ServingCellInfo {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("+QENG: \"servingcell\",") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
            if parts.len() < 4 { continue; }
            let connected = parts[0] != "NOCONN";
            let tech = parts[1].to_string();

            if tech == "LTE" && parts.len() >= 17 {
                // conn,rat,duplex,mcc,mnc,cellid,pcid,earfcn,band,bw,dl_bw,tac,rsrp,rsrq,rssi,sinr,srxlev
                let bw_raw = parts.get(9).unwrap_or(&"").trim();
                let bandwidth = if qualcomm_bandwidth {
                    decode_qualcomm_bandwidth(bw_raw.parse::<u32>().unwrap_or(0))
                } else {
                    bw_raw.to_string()
                };
                return ServingCellInfo {
                    connected,
                    tech,
                    operator_mcc: parts.get(3).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(4).unwrap_or(&"").to_string(),
                    cell_id: parts.get(5).unwrap_or(&"").to_string(),
                    pci: parts.get(6).unwrap_or(&"").to_string(),
                    arfcn: parts.get(7).unwrap_or(&"").to_string(),
                    band: parts.get(8).unwrap_or(&"").to_string(),
                    bandwidth,
                    rsrp: parts.get(12).unwrap_or(&"").to_string(),
                    rsrq: parts.get(13).unwrap_or(&"").to_string(),
                    sinr: parts.get(15).unwrap_or(&"").to_string(),
                    tx_power: String::new(),
                    scs: String::new(),
                };
            }

            if tech == "NR5G-SA" && parts.len() >= 14 {
                // conn,rat,mcc,mnc,cellid,pcid,arfcn,band,arfcn_ul,scs,rsrp,rsrq,sinr,srxlev
                return ServingCellInfo {
                    connected,
                    tech,
                    operator_mcc: parts.get(2).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(3).unwrap_or(&"").to_string(),
                    cell_id: parts.get(4).unwrap_or(&"").to_string(),
                    pci: parts.get(5).unwrap_or(&"").to_string(),
                    arfcn: parts.get(6).unwrap_or(&"").to_string(),
                    band: parts.get(7).unwrap_or(&"").to_string(),
                    bandwidth: String::new(),
                    rsrp: parts.get(10).unwrap_or(&"").to_string(),
                    rsrq: parts.get(11).unwrap_or(&"").to_string(),
                    sinr: parts.get(12).unwrap_or(&"").to_string(),
                    tx_power: String::new(),
                    scs: parts.get(9).unwrap_or(&"").to_string(),
                };
            }
        }
    }
    ServingCellInfo::default()
}

/// Qualcomm bandwidth index → MHz string: 0→1.4, 1→3, 2→5, 3→10, 4→15, 5→20, 6→100
fn decode_qualcomm_bandwidth(idx: u32) -> String {
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

pub fn parse_cpin(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CPIN: ") { return rest.trim().to_string(); }
    }
    "UNKNOWN".to_string()
}

pub fn parse_cgsn(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') { continue; }
        if line.chars().all(|c| c.is_ascii_digit()) && line.len() >= 14 { return line; }
    }
    String::new()
}

pub fn parse_iccid(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CCID: ")
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
        if line.starts_with('+') { continue; }
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

pub fn parse_qtemp(response: &str) -> TemperatureInfo {
    let mut info = TemperatureInfo::default();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QTEMP: ") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
            match parts.get(0).unwrap_or(&"") {
                &"soc-thermal" | &"mdm-core" => {
                    info.soc_temp = parts.get(1).unwrap_or(&"").to_string();
                }
                &"pa-thermal" | &"pa0-thermal" => {
                    info.pa_temp = parts.get(1).unwrap_or(&"").to_string();
                }
                _ => {}
            }
        }
    }
    info
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

pub fn parse_cgdcont_apn(response: &str, active_cids: &std::collections::HashSet<i32>) -> Vec<ApnEntry> {
    let mut entries = vec![];
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CGDCONT:") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
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

pub fn parse_band_list(response: &str) -> Vec<String> {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNWPREFCFG:") {
            let parts: Vec<&str> = rest.split(',').collect();
            if parts.len() >= 2 {
                return parts[1].trim().trim_matches('"')
                    .split(':')
                    .map(|b| b.trim_start_matches('B').to_string())
                    .collect();
            }
        }
    }
    vec![]
}

pub fn parse_qeng_neighbour_cells(_response: &str) -> NeighborCells {
    NeighborCells { lte: vec![], nr: vec![] }
}

pub fn extract_data_lines(response: &str) -> Vec<String> {
    response.lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && t != "OK" && !t.starts_with("ERROR")
                && !t.starts_with("+CME ERROR")
                && !t.starts_with("AT+") && !t.starts_with("AT^")
                && t != "AT"
        })
        .map(|l| l.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_qeng_lte_qualcomm_bandwidth_index() {
        // Qualcomm: bandwidth field is an index (5 → 20 MHz)
        let raw = r#"+QENG: "servingcell","CONNECT","LTE","FDD",460,11,1A2B3C4,100,2650,3,5,5,5AE,-95,-10,-65,18,10"#;
        let info = parse_qeng_serving_cell(raw, true);
        assert_eq!(info.bandwidth, "20");
        assert_eq!(info.arfcn, "2650");
        assert_eq!(info.rsrp, "-95");
        assert_eq!(info.sinr, "18");
    }

    #[test]
    fn parse_qeng_lte_unisoc_bandwidth_direct() {
        // UniSoc: bandwidth field is already MHz
        let raw = r#"+QENG: "servingcell","CONNECT","LTE","FDD",460,11,1A2B3C4,100,2650,3,100,5,5AE,-95,-10,-65,18,10"#;
        let info = parse_qeng_serving_cell(raw, false);
        assert_eq!(info.bandwidth, "100");
    }

    #[test]
    fn parse_qeng_returns_default_on_empty() {
        let info = parse_qeng_serving_cell("OK", true);
        assert!(!info.connected);
    }
}
