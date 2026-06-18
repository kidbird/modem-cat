use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};

pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    use super::parser::is_ok;
    // 高通手册 §12.10: AT+QMAP="connect",<rule_num>,<connect>  (<connect>: 1=发起, 0=终止)。
    // 原代码缺少 <connect> 标志位。<rule_num> 为多数据呼叫规则号(0~3)，此处沿用 cid；
    // 若硬件固定使用规则 0，请在真机验证后将 rule_num 改为 0。
    let resp = t.send_at(&format!("AT+QMAP=\"connect\",{},1", cid))?;
    if !is_ok(&resp) {
        return Err(format!("QMAP connect failed: {}", resp.trim()));
    }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    use super::parser::is_ok;
    // 高通手册 §12.10: 终止数据呼叫为 AT+QMAP="connect",<rule_num>,0
    // QMAP 没有 "disconnect" 子命令(§12.1~12.16)，原写法会被模组返回 ERROR。
    let resp = t.send_at(&format!("AT+QMAP=\"connect\",{},0", cid))?;
    if !is_ok(&resp) {
        return Err(format!("QMAP disconnect failed: {}", resp.trim()));
    }
    Ok(())
}

/// Parse `AT+QMAP="MPDN_rule"` query response — return IPPT mode for rule 0.
/// Response: +QMAP: "MPDN_rule",<rule_num>,<profileID>,<p3>,<IPPT_mode>,<auto_connect>
/// Returns: raw IPPT mode (e.g. 0=关闭/Disabled, 1=ETH, 3=USB)
pub fn parse_mpdn_ippt_mode(response: &str) -> i32 {
    for line in response.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 5
                && parts[0].eq_ignore_ascii_case("MPDN_rule")
                && parts[1].trim() == "0"
            {
                let profile_id: i32 = parts[2].trim().parse().unwrap_or(0);
                if profile_id == 0 {
                    return 0;
                }
                let ippt_mode: i32 = parts[4].trim().parse().unwrap_or(0);
                return ippt_mode;
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
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            // parts[0]="MPDN_status", parts[1]=rule_num, parts[2]=profileID, parts[3]=IPPT_status, parts[4]=connect_status
            if parts.len() >= 5
                && parts[0].eq_ignore_ascii_case("MPDN_status")
                && parts[1].trim() == "0"
            {
                return parts[4].trim() == "1";
            }
        }
    }
    false
}

/// Parse `AT+QMAP="ETH_PDU"` response — return true if ETH PDU is enabled.
pub fn parse_eth_pdu_enabled(response: &str) -> bool {
    for line in response.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.first().map(|s| s.eq_ignore_ascii_case("ETH_PDU")) == Some(true) {
                return parts
                    .get(1)
                    .map(|s| s.eq_ignore_ascii_case("enable"))
                    .unwrap_or(false);
            }
        }
    }
    false
}

/// Parse `AT+QMAP="mpdn_rule"` response — return CID for the given rule_id.
pub fn parse_mpdn_rule_cid(response: &str, rule_id: i32) -> Option<i32> {
    for line in response.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 3
                && parts[0].eq_ignore_ascii_case("MPDN_rule")
                && parts[1].trim().parse::<i32>().unwrap_or(-1) == rule_id
            {
                return parts[2].trim().parse::<i32>().ok();
            }
        }
    }
    None
}

/// Parse `AT+QMAP="MPDN_status"` response — return connect_status for the given rule_id.
pub fn parse_mpdn_connect_status_by_rule(response: &str, rule_id: i32) -> bool {
    for line in response.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 5
                && parts[0].eq_ignore_ascii_case("MPDN_status")
                && parts[1].trim().parse::<i32>().unwrap_or(-1) == rule_id
            {
                return parts[4].trim() == "1";
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

fn parse_qmap_wwan(response: &str) -> IpInfo {
    let mut info = ip_info_default();
    for line in response.lines() {
        let t2 = line.trim();
        if let Some(rest) = t2.strip_prefix("+QMAP: \"WWAN\",") {
            let parts: Vec<&str> = rest
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            // 手册 §12.2: +QMAP: "WWAN",<status>,<profileID>,<IP_family>,<IP_address>
            // strip_prefix 已消费 "WWAN"，剩 4 个字段: [0]status [1]profileID [2]family [3]addr。
            // 原代码误用 len>=5 + [3]/[4]，导致本快路径恒不命中而始终回退 CGPADDR。
            if parts.len() >= 4 {
                let family = parts.get(2).unwrap_or(&"");
                let addr = parts.get(3).unwrap_or(&"");
                if *addr == "0.0.0.0" || *addr == "0:0:0:0:0:0:0:0" {
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

pub fn query_ip_info(t: &mut dyn AtTransport, _data_cid: i32) -> Result<IpInfo, String> {
    let qmap_resp = t.send_at("AT+QMAP=\"WWAN\"")?;
    Ok(parse_qmap_wwan(&qmap_resp))
}

/// Set auto-connect for a QMAP data call rule.
/// `AT+QMAP="auto_connect",<rule_num>,<auto_connect>[,<profile_id>]`
/// auto_connect: 0=disabled, 1=enabled
pub fn set_auto_connect(
    t: &mut dyn AtTransport,
    rule_num: i32,
    auto_connect: i32,
    profile_id: Option<i32>,
) -> Result<(), String> {
    use super::parser::is_ok;
    let cmd = if let Some(pid) = profile_id {
        format!(
            "AT+QMAP=\"auto_connect\",{},{},{}",
            rule_num, auto_connect, pid
        )
    } else {
        format!("AT+QMAP=\"auto_connect\",{},{}", rule_num, auto_connect)
    };
    let resp = t.send_at(&cmd)?;
    if !is_ok(&resp) {
        return Err(format!("QMAP auto_connect set failed: {}", resp.trim()));
    }
    Ok(())
}

/// Parse `AT+QMAP="auto_connect"` query response — return auto_connect state for rule 0.
/// Response: +QMAP: "auto_connect",<rule_num>,<auto_connect>
pub fn parse_auto_connect(response: &str) -> i32 {
    for line in response.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QMAP:") {
            let parts: Vec<&str> = rest
                .trim()
                .split(',')
                .map(|s| s.trim().trim_matches('"'))
                .collect();
            if parts.len() >= 3
                && parts[0].eq_ignore_ascii_case("auto_connect")
                && parts[1].trim() == "0"
            {
                return parts[2].trim().parse().unwrap_or(0);
            }
        }
    }
    0
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    #[test]
    fn connect_data_uses_qmap_connect_with_flag() {
        let mut t = MockTransport::new(vec!["OK"]);
        connect_data(&mut t, 1).unwrap();
        assert_eq!(t.sent, vec![r#"AT+QMAP="connect",1,1"#]);
    }

    #[test]
    fn disconnect_data_uses_qmap_connect_zero_not_disconnect() {
        let mut t = MockTransport::new(vec!["OK"]);
        disconnect_data(&mut t, 1).unwrap();
        assert_eq!(t.sent, vec![r#"AT+QMAP="connect",1,0"#]);
    }

    // 手册 §12.2: +QMAP: "WWAN",<status>,<profileID>,<IP_family>,<IP_address>
    #[test]
    fn parse_qmap_wwan_extracts_ipv4_after_prefix_consumed() {
        let resp = "+QMAP: \"WWAN\",0,1,\"IPV4\",\"10.1.2.3\"\r\nOK";
        let info = parse_qmap_wwan(resp);
        assert_eq!(info.ipv4_addr, "10.1.2.3");
        assert!(info.ipv6_addr.is_empty());
    }

    #[test]
    fn parse_qmap_wwan_skips_all_zero_addresses() {
        let resp = "+QMAP: \"WWAN\",0,1,\"IPV4\",\"0.0.0.0\"\r\n\
                    +QMAP: \"WWAN\",0,1,\"IPV6\",\"0:0:0:0:0:0:0:0\"\r\nOK";
        let info = parse_qmap_wwan(resp);
        assert!(info.ipv4_addr.is_empty());
        assert!(info.ipv6_addr.is_empty());
    }

    #[test]
    fn test_parse_mpdn_ippt_mode() {
        let resp_disabled = "+QMAP: \"MPDN_rule\",0,0,0,0,0,\"00:00:00:00:00:00\"\r\nOK";
        assert_eq!(parse_mpdn_ippt_mode(resp_disabled), 0);

        let resp_eth = "+QMAP: \"MPDN_rule\",0,1,0,1,1,\"FF:FF:FF:FF:FF:FF\"\r\nOK";
        assert_eq!(parse_mpdn_ippt_mode(resp_eth), 1);

        let resp_usb = "+QMAP: \"MPDN_rule\",0,1,0,3,1,\"FF:FF:FF:FF:FF:FF\"\r\nOK";
        assert_eq!(parse_mpdn_ippt_mode(resp_usb), 3);
    }

    #[test]
    fn query_ip_info_uses_single_qmap_path() {
        let mut transport = crate::transport::MockTransport::new(vec![
            "+QMAP: \"WWAN\",0,1,\"IPV4\",\"10.0.0.8\"\r\nOK",
        ]);
        let info = query_ip_info(&mut transport, 1).expect("query_ip_info should succeed");

        assert_eq!(transport.sent, vec!["AT+QMAP=\"WWAN\""]);
        assert_eq!(info.ipv4_addr, "10.0.0.8");
    }
}
