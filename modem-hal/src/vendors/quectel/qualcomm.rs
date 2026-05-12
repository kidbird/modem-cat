use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};

pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let resp = t.send_at(&format!("AT+QMAP=\"connect\",{}", cid))?;
    if resp.contains("ERROR") {
        return Err(format!("QMAP connect failed: {}", resp));
    }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    t.send_at(&format!("AT+QMAP=\"disconnect\",{}", cid))?;
    Ok(())
}

pub fn query_ip_info(t: &mut dyn AtTransport, _cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at("AT+QMAP=\"WWAN\"")?;
    let mut info = IpInfo {
        ipv4_addr: String::new(),
        ipv4_mask: String::new(),
        ipv4_gw: String::new(),
        ipv4_dns: String::new(),
        ipv6_addr: String::new(),
        ipv6_gw: String::new(),
        ipv6_dns: String::new(),
    };
    for line in resp.lines() {
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
