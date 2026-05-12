use crate::types::{ServingCellInfo, SignalInfo};

/// Parse AT^HCSQ? response — converts indexed values to dBm strings
pub fn parse_hcsq(response: &str) -> SignalInfo {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^HCSQ: ") {
            let parts: Vec<&str> = rest
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            let mode = parts.get(0).unwrap_or(&"");
            if *mode == "LTE" && parts.len() >= 5 {
                // ^HCSQ: "LTE",rssi,rsrp,sinr,rsrq
                let rsrp_idx: u32 = parts[2].parse().unwrap_or(255);
                let sinr_idx: u32 = parts[3].parse().unwrap_or(255);
                let rsrq_idx: u32 = parts[4].parse().unwrap_or(255);
                return SignalInfo {
                    rsrp: decode_rsrp(rsrp_idx),
                    sinr: decode_sinr(sinr_idx),
                    rsrq: decode_rsrq(rsrq_idx),
                    ant_values: Default::default(),
                };
            }
            if *mode == "NR" && parts.len() >= 4 {
                // ^HCSQ: "NR",rsrp,sinr,rsrq
                let rsrp_idx: u32 = parts[1].parse().unwrap_or(255);
                let sinr_idx: u32 = parts[2].parse().unwrap_or(255);
                let rsrq_idx: u32 = parts[3].parse().unwrap_or(255);
                return SignalInfo {
                    rsrp: decode_rsrp(rsrp_idx),
                    sinr: decode_sinr(sinr_idx),
                    rsrq: decode_rsrq(rsrq_idx),
                    ant_values: Default::default(),
                };
            }
        }
    }
    SignalInfo::default()
}

fn decode_rsrp(idx: u32) -> String {
    if idx == 255 {
        return "N/A".to_string();
    }
    format!("{}", idx as i32 - 140)
}

fn decode_sinr(idx: u32) -> String {
    if idx == 255 {
        return "N/A".to_string();
    }
    let val = (idx as f32) * 0.2 - 20.0;
    format!("{:.1}", val)
}

fn decode_rsrq(idx: u32) -> String {
    if idx == 255 {
        return "N/A".to_string();
    }
    let val = (idx as f32) * 0.5 - 19.5;
    format!("{:.1}", val)
}

/// Parse AT^MONSC response — direct dBm values
pub fn parse_monsc(response: &str) -> ServingCellInfo {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^MONSC: ") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
            let rat = parts.get(0).unwrap_or(&"");
            if *rat == "LTE" && parts.len() >= 10 {
                // ^MONSC: LTE,MCC,MNC,ARFCN,Cell_ID,PCI,TAC,RSRP,RSRQ,RSSI
                return ServingCellInfo {
                    connected: true,
                    tech: "LTE".to_string(),
                    operator_mcc: parts.get(1).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(2).unwrap_or(&"").to_string(),
                    arfcn: parts.get(3).unwrap_or(&"").to_string(),
                    cell_id: parts.get(4).unwrap_or(&"").to_string(),
                    pci: parts.get(5).unwrap_or(&"").to_string(),
                    band: String::new(),
                    bandwidth: String::new(),
                    rsrp: parts.get(7).unwrap_or(&"").to_string(),
                    rsrq: parts.get(8).unwrap_or(&"").to_string(),
                    sinr: String::new(),
                    tx_power: String::new(),
                    scs: String::new(),
                };
            }
            if *rat == "NR" && parts.len() >= 11 {
                // ^MONSC: NR,MCC,MNC,ARFCN,SCS,Cell_ID,PCI,TAC,RSRP,RSRQ,SINR
                return ServingCellInfo {
                    connected: true,
                    tech: "NR".to_string(),
                    operator_mcc: parts.get(1).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(2).unwrap_or(&"").to_string(),
                    arfcn: parts.get(3).unwrap_or(&"").to_string(),
                    scs: parts.get(4).unwrap_or(&"").to_string(),
                    cell_id: parts.get(5).unwrap_or(&"").to_string(),
                    pci: parts.get(6).unwrap_or(&"").to_string(),
                    band: String::new(),
                    bandwidth: String::new(),
                    rsrp: parts.get(8).unwrap_or(&"").to_string(),
                    rsrq: parts.get(9).unwrap_or(&"").to_string(),
                    sinr: parts.get(10).unwrap_or(&"").to_string(),
                    tx_power: String::new(),
                };
            }
        }
    }
    ServingCellInfo::default()
}

/// Parse AT^DCONNSTAT? — check if any CID is connected
pub fn parse_dconnstat(response: &str) -> bool {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^DCONNSTAT:") {
            // ^DCONNSTAT: <cid>,"<APN>",<ipv4_stat>,<ipv6_stat>,<type>
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.get(2).map(|s| s.trim()) == Some("1") {
                return true;
            }
        }
    }
    false
}

/// Parse AT^SYSCFGEX? — extract acqorder and lteband hex
pub fn parse_syscfgex(response: &str) -> (String, String) {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^SYSCFGEX:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            let acqorder = parts.get(0).unwrap_or(&"").to_string();
            let lteband = parts.get(4).unwrap_or(&"").to_string();
            return (acqorder, lteband);
        }
    }
    (String::new(), String::new())
}

/// Convert TdTech hex IP (big-endian u32) to dotted decimal
pub fn hex_ip_to_string(hex: &str) -> String {
    let n =
        u32::from_str_radix(hex.trim_start_matches("0x").trim_start_matches("0X"), 16).unwrap_or(0);
    let b = n.to_be_bytes();
    format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hcsq_lte_converts_indices() {
        // rsrp_idx=60 → -140+60 = -80 dBm
        // sinr_idx=200 → 200*0.2-20 = 20.0 dB
        // rsrq_idx=20 → 20*0.5-19.5 = -9.5 dB
        let raw = "^HCSQ: \"LTE\",30,60,200,20";
        let s = parse_hcsq(raw);
        assert_eq!(s.rsrp, "-80");
        assert_eq!(s.sinr, "20.0");
        assert_eq!(s.rsrq, "-9.5");
    }

    #[test]
    fn parse_hcsq_unknown_index_255_returns_na() {
        let raw = "^HCSQ: \"LTE\",255,255,255,255";
        let s = parse_hcsq(raw);
        assert_eq!(s.rsrp, "N/A");
        assert_eq!(s.sinr, "N/A");
        assert_eq!(s.rsrq, "N/A");
    }

    #[test]
    fn parse_monsc_lte_returns_direct_dbm() {
        let raw = "^MONSC: LTE,460,11,2650,1A2B3C,5AE,1234,-95,-10,-75";
        let c = parse_monsc(raw);
        assert_eq!(c.tech, "LTE");
        assert_eq!(c.arfcn, "2650");
        assert_eq!(c.rsrp, "-95");
        assert_eq!(c.rsrq, "-10");
    }

    #[test]
    fn parse_monsc_nr_returns_direct_dbm() {
        let raw = "^MONSC: NR,460,11,504990,1,AABBCC,3EF,112233,-85,-12,15";
        let c = parse_monsc(raw);
        assert_eq!(c.tech, "NR");
        assert_eq!(c.sinr, "15");
        assert_eq!(c.scs, "1");
    }

    #[test]
    fn hex_ip_conversion() {
        // 0xC0A80101 = 192.168.1.1
        assert_eq!(hex_ip_to_string("0xC0A80101"), "192.168.1.1");
        assert_eq!(hex_ip_to_string("0x08080808"), "8.8.8.8");
    }
}
