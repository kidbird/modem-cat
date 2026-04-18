use crate::types::{ApnEntry, NeighborCell};

/// Check if AT response indicates success (ends with OK)
pub fn is_ok(response: &str) -> bool {
    let trimmed = response.trim();
    trimmed.ends_with("OK") || trimmed.contains("OK\n") || trimmed.contains("OK\r\n")
}

/// Extract the data lines (skip echo, empty lines, OK, ERROR)
pub fn extract_data_lines(response: &str) -> Vec<String> {
    response
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && t != "OK" && !t.starts_with("ERROR") && !t.starts_with("+CME ERROR")
        })
        .filter(|l| {
            // Skip echo lines (command echoed back)
            let t = l.trim();
            !t.starts_with("AT+") && !t.eq("AT") && !t.eq("at")
        })
        .map(|l| l.trim().to_string())
        .collect()
}

// ── Basic info parsers ──

pub fn parse_cpin(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CPIN: ") {
            return rest.trim().to_string();
        }
    }
    "UNKNOWN".to_string()
}

pub fn parse_cgsn(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') { continue; }
        if line.chars().all(|c| c.is_ascii_digit()) && line.len() >= 14 {
            return line;
        }
    }
    String::new()
}

pub fn parse_ccid(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CCID: ") {
            return rest.trim().to_string();
        }
        if let Some(rest) = line.strip_prefix("+ICCID: ") {
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

pub fn parse_cgmi(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') { continue; }
        return line.trim().to_string();
    }
    String::new()
}

pub fn parse_cgmr(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') { continue; }
        return line.trim().to_string();
    }
    String::new()
}

/// Parse AT+GMR → firmware revision
/// Real: "+GMR: RM500UCNVABR11A06M4G_01.200.01.200"
pub fn parse_gmr(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+GMR:") {
            return rest.trim().to_string();
        }
    }
    // Fallback: first non-empty, non-echo line
    for line in response.lines() {
        let t = line.trim();
        if t.is_empty() || t == "OK" || t.starts_with("AT+") || t.starts_with('+') {
            continue;
        }
        return t.to_string();
    }
    String::new()
}

// ── Serving cell info ──

/// Parse AT+QENG="servingcell"
///
/// Real NR5G-SA response:
///   +QENG: "servingcell","NOCONN","NR5G-SA","TDD",460,15,155843002,222,200028,504990,41,100,-94,-4,3,0,25,1
///   Parts: [0]"servingcell" [1]state [2]tech [3]duplex [4]MCC [5]MNC [6]cellID [7]PCI [8]TAC [9]ARFCN [10]band [11]BW [12]RSRP [13]RSRQ [14]SINR [15]SCS [16]tx_power [17]srxlev
///
/// Real LTE response (expected):
///   +QENG: "servingcell",<state>,"LTE","FDD",<MCC>,<MNC>,<cellID>,<PCI>,<earfcn>,<band>,<UL_BW>,<DL_BW>,<TAC>,<RSRP>,<RSRQ>,<RSSI>,<SINR>,<CQI>,<tx_power>,<srxlev>
pub struct ServingCellInfo {
    pub connected: bool,
    pub tech: String,
    pub operator_mcc: String,
    pub operator_mnc: String,
    pub cell_id: String,
    pub pci: String,
    pub arfcn: String,
    pub band: String,
    pub bandwidth: String,
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub tx_power: String,
    pub scs: String,
}

pub fn parse_qeng_servingcell(response: &str) -> Option<ServingCellInfo> {
    for line in extract_data_lines(response) {
        if !line.starts_with("+QENG:") { continue; }
        let data = line.strip_prefix("+QENG:").unwrap().trim();
        let parts: Vec<&str> = data.splitn(25, ',').collect();
        if parts.len() < 3 { continue; }

        let state = parts[1].trim().trim_matches('"');
        let tech = parts[2].trim().trim_matches('"');
        // "NOCONN" = registered but no RRC connection, "CONNECT" = registered with RRC
        let connected = state != "SEARCH";

        match tech {
            "NR5G-SA" if parts.len() >= 16 => {
                return Some(ServingCellInfo {
                    connected,
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
                    sinr: parts[14].trim().to_string(),
                    scs: parts[15].trim().to_string(),
                    tx_power: parts.get(16).unwrap_or(&"").trim().to_string(),
                });
            }
            "LTE" if parts.len() >= 19 => {
                return Some(ServingCellInfo {
                    connected,
                    tech: tech.to_string(),
                    operator_mcc: parts[4].trim().trim_matches('"').to_string(),
                    operator_mnc: parts[5].trim().trim_matches('"').to_string(),
                    cell_id: parts[6].trim().to_string(),
                    pci: parts[7].trim().to_string(),
                    arfcn: parts[8].trim().to_string(),
                    band: parts[9].trim().to_string(),
                    bandwidth: format_bw(parts[11].trim()),
                    rsrp: format_rsrp(parts[12].trim()),
                    rsrq: format_rsrq(parts[13].trim()),
                    sinr: parts[16].trim().to_string(),
                    tx_power: parts[18].trim().to_string(),
                    scs: String::new(),
                });
            }
            "NR5G-NSA" if parts.len() >= 15 => {
                return Some(ServingCellInfo {
                    connected,
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
                    sinr: parts[14].trim().to_string(),
                    tx_power: String::new(),
                    scs: String::new(),
                });
            }
            _ => {}
        }
    }
    None
}

// ── Signal parsers ──

/// Parse AT+QRSRP
/// May return +CME ERROR if unsupported, or "+QRSRP: <rsrp0>,<rsrp1>,<rsrp2>,<rsrp3>"
pub fn parse_qrsrp(response: &str) -> (String, String, [String; 4]) {
    let mut rsrp = String::new();
    let mut rsrq = String::new();
    let mut ant = [String::new(), String::new(), String::new(), String::new()];

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QRSRP:") {
            let vals: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
            if vals.len() >= 4 {
                for (i, v) in vals.iter().take(4).enumerate() {
                    ant[i] = format_rsrp(v);
                }
                rsrp = ant[0].clone();
            } else if vals.len() >= 2 {
                rsrp = format_rsrp(vals[0]);
                rsrq = format_rsrq(vals[1]);
            } else if vals.len() == 1 {
                rsrp = format_rsrp(vals[0]);
            }
        }
    }

    (rsrp, rsrq, ant)
}

/// Parse AT+QSNR → SINR
pub fn parse_qsnr(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QSNR:") {
            return format!("{} dB", rest.trim());
        }
    }
    String::new()
}

/// Parse AT+QANTRSSI? → antenna values
/// Real: "+QANTRSSI: 1,-87,-69,-75,-72" (first value is count, then 4 antenna values)
pub fn parse_qantrssi(response: &str) -> [String; 4] {
    let mut ant = [String::new(), String::new(), String::new(), String::new()];

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QANTRSSI:") {
            let vals: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
            if vals.len() >= 5 {
                // Format: count,ant0,ant1,ant2,ant3
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

// ── Network status parsers ──

/// Parse AT+QNETDEVSTATUS=<cid>
/// Real: "+QNETDEVSTATUS: 10.2.133.230,255.255.255.0,10.2.133.1,,43.239.172.1,43.239.172.2,,,,,,"
/// Parts: [0]ipv4 [1]mask [2]gw [3]empty [4]dns1 [5]dns2 ...
pub fn parse_qnetdevstatus(response: &str) -> (String, String, String, String, String) {
    let mut ipv4 = String::new();
    let mut mask = String::new();
    let mut gw = String::new();
    let mut dns = String::new();
    let ipv6 = String::new();

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNETDEVSTATUS:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|v| v.trim()).collect();
            if !parts.is_empty() { ipv4 = parts[0].to_string(); }
            if parts.len() > 1 { mask = parts[1].to_string(); }
            if parts.len() > 2 { gw = parts[2].to_string(); }
            if parts.len() > 4 { dns = parts[4].to_string(); }
            // IPv6 might be in later positions
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

/// Parse AT+CGACT? → vec of (cid, status)
pub fn parse_cgact(response: &str) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CGACT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                if let (Ok(cid), Ok(status)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                    result.push((cid, status));
                }
            }
        }
    }
    result
}

/// Parse AT+C5GQOSRDP=<cid>
/// Real: "+C5GQOSRDP: 1,9,0,0,0,0,1000000,100000,0"
/// Parts: [0]cid [1]5qi [2-5]zeros [6]dl_bw(bps) [7]ul_bw(bps) [8]unknown
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

/// Parse AT+QICSGP? → APN list
/// `active_cids` contains CIDs that are currently active (from AT+CGACT?).
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
                    _ => "IPv4",
                };
                let apn_name = parts.get(2).map(|v| v.trim().trim_matches('"')).unwrap_or("").to_string();
                let username = parts.get(3).map(|v| v.trim().trim_matches('"')).unwrap_or("").to_string();
                let auth_type = parts.get(5).and_then(|v| v.trim().parse::<i32>().ok()).unwrap_or(0);
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

/// Parse AT+QENG="neighbourcell" → (LTE cells, NR cells)
///
/// Supported formats (from Quectel RGx00U&RM500U AT Command Manual V1.1):
///
/// NR/EN-DC mode:
///   +QENG: "neighbourcell","NR",<EARFCN>,<PCID>,<RSRP>,<RSRQ>,<SINR>,<srxlev>,<RAT>
///   +QENG: "neighbourcell","LTE",<EARFCN>,<PCID>,<RSRP>,<RSRQ>,<SINR>,<srxlev>,<RAT>
///
/// LTE mode:
///   +QENG: "neighbourcell intra","LTE",<EARFCN>,<PCID>,<RSRP>,<RSRQ>,<srxlev>,<SINR>,<cell_resel_priority>,<threshX_low>,<threshX_high>,<RAT>
///   +QENG: "neighbourcell inter","LTE",<EARFCN>,<PCID>,<RSRP>,<RSRQ>,<srxlev>,<SINR>,<cell_resel_priority>,<threshX_low>,<threshX_high>,<RAT>
///
/// WCDMA mode:
///   +QENG: "neighbourcell","WCDMA",<UARFCN>,<srxqual>,<PSC>,<RSCP>,<ecno>,<set>,<rank>,<srxlev>,<thresh_Xhigh>,<thresh_Xlow>,<RAT>
pub fn parse_qeng_neighbourcell(response: &str) -> (Vec<NeighborCell>, Vec<NeighborCell>) {
    use crate::types::NeighborCell;
    let mut lte_cells = Vec::new();
    let mut nr_cells = Vec::new();

    for line in extract_data_lines(response) {
        if !line.starts_with("+QENG: \"neighbourcell") {
            continue;
        }

        // Strip the +QENG: prefix
        let rest = line.strip_prefix("+QENG: ").unwrap_or("");

        // The line starts with a quoted string like "neighbourcell" or "neighbourcell intra"
        // followed by a comma and the RAT type: "NR", "LTE", or "WCDMA"
        let bytes = rest.as_bytes();
        let mut pos = 0;

        // Skip opening quote
        if pos >= bytes.len() || bytes[pos] != b'"' {
            continue;
        }
        pos += 1;

        // Read until closing quote of first quoted string
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b'"' {
            pos += 1;
        }
        if pos >= bytes.len() {
            continue;
        }
        let _cell_type = std::str::from_utf8(&bytes[start..pos]).unwrap_or("");
        pos += 1; // skip closing quote

        // Skip comma
        if pos < bytes.len() && bytes[pos] == b',' {
            pos += 1;
        }

        // Skip whitespace
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        // Read RAT type: "NR", "LTE", or "WCDMA"
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
        let rat = std::str::from_utf8(&bytes[rat_start..pos]).unwrap_or("").trim();
        pos += 1; // skip closing quote

        // Remaining fields after the RAT quote (skip leading comma if present)
        let remaining = &rest[pos..];
        let remaining = remaining.strip_prefix(',').unwrap_or(remaining);
        let parts: Vec<&str> = remaining.split(',').map(|s| s.trim()).collect();

        match rat {
            "NR" => {
                // +QENG: "neighbourcell","NR",<EARFCN>,<PCID>,<RSRP>,<RSRQ>,<SINR>,<srxlev>,<RAT>
                // parts: [EARFCN, PCID, RSRP, RSRQ, SINR, srxlev, RAT]
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
                // Two possible field layouts depending on mode:
                // NR/EN-DC: <EARFCN>,<PCID>,<RSRP>,<RSRQ>,<SINR>,<srxlev>,<RAT>
                // LTE mode:  <EARFCN>,<PCID>,<RSRP>,<RSRQ>,<srxlev>,<SINR>,<cell_resel_priority>,<threshX_low>,<threshX_high>,<RAT>
                if parts.len() >= 6 {
                    let earfcn = parts.get(0).unwrap_or(&"").to_string();
                    let pci = parts.get(1).unwrap_or(&"").to_string();
                    let rsrp = parts.get(2).unwrap_or(&"").to_string();
                    let rsrq = parts.get(3).unwrap_or(&"").to_string();
                    // Distinguish by total part count:
                    // NR/EN-DC LTE has ~7 parts, LTE mode has ~10 parts
                    let (sinr, srxlev) = if parts.len() >= 10 {
                        // LTE mode: srxlev at index 4, sinr at index 5
                        (parts.get(5).unwrap_or(&"").to_string(), parts.get(4).unwrap_or(&"").to_string())
                    } else {
                        // NR/EN-DC mode: sinr at index 4, srxlev at index 5
                        (parts.get(4).unwrap_or(&"").to_string(), parts.get(5).unwrap_or(&"").to_string())
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
                // +QENG: "neighbourcell","WCDMA",<UARFCN>,<srxqual>,<PSC>,<RSCP>,<ecno>,<set>,<rank>,<srxlev>,<thresh_Xhigh>,<thresh_Xlow>,<RAT>
                // Treat WCDMA as LTE neighbours for display purposes
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

    (lte_cells, nr_cells)
}

/// Parse AT+COPS? → (operator_name, act)
/// Real: "+COPS: 0,0,"CHINA BROADNET",11"
pub fn parse_cops(response: &str) -> (String, String) {
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

/// Parse AT+QNWPREFCFG="mode_pref"
/// Real: "+QNWPREFCFG: "mode_pref",AUTO"
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

/// Parse AT+CEREG?
pub fn parse_cereg(response: &str) -> (String, Option<String>, Option<String>) {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CEREG:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                let stat = parts[1].trim();
                let status_str = match stat {
                    "0" => "未注册",
                    "1" => "已注册(本地)",
                    "2" => "搜索中",
                    "3" => "注册拒绝",
                    "4" => "未知",
                    "5" => "已注册(漫游)",
                    _ => stat,
                };
                let tac = parts.get(2).map(|v| v.trim().to_string());
                let ci = parts.get(3).map(|v| v.trim().to_string());
                return (status_str.to_string(), tac, ci);
            }
        }
    }
    ("未注册".to_string(), None, None)
}

// ── Formatting helpers ──

fn format_rsrp(val: &str) -> String {
    if val.is_empty() || val == "0" { return String::new(); }
    if let Ok(v) = val.parse::<i32>() {
        if v < 0 { format!("{} dBm", v) }
        else { format!("-{} dBm", v) }
    } else {
        val.to_string()
    }
}

fn format_rsrq(val: &str) -> String {
    if val.is_empty() || val == "0" { return String::new(); }
    if let Ok(v) = val.parse::<i32>() {
        if v < 0 { format!("{} dB", v) }
        else { format!("-{} dB", v) }
    } else {
        val.to_string()
    }
}

/// Bandwidth value from QENG: raw number → "100 MHz"
fn format_bw(val: &str) -> String {
    if val.is_empty() || val == "0" { return String::new(); }
    if let Ok(v) = val.parse::<u32>() {
        format!("{} MHz", v)
    } else {
        val.to_string()
    }
}

/// Bandwidth in Kbps from C5GQOSRDP → human readable
/// 1000000 → "1000 Mbps", 100000 → "100 Mbps"
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

/// Parse AT+QBASELINE → (ap_baseline, cp_baseline)
/// Real response:
///   AP: UNC_LINUX_TRUNK_20C_W23.38.4_P35
///   CP: 5G_MODEM_21A_W23.38.3_P35
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

/// Parse AT+QTEMP → (soc_temp, pa_temp)
/// Real response:
///   +QTEMP: "soc-thermal","48"
///   +QTEMP: "pa-thermal","46"
///   +QTEMP: "pa5g-thermal","46"
///   +QTEMP: "board-thermal","47"
pub fn parse_qtemp(response: &str) -> (String, String) {
    let mut soc = String::new();
    let mut pa = String::new();
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QTEMP:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim().trim_matches('"')).collect();
            if parts.len() >= 2 {
                let label = parts[0].to_lowercase();
                let value = format!("{}°C", parts[1]);
                if label.contains("soc") && soc.is_empty() {
                    soc = value;
                } else if label.contains("pa5g") && pa.is_empty() {
                    pa = value;
                } else if label.contains("pa") && !label.contains("5g") && pa.is_empty() {
                    pa = value;
                }
            }
        }
    }
    (soc, pa)
}

// ── Band configuration parsers ──

/// Parse AT+QNWPREFCFG="lte_band" or AT+QNWPREFCFG="nr5g_band"
/// Format: +QNWPREFCFG: "lte_band",1:3:5:7:8:20:28:34:38:39:40:41
/// Returns band display names like ["B1", "B3", "B5", ...] for LTE
/// or ["n1", "n3", "n5", ...] for NR5G
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

/// Parse AT+QNWPREFCFG=? response to get supported bands
/// Format:
/// +QNWPREFCFG: "gw_band",(<gw_band>)
/// +QNWPREFCFG: "lte_band",(1:3:5:7:8:20:28:34:38:39:40:41)
/// +QNWPREFCFG: "nr5g_band",(1:3:5:7:8:20:28:38:40:41:77:78:79)
/// Returns (lte_supported, nr_supported)
pub fn parse_qnwprefcfg_supported(response: &str) -> (Vec<String>, Vec<String>) {
    let mut lte = Vec::new();
    let mut nr = Vec::new();

    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QNWPREFCFG:") {
            let rest = rest.trim();
            // Find the band type and value
            // Handles both: +QNWPREFCFG: "lte_band",(1:3:5) and +QNWPREFCFG: "lte_band",1:3:5
            if let Some(pos) = rest.find("\"lte_band\"") {
                let after = &rest[pos + "\"lte_band\"".len()..];
                let value = if let Some(start) = after.find('(') {
                    if let Some(end) = after.find(')') {
                        Some(&after[start + 1..end])
                    } else { None }
                } else if let Some(start) = after.find(',') {
                    Some(after[start + 1..].trim())
                } else { None };
                if let Some(v) = value {
                    lte = v.split(':').map(|s| s.trim()).filter(|s| !s.is_empty())
                        .map(|s| format!("B{}", s)).collect();
                }
            } else if let Some(pos) = rest.find("\"nr5g_band\"") {
                let after = &rest[pos + "\"nr5g_band\"".len()..];
                let value = if let Some(start) = after.find('(') {
                    if let Some(end) = after.find(')') {
                        Some(&after[start + 1..end])
                    } else { None }
                } else if let Some(start) = after.find(',') {
                    Some(after[start + 1..].trim())
                } else { None };
                if let Some(v) = value {
                    nr = v.split(':').map(|s| s.trim()).filter(|s| !s.is_empty())
                        .map(|s| format!("n{}", s)).collect();
                }
            }
        }
    }

    (lte, nr)
}

/// Parse AT+QCFG="pcie/mode" response
/// Response: +QCFG: "pcie/mode",1
pub fn parse_qcfg_int(response: &str, key: &str) -> Option<i32> {
    let prefix = format!("+QCFG: \"{}\",", key);
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix(&prefix) {
            return rest.trim().trim_end_matches(',').parse().ok();
        }
    }
    None
}

/// Parse AT+QCFG="usbcfg" response
/// Response: +QCFG: "usbcfg",0x2c7c,0x0900,1,1,1,1,1,1,1
/// ADB is the last parameter
pub fn parse_qcfg_usbcfg_adb(response: &str) -> bool {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+QCFG: \"usbcfg\",") {
            let parts: Vec<&str> = rest.split(',').collect();
            if let Some(last) = parts.last() {
                return last.trim() == "1";
            }
        }
    }
    false
}

/// Parse AT+QCFG="usbnet" response → returns the mode number
/// Response: +QCFG: "usbnet",3
pub fn parse_qcfg_usbnet(response: &str) -> Option<i32> {
    parse_qcfg_int(response, "usbnet")
}

/// Parse AT+QGDCNT? → (ul_bytes, dl_bytes)
/// Real: "+QGDCNT: 1234567,8901234"
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
