use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};

pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    use super::parser::is_ok;
    let resp = t.send_at(&format!("AT+QNETDEVCTL={},3,1", cid))?;
    if !is_ok(&resp) {
        return Err(format!("QNETDEVCTL connect failed: {}", resp.trim()));
    }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    use super::parser::is_ok;
    let resp = t.send_at(&format!("AT+QNETDEVCTL={},2,0", cid))?;
    if !is_ok(&resp) {
        return Err(format!("QNETDEVCTL disconnect failed: {}", resp.trim()));
    }
    Ok(())
}

pub fn query_ip_info(t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at(&format!("AT+QNETDEVSTATUS={}", cid))?;
    let mut info = IpInfo {
        ipv4_addr: String::new(),
        ipv4_mask: String::new(),
        ipv4_gw: String::new(),
        ipv4_dns: String::new(),
        ipv6_addr: String::new(),
        ipv6_dns: String::new(),
    };
    for line in resp.lines() {
        let t2 = line.trim();
        if let Some(rest) = t2.strip_prefix("+QNETDEVSTATUS:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            // Format: <ipv4_addr>,<mask>,<gw>,<empty>,<dns1>,<dns2>,<ipv6_addr>,<empty>,<empty>,<empty>,<v6dns1>,<v6dns2>
            if parts.len() >= 6 {
                info.ipv4_addr = parts.first().unwrap_or(&"").to_string();
                info.ipv4_mask = parts.get(1).unwrap_or(&"").to_string();
                info.ipv4_gw = parts.get(2).unwrap_or(&"").to_string();
                let dns1 = parts.get(4).unwrap_or(&"");
                let dns2 = parts.get(5).unwrap_or(&"");
                let mut dns_parts = Vec::new();
                if !dns1.is_empty() { dns_parts.push(*dns1); }
                if !dns2.is_empty() { dns_parts.push(*dns2); }
                info.ipv4_dns = dns_parts.join(", ");
            }
            if parts.len() >= 12 {
                info.ipv6_addr = parts.get(6).unwrap_or(&"").to_string();
                let v6dns1 = parts.get(10).unwrap_or(&"");
                let v6dns2 = parts.get(11).unwrap_or(&"");
                let mut v6dns_parts = Vec::new();
                if !v6dns1.is_empty() { v6dns_parts.push(*v6dns1); }
                if !v6dns2.is_empty() { v6dns_parts.push(*v6dns2); }
                info.ipv6_dns = v6dns_parts.join(", ");
            }
        }
    }
    Ok(info)
}

pub fn query_traffic(t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
    let resp = t.send_at("AT+QGDCNT?")?;
    for line in resp.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QGDCNT:") {
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
