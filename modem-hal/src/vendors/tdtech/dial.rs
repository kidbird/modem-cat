use crate::transport::AtTransport;
use crate::types::IpInfo;
use super::parser::{hex_ip_to_string, parse_dconnstat};

pub fn connect(
    t: &mut dyn AtTransport,
    cid: i32,
    apn: &str,
    user: &str,
    pass: &str,
    auth: i32,
) -> Result<(), String> {
    let cmd = if apn.is_empty() {
        format!("AT^NDISDUP={},1", cid)
    } else {
        format!("AT^NDISDUP={},1,\"{}\",\"{}\",\"{}\",{}", cid, apn, user, pass, auth)
    };
    let resp = t.send_at(&cmd)?;
    if resp.contains("ERROR") {
        return Err(format!("NDISDUP connect failed: {}", resp));
    }
    // Manual requires 5s before DHCP is available
    std::thread::sleep(std::time::Duration::from_secs(5));
    let stat = t.send_at("AT^DCONNSTAT?")?;
    if !parse_dconnstat(&stat) {
        return Err("Connection not established after NDISDUP".to_string());
    }
    Ok(())
}

pub fn disconnect(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    t.send_at(&format!("AT^NDISDUP={},0", cid))?;
    Ok(())
}

pub fn query_ip(t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at(&format!("AT^DHCP={}", cid))?;
    for line in resp.lines() {
        let ln = line.trim();
        if let Some(rest) = ln.strip_prefix("^DHCP:") {
            // ^DHCP: clip,netmask,gate,dhcp,pDNS,sDNS,max_rx,max_tx
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim()).collect();
            return Ok(IpInfo {
                ipv4_addr: parts.get(0).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv4_mask: parts.get(1).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv4_gw:   parts.get(2).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv4_dns:  parts.get(4).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv6_addr: String::new(),
                ipv6_gw:   String::new(),
                ipv6_dns:  String::new(),
            });
        }
    }
    Err("No DHCP response".to_string())
}
