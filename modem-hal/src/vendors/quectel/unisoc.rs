use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};

pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let resp = t.send_at(&format!("AT+QNETDEVCTL=1,{},1", cid))?;
    if resp.contains("ERROR") { return Err(format!("QNETDEVCTL connect failed: {}", resp)); }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    t.send_at(&format!("AT+QNETDEVCTL=0,{},1", cid))?;
    Ok(())
}

pub fn query_ip_info(t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at(&format!("AT+QNETDEVSTATUS={}", cid))?;
    let mut info = IpInfo {
        ipv4_addr: String::new(), ipv4_mask: String::new(),
        ipv4_gw: String::new(), ipv4_dns: String::new(),
        ipv6_addr: String::new(), ipv6_gw: String::new(), ipv6_dns: String::new(),
    };
    for line in resp.lines() {
        let t2 = line.trim();
        if let Some(rest) = t2.strip_prefix("+QNETDEVSTATUS:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim().trim_matches('"')).collect();
            if parts.len() >= 5 {
                info.ipv4_addr = parts.get(1).unwrap_or(&"").to_string();
                info.ipv4_mask = parts.get(2).unwrap_or(&"").to_string();
                info.ipv4_gw   = parts.get(3).unwrap_or(&"").to_string();
                info.ipv4_dns  = parts.get(4).unwrap_or(&"").to_string();
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
    Ok(TrafficInfo { ul_bytes: 0, dl_bytes: 0 })
}
