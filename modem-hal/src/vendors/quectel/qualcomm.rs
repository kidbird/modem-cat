use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};
use super::parser::parse_cgact;

pub fn connect_data(t: &mut dyn AtTransport) -> Result<(), String> {
    use super::parser::is_ok;
    let resp = t.send_at("AT+QMAP=\"connect\",0,1")?;
    if !is_ok(&resp) {
        return Err(format!("QMAP connect failed: {}", resp.trim()));
    }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport) -> Result<(), String> {
    use super::parser::is_ok;
    let resp = t.send_at("AT+QMAP=\"connect\",0,0")?;
    if !is_ok(&resp) {
        return Err(format!("QMAP disconnect failed: {}", resp.trim()));
    }
    Ok(())
}

/// Parse `AT+QMAP="MPDN_rule"` query response — return IPPT mode for rule 0.
/// Response: +QMAP: "MPDN_rule",<rule_num>,<profileID>,<p3>,<IPPT_mode>,<auto_connect>
/// Returns: 0=关闭 (profileID=0), 1=IPPT路由 (IPPT_mode=0), 2=IPPT桥接 (IPPT_mode=1, ETH)
pub fn parse_mpdn_ippt_mode(response: &str) -> i32 {
    for line in response.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim().trim_matches('"')).collect();
            if parts.len() >= 5 && parts[0].eq_ignore_ascii_case("MPDN_rule") && parts[1].trim() == "0" {
                let profile_id: i32 = parts[2].trim().parse().unwrap_or(0);
                if profile_id == 0 { return 0; }
                let ippt_mode: i32 = parts[4].trim().parse().unwrap_or(0);
                return if ippt_mode == 1 { 2 } else { 1 };
            }
        }
    }
    0
}

/// Parse `AT+QMAP="MPDN_status"` response, return connect_status of rule 0.
/// Response format: +QMAP: "MPDN_status",<rule_num>,<profileID>,<IPPT_status>,<connect_status>
pub fn parse_mpdn_connect_status(response: &str) -> bool {
    for line in response.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim().trim_matches('"')).collect();
            // parts[0]="MPDN_status", parts[1]=rule_num, parts[2]=profileID, parts[3]=IPPT_status, parts[4]=connect_status
            if parts.len() >= 5 && parts[0].eq_ignore_ascii_case("MPDN_status") {
                if parts[1].trim() == "0" {
                    return parts[4].trim() == "1";
                }
            }
        }
    }
    false
}

fn ip_info_default() -> IpInfo {
    IpInfo {
        ipv4_addr: String::new(),
        ipv4_mask: String::new(),
        ipv4_gw: String::new(),
        ipv4_dns: String::new(),
        ipv6_addr: String::new(),
        ipv6_dns: String::new(),
    }
}

pub fn parse_ims_cids(response: &str) -> Vec<i32> {
    let mut ims = Vec::new();
    for line in response.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("+CGDCONT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 3 {
                let apn = parts[2].trim().trim_matches('"').to_lowercase();
                if apn == "ims" {
                    if let Ok(cid) = parts[0].trim().parse::<i32>() {
                        ims.push(cid);
                    }
                }
            }
        }
    }
    ims
}

fn dotted_to_ipv6(decimal_str: &str) -> String {
    let bytes: Vec<u8> = decimal_str
        .split('.')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();
    if bytes.len() != 16 {
        return String::new();
    }
    bytes
        .chunks(2)
        .map(|chunk| format!("{:02x}{:02x}", chunk[0], chunk[1]))
        .collect::<Vec<_>>()
        .join(":")
}

fn parse_cgpaddr(response: &str, info: &mut IpInfo) {
    for line in response.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("+CGPADDR:") {
            let after_cid = match rest.trim().splitn(2, ',').nth(1) {
                Some(s) => s.trim(),
                None => continue,
            };
            let addrs: Vec<&str> = after_cid.split("\",\"").collect();
            let addr0 = addrs
                .first()
                .map(|s| s.trim().trim_matches('"'))
                .unwrap_or("");
            let addr1 = addrs
                .get(1)
                .map(|s| s.trim().trim_matches('"'))
                .unwrap_or("");

            if !addr0.is_empty() && addr0 != "0.0.0.0" {
                let dots = addr0.matches('.').count();
                if dots == 3 {
                    info.ipv4_addr = addr0.to_string();
                } else if dots == 15 {
                    info.ipv6_addr = dotted_to_ipv6(addr0);
                }
            }
            if !addr1.is_empty() && addr1 != "0.0.0.0" {
                let dots = addr1.matches('.').count();
                if dots == 15 {
                    info.ipv6_addr = dotted_to_ipv6(addr1);
                }
            }
        }
    }
}

fn has_valid_ip(info: &IpInfo) -> bool {
    !info.ipv4_addr.is_empty() || !info.ipv6_addr.is_empty()
}

fn parse_qmap_wwan(response: &str) -> IpInfo {
    let mut info = ip_info_default();
    for line in response.lines() {
        let t2 = line.trim();
        if let Some(rest) = t2.strip_prefix("+QMAP: \"WWAN\",") {
            let parts: Vec<&str> = rest
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 5 {
                let family = parts.get(3).unwrap_or(&"");
                let addr = parts.get(4).unwrap_or(&"");
                if *addr == "0.0.0.0" {
                    continue;
                }
                if *family == "IPV4" {
                    info.ipv4_addr = addr.to_string();
                } else if *family == "IPV6" {
                    info.ipv6_addr = addr.to_string();
                }
            }
        }
    }
    info
}

pub fn query_ip_info(t: &mut dyn AtTransport, data_cid: i32) -> Result<IpInfo, String> {
    let qmap_resp = t.send_at("AT+QMAP=\"WWAN\"")?;
    let info = parse_qmap_wwan(&qmap_resp);
    if has_valid_ip(&info) {
        return Ok(info);
    }

    let mut info = ip_info_default();

    let cgact_resp = t.send_at("AT+CGACT?")?;
    let active_cids: Vec<i32> = parse_cgact(&cgact_resp)
        .into_iter()
        .filter(|(_, status)| *status == 1)
        .map(|(cid, _)| cid)
        .collect();

    let cgdcont_resp = t.send_at("AT+CGDCONT?")?;
    let ims_cids = parse_ims_cids(&cgdcont_resp);

    let valid_cids: Vec<i32> = active_cids
        .into_iter()
        .filter(|c| !ims_cids.contains(c))
        .collect();

    if valid_cids.is_empty() {
        return Ok(info);
    }

    let target_cid = if valid_cids.contains(&data_cid) {
        data_cid
    } else {
        valid_cids[0]
    };

    let resp = t.send_at(&format!("AT+CGPADDR={}", target_cid))?;
    parse_cgpaddr(&resp, &mut info);
    Ok(info)
}

pub fn query_traffic(t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
    let resp = t.send_at("AT+QGDNRCNT?")?;
    for line in resp.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QGDNRCNT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                return Ok(TrafficInfo {
                    ul_bytes: parts[0].trim().parse().unwrap_or(0),
                    dl_bytes: parts[1].trim().parse().unwrap_or(0),
                });
            }
        }
    }
    Ok(TrafficInfo {
        ul_bytes: 0,
        dl_bytes: 0,
    })
}
