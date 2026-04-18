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
                if !response.is_empty() && start.elapsed() > Duration::from_millis(500) { break; }
                if start.elapsed() > Duration::from_secs(3) { break; }
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
    let ports = serialport::available_ports().unwrap();
    println!("\n=== AT Parser Integration Test ===\n");

    for p in &ports {
        print!("Probing {} ... ", p.port_name);
        let mut port = match serialport::new(&p.port_name, 115200)
            .timeout(Duration::from_millis(500)).open() {
            Ok(port) => port,
            Err(e) => { println!("OPEN FAILED"); continue; }
        };
        std::thread::sleep(Duration::from_millis(500));
        let resp = send_at(&mut port, "AT");
        if !is_at_response(&resp) { println!("No AT"); continue; }
        println!("OK");

        // ── Parse SIM ──
        let cpin = send_at(&mut port, "AT+CPIN?");
        println!("\n[SIM] raw: {}", cpin.trim().replace('\n', " | "));
        let sim = modem_cat_lib::at_parser::parse_cpin(&cpin);
        println!("[SIM] parsed: {}", sim);

        // ── Parse IMEI ──
        let cgsn = send_at(&mut port, "AT+CGSN");
        let imei = modem_cat_lib::at_parser::parse_cgsn(&cgsn);
        println!("[IMEI] {}", imei);

        // ── Parse ICCID ──
        let ccid = send_at(&mut port, "AT+CCID");
        let iccid = modem_cat_lib::at_parser::parse_ccid(&ccid);
        println!("[ICCID] {}", iccid);

        // ── Parse hardware ──
        let cgmi = send_at(&mut port, "AT+CGMI");
        let mfr = modem_cat_lib::at_parser::parse_cgmi(&cgmi);
        let cgmm = send_at(&mut port, "AT+CGMM");
        let model = modem_cat_lib::at_parser::parse_cgmm(&cgmm);
        let cgmr = send_at(&mut port, "AT+CGMR");
        let fw = modem_cat_lib::at_parser::parse_cgmr(&cgmr);
        println!("[HW] {} {} fw={}", mfr, model, fw);

        // ── Parse operator ──
        let cops = send_at(&mut port, "AT+COPS?");
        let (op, act) = modem_cat_lib::at_parser::parse_cops(&cops);
        println!("[Operator] {} ({})", op, act);

        // ── Parse serving cell ──
        let qeng = send_at(&mut port, "AT+QENG=\"servingcell\"");
        println!("\n[QENG] raw: {}", qeng.trim().replace('\n', " | "));
        if let Some(sc) = modem_cat_lib::at_parser::parse_qeng_servingcell(&qeng) {
            println!("[Cell] tech={} connected={} mcc={} mnc={} cell={} pci={}",
                sc.tech, sc.connected, sc.operator_mcc, sc.operator_mnc, sc.cell_id, sc.pci);
            println!("[Cell] arfcn={} band={} bw={} rsrp={} rsrq={} sinr={}",
                sc.arfcn, sc.band, sc.bandwidth, sc.rsrp, sc.rsrq, sc.sinr);
        } else {
            println!("[Cell] PARSE FAILED");
        }

        // ── Parse antennas ──
        let antrssi = send_at(&mut port, "AT+QANTRSSI?");
        let ants = modem_cat_lib::at_parser::parse_qantrssi(&antrssi);
        println!("[ANT] {:?} (raw: {})", ants, antrssi.trim().replace('\n', " | "));

        // ── Parse APN ──
        let qicsgp = send_at(&mut port, "AT+QICSGP?");
        let apns = modem_cat_lib::at_parser::parse_qicsgp(&qicsgp);
        for a in &apns {
            println!("[APN] cid={} name={} type={} auth={}", a.cid, a.apn_name, a.ip_type, a.auth_type);
        }

        // ── Parse CGACT ──
        let cgact = send_at(&mut port, "AT+CGACT?");
        let acts = modem_cat_lib::at_parser::parse_cgact(&cgact);
        for (cid, st) in &acts {
            println!("[CGACT] cid={} status={}", cid, st);
        }

        // ── Parse IP ──
        let ipresp = send_at(&mut port, "AT+QNETDEVSTATUS=1");
        let (ip4, mask, gw, dns, ip6) = modem_cat_lib::at_parser::parse_qnetdevstatus(&ipresp);
        println!("[IP] addr={} mask={} gw={} dns={}", ip4, mask, gw, dns);
        if !ip6.is_empty() { println!("[IP] ipv6={}", ip6); }

        // ── Parse QoS ──
        let qosresp = send_at(&mut port, "AT+C5GQOSRDP=1");
        let (cqi, ul_bw, dl_bw) = modem_cat_lib::at_parser::parse_c5gqosrdp(&qosresp);
        println!("[QoS] 5qi={} UL={} DL={}", cqi, ul_bw, dl_bw);

        // ── Parse network mode ──
        let mode = send_at(&mut port, "AT+QNWPREFCFG=\"mode_pref\"");
        let pref = modem_cat_lib::at_parser::parse_qnwprefcfg_mode(&mode);
        println!("[NetMode] {}", pref);

        println!("\n=== Test Complete ===");
        return;
    }
    println!("\n!!! No AT port found !!!");
}
