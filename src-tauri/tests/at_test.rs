use std::io::{Read, Write};
use std::time::Duration;

fn send_at(port: &mut Box<dyn serialport::SerialPort>, cmd: &str) -> String {
    let mut drain = [0u8; 4096];
    let _ = port.read(&mut drain);
    std::thread::sleep(Duration::from_millis(100));

    match port.write_all(format!("{}\r\n", cmd).as_bytes()) {
        Ok(_) => {}
        Err(e) => return format!("WRITE_ERROR:{}", e),
    }
    port.flush().ok();
    std::thread::sleep(Duration::from_millis(500));

    let mut response = Vec::new();
    let mut buf = [0u8; 4096];
    let start = std::time::Instant::now();
    loop {
        match port.read(&mut buf) {
            Ok(n) => response.extend_from_slice(&buf[..n]),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                if !response.is_empty() && start.elapsed() > Duration::from_millis(500) {
                    break;
                }
                if start.elapsed() > Duration::from_secs(3) {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    String::from_utf8_lossy(&response).to_string()
}

fn is_at_response(text: &str) -> bool {
    let lines: Vec<&str> = text.lines().map(|l| l.trim()).collect();
    lines.iter().any(|l| *l == "OK") && lines.iter().any(|l| *l == "AT" || lines.len() <= 2)
}

#[test]
fn test_at_with_parser() {
    use modem_hal::vendors::quectel::parser::*;

    let ports = serialport::available_ports().unwrap();
    println!("\n=== AT Parser Integration Test ===\n");

    for p in &ports {
        print!("Probing {} ... ", p.port_name);
        let mut port = match serialport::new(&p.port_name, 115200)
            .timeout(Duration::from_millis(500))
            .open()
        {
            Ok(port) => port,
            Err(_e) => {
                println!("OPEN FAILED");
                continue;
            }
        };
        std::thread::sleep(Duration::from_millis(500));
        let resp = send_at(&mut port, "AT");
        if !is_at_response(&resp) {
            println!("No AT");
            continue;
        }
        println!("OK");

        // ── Parse SIM ──
        let cpin = send_at(&mut port, "AT+CPIN?");
        println!("\n[SIM] raw: {}", cpin.trim().replace('\n', " | "));
        let sim = parse_cpin(&cpin);
        println!("[SIM] parsed: {}", sim);

        // ── Parse IMEI ──
        let cgsn = send_at(&mut port, "AT+CGSN");
        let imei = parse_cgsn(&cgsn);
        println!("[IMEI] {}", imei);

        // ── Parse ICCID ──
        let ccid = send_at(&mut port, "AT+CCID");
        let iccid = parse_iccid(&ccid);
        println!("[ICCID] {}", iccid);

        // ── Parse hardware ──
        let cgmi = send_at(&mut port, "AT+CGMI");
        let mfr = parse_cgmm(&cgmi);
        let cgmm = send_at(&mut port, "AT+CGMM");
        let model = parse_cgmm(&cgmm);
        let cgmr = send_at(&mut port, "AT+GMR");
        let fw = parse_gmr(&cgmr);
        println!("[HW] {} {} fw={}", mfr, model, fw);

        // ── Parse operator ──
        let cops = send_at(&mut port, "AT+COPS?");
        let (op, act) = parse_cops_with_act(&cops);
        println!("[Operator] {} ({})", op, act);

        // ── Parse serving cell ──
        let qeng = send_at(&mut port, "AT+QENG=\"servingcell\"");
        println!("\n[QENG] raw: {}", qeng.trim().replace('\n', " | "));
        let sc = parse_qeng_serving_cell(&qeng, true);
        println!(
            "[Cell] tech={} connected={} mcc={} mnc={} cell={} pci={}",
            sc.tech, sc.connected, sc.operator_mcc, sc.operator_mnc, sc.cell_id, sc.pci
        );
        println!(
            "[Cell] arfcn={} band={} bw={} rsrp={} rsrq={} sinr={}",
            sc.arfcn, sc.band, sc.bandwidth, sc.rsrp, sc.rsrq, sc.sinr
        );

        // ── Parse antennas ──
        let antrssi = send_at(&mut port, "AT+QANTRSSI?");
        let ants = parse_qantrssi(&antrssi);
        println!(
            "[ANT] {:?} (raw: {})",
            ants,
            antrssi.trim().replace('\n', " | ")
        );

        // ── Parse APN ──
        let qicsgp = send_at(&mut port, "AT+QICSGP?");
        let apns = parse_qicsgp(&qicsgp, &[]);
        for a in &apns {
            println!(
                "[APN] cid={} name={} type={} auth={}",
                a.cid, a.apn_name, a.ip_type, a.auth_type
            );
        }

        // ── Parse CGACT ──
        let cgact = send_at(&mut port, "AT+CGACT?");
        let acts = parse_cgact(&cgact);
        for (cid, st) in &acts {
            println!("[CGACT] cid={} status={}", cid, st);
        }

        // ── Parse IP ──
        let ipresp = send_at(&mut port, "AT+QNETDEVSTATUS=1");
        let mut mock = modem_hal::transport::MockTransport::new(vec![&ipresp]);
        let ip = modem_hal::vendors::quectel::unisoc::query_ip_info(&mut mock, 1)
            .unwrap_or_default();
        println!("[IP] addr={} mask={} gw={} dns={}", ip.ipv4_addr, ip.ipv4_mask, ip.ipv4_gw, ip.ipv4_dns);
        if !ip.ipv6_addr.is_empty() {
            println!("[IP] ipv6={} dns6={}", ip.ipv6_addr, ip.ipv6_dns);
        }

        // ── Parse QoS ──
        let qosresp = send_at(&mut port, "AT+C5GQOSRDP=1");
        let (cqi, ul_bw, dl_bw) = parse_c5gqosrdp(&qosresp);
        println!("[QoS] 5qi={} UL={} DL={}", cqi, ul_bw, dl_bw);

        // ── Parse network mode ──
        let mode = send_at(&mut port, "AT+QNWPREFCFG=\"mode_pref\"");
        let pref = parse_qnwprefcfg_mode(&mode);
        println!("[NetMode] {}", pref);

        println!("\n=== Test Complete ===");
        return;
    }
    println!("\n!!! No AT port found !!!");
}