use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::Ordering;
use std::time::Duration;

use modem_hal::transport::AtTransport;
use modem_hal::ModemFactory;

use crate::{wrap_transport, AppState};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// ── Port listing helpers ──

#[derive(Debug, Clone, Default)]
struct WindowsPortInfo {
    description: Option<String>,
    manufacturer: Option<String>,
    usb_vid: Option<u16>,
    usb_pid: Option<u16>,
}

#[derive(Debug, Clone, Default)]
struct ResolvedPortInfo {
    description: Option<String>,
    manufacturer: Option<String>,
    usb_vid: Option<u16>,
    usb_pid: Option<u16>,
    detected_model: Option<String>,
    detected_chipset: Option<String>,
}

fn extract_hex_id_from_identifier(identifier: &str, marker: &str) -> Option<u16> {
    let upper = identifier.to_uppercase();
    let start = upper.find(marker)? + marker.len();
    let hex: String = upper[start..]
        .chars()
        .take_while(|c| c.is_ascii_hexdigit())
        .take(4)
        .collect();
    if hex.len() != 4 {
        return None;
    }
    u16::from_str_radix(&hex, 16).ok()
}

fn parse_vid_pid_from_identifier(identifier: &str) -> (Option<u16>, Option<u16>) {
    (
        extract_hex_id_from_identifier(identifier, "VID_"),
        extract_hex_id_from_identifier(identifier, "PID_"),
    )
}


#[cfg(target_os = "windows")]
fn get_windows_all_port_info() -> HashMap<String, WindowsPortInfo> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    let mut result = HashMap::new();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let enum_key = match hklm.open_subkey(r"SYSTEM\CurrentControlSet\Enum") {
        Ok(k) => k,
        Err(_) => return result,
    };

    for bus in enum_key.enum_keys().flatten() {
        let bus_key = match enum_key.open_subkey(&bus) {
            Ok(k) => k,
            Err(_) => continue,
        };

        for device in bus_key.enum_keys().flatten() {
            let device_key = match bus_key.open_subkey(&device) {
                Ok(k) => k,
                Err(_) => continue,
            };

            for func in device_key.enum_keys().flatten() {
                let func_key = match device_key.open_subkey(&func) {
                    Ok(k) => k,
                    Err(_) => continue,
                };

                let dp_key = match func_key.open_subkey("Device Parameters") {
                    Ok(k) => k,
                    Err(_) => continue,
                };

                let port_name: String = match dp_key.get_value("PortName") {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let friendly_name: Option<String> = func_key.get_value("FriendlyName").ok();
                let manufacturer: Option<String> = func_key.get_value("Manufacturer").ok();
                let (mut usb_vid, mut usb_pid) = parse_vid_pid_from_identifier(&device);

                // Fallback: read HardwareID multi-string value from device or func key
                if usb_vid.is_none() || usb_pid.is_none() {
                    let hw_ids: Option<Vec<String>> = device_key
                        .get_value("HardwareID")
                        .ok()
                        .or_else(|| func_key.get_value("HardwareID").ok());
                    if let Some(ids) = hw_ids {
                        for hw_id in &ids {
                            if usb_vid.is_none() || usb_pid.is_none() {
                                let (v, p) = parse_vid_pid_from_identifier(hw_id);
                                if usb_vid.is_none() {
                                    usb_vid = v;
                                }
                                if usb_pid.is_none() {
                                    usb_pid = p;
                                }
                            }
                        }
                    }
                }

                result.insert(
                    port_name,
                    WindowsPortInfo {
                        description: friendly_name,
                        manufacturer,
                        usb_vid,
                        usb_pid,
                    },
                );
            }
        }
    }

    result
}

#[cfg(not(target_os = "windows"))]
fn get_windows_all_port_info() -> HashMap<String, WindowsPortInfo> {
    HashMap::new()
}

fn resolve_port_info(
    port: &serialport::SerialPortInfo,
    win_info: &HashMap<String, WindowsPortInfo>,
) -> ResolvedPortInfo {
    let mut description: Option<String> = None;
    let mut manufacturer: Option<String> = None;
    let mut usb_vid: Option<u16> = None;
    let mut usb_pid: Option<u16> = None;

    if let serialport::SerialPortType::UsbPort(info) = &port.port_type {
        description = info.product.clone();
        manufacturer = info.manufacturer.clone();
        usb_vid = Some(info.vid);
        usb_pid = Some(info.pid);
        log::debug!(
            "resolve_port_info [{}]: UsbPort vid={:#04x}, pid={:#04x}",
            port.port_name,
            info.vid,
            info.pid
        );
    } else {
        log::debug!(
            "resolve_port_info [{}]: not UsbPort (type={:?})",
            port.port_name,
            port.port_type
        );
    }

    if let Some(info) = win_info.get(&port.port_name) {
        if info.description.is_some() {
            description = info.description.clone();
        }
        if info.manufacturer.is_some() && manufacturer.is_none() {
            manufacturer = info.manufacturer.clone();
        }
        if usb_vid.is_none() {
            usb_vid = info.usb_vid;
            log::debug!(
                "resolve_port_info [{}]: registry fallback vid={:#06x?}",
                port.port_name,
                usb_vid
            );
        }
        if usb_pid.is_none() {
            usb_pid = info.usb_pid;
            log::debug!(
                "resolve_port_info [{}]: registry fallback pid={:#06x?}",
                port.port_name,
                usb_pid
            );
        }
    }

    log::debug!(
        "resolve_port_info [{}]: final vid={:#06x?}, pid={:#06x?}",
        port.port_name,
        usb_vid,
        usb_pid
    );

    let detected_model = usb_vid
        .zip(usb_pid)
        .and_then(|(vid, pid)| ModemFactory::detect_model_from_vid_pid(vid, pid))
        .map(str::to_string);
    let detected_chipset = usb_vid
        .zip(usb_pid)
        .and_then(|(vid, pid)| ModemFactory::detect_vendor_from_vid_pid(vid, pid))
        .map(|vendor| vendor.as_str().to_string());

    ResolvedPortInfo {
        description,
        manufacturer,
        usb_vid,
        usb_pid,
        detected_model,
        detected_chipset,
    }
}

fn lookup_usb_ids_for_port(port_name: &str) -> Option<(u16, u16)> {
    let ports = serialport::available_ports().ok()?;
    let win_info = get_windows_all_port_info();
    let port = ports.into_iter().find(|port| port.port_name == port_name)?;
    let resolved = resolve_port_info(&port, &win_info);
    resolved.usb_vid.zip(resolved.usb_pid)
}

pub(crate) fn snapshot_ports() -> Result<Vec<modem_hal::types::PortInfo>, String> {
    let ports =
        serialport::available_ports().map_err(|e| format!("Failed to list ports: {}", e))?;

    let win_info = get_windows_all_port_info();

    let result: Vec<modem_hal::types::PortInfo> = ports
        .into_iter()
        .map(|port| {
            let resolved = resolve_port_info(&port, &win_info);

            let is_at_port = is_at_port(
                &port.port_name,
                &resolved.description.as_ref(),
                &resolved.manufacturer.as_ref(),
            );

            let display_name = build_display_name(
                &port.port_name,
                &resolved.description,
                &resolved.manufacturer,
                is_at_port,
            );

            modem_hal::types::PortInfo {
                port_name: port.port_name,
                description: resolved.description,
                manufacturer: resolved.manufacturer,
                usb_vid: resolved.usb_vid,
                usb_pid: resolved.usb_pid,
                detected_model: resolved.detected_model,
                detected_chipset: resolved.detected_chipset,
                is_at_port,
                display_name,
            }
        })
        .collect();

    Ok(result)
}

#[tauri::command]
pub(crate) fn list_ports() -> Result<Vec<modem_hal::types::PortInfo>, String> {
    snapshot_ports()
}

fn build_display_name(
    port_name: &str,
    description: &Option<String>,
    manufacturer: &Option<String>,
    is_at_port: bool,
) -> String {
    if let Some(desc) = description {
        if !desc.is_empty() {
            let cleaned = regex_remove_com_suffix(desc);
            if is_at_port {
                return format!("{} ({}) [AT]", cleaned, port_name);
            }
            return format!("{} ({})", cleaned, port_name);
        }
    }

    let mut parts = Vec::new();
    if let Some(mfg) = manufacturer {
        if !mfg.is_empty() {
            parts.push(mfg.clone());
        }
    }

    if is_at_port {
        if parts.is_empty() {
            format!("{} - AT端口", port_name)
        } else {
            format!("{} ({}) - AT端口", parts.join(" - "), port_name)
        }
    } else if parts.is_empty() {
        port_name.to_string()
    } else {
        format!("{} ({})", parts.join(" - "), port_name)
    }
}

fn regex_remove_com_suffix(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.ends_with(')') {
        if let Some(open) = trimmed.rfind('(') {
            let inside = &trimmed[open + 1..trimmed.len() - 1];
            if inside.starts_with("COM") && inside[3..].chars().all(|c| c.is_ascii_digit()) {
                return trimmed[..open].trim().to_string();
            }
        }
    }
    s.to_string()
}

pub(crate) fn is_at_port(
    _port_name: &str,
    description: &Option<&String>,
    manufacturer: &Option<&String>,
) -> bool {
    let desc_upper = description.map_or_else(String::new, |s| s.to_uppercase());
    let mfg_upper = manufacturer.map_or_else(String::new, |s| s.to_uppercase());

    if desc_upper
        .split(|c: char| !c.is_alphabetic())
        .any(|word| word == "AT")
    {
        return true;
    }

    let is_modem_manufacturer = mfg_upper.contains("QUECTEL")
        || mfg_upper.contains("QUALCOMM")
        || mfg_upper.contains("SIERRA")
        || mfg_upper.contains("FIBOCOM")
        || mfg_upper.contains("ZTE")
        || mfg_upper.contains("HUAWEI")
        || mfg_upper.contains("SIMCOM")
        || mfg_upper.contains("TELIT")
        || mfg_upper.contains("U-BLOX")
        || mfg_upper.contains("THALES")
        || mfg_upper.contains("MOBILE")
        || mfg_upper.contains("BROADMOBI");

    if is_modem_manufacturer {
        let desc_words: Vec<&str> = desc_upper.split(|c: char| !c.is_alphabetic()).collect();
        if desc_upper.contains("NMEA")
            || desc_upper.contains("DIAG")
            || desc_upper.contains("DEBUG")
            || desc_upper.contains("QDLOADER")
            || desc_words.contains(&"DM")
        {
            return false;
        }
        if desc_upper.contains("MODEM") || desc_upper.contains("COMMAND") {
            return true;
        }
        if desc_upper.is_empty() {
            return true;
        }
    }

    false
}

// ── Auto-connect ──

const AT_PROBE_TIMEOUT: Duration = Duration::from_secs(2);

pub(crate) async fn run_after_at_probe_timeout<T, U, Probe, Detect, DetectFuture>(
    probe_timeout: Duration,
    at_probe: Probe,
    detect_vendor: Detect,
) -> Result<U, String>
where
    Probe: Future<Output = Result<T, String>>,
    Detect: FnOnce(T) -> DetectFuture,
    DetectFuture: Future<Output = Result<U, String>>,
{
    let transport = match tokio::time::timeout(probe_timeout, at_probe).await {
        Ok(result) => result?,
        Err(_) => return Err(format!("AT probe timed out after {:?}", probe_timeout)),
    };

    detect_vendor(transport).await
}

#[tauri::command]
pub(crate) async fn auto_connect_at(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let ports =
        serialport::available_ports().map_err(|e| format!("Failed to list ports: {}", e))?;

    let win_info = get_windows_all_port_info();

    let mut at_candidates: Vec<(String, Option<(u16, u16)>)> = Vec::new();

    for port in &ports {
        let resolved = resolve_port_info(port, &win_info);

        if is_at_port(
            &port.port_name,
            &resolved.description.as_ref(),
            &resolved.manufacturer.as_ref(),
        ) {
            let usb_ids = resolved.usb_vid.zip(resolved.usb_pid);
            log::debug!(
                "auto_connect: candidate {} vid={:#06x?} pid={:#06x?}",
                port.port_name,
                resolved.usb_vid,
                resolved.usb_pid
            );
            at_candidates.push((port.port_name.clone(), usb_ids));
        }
    }

    if at_candidates.is_empty() {
        return Err("未找到AT端口".to_string());
    }

    // 首选端口：名称或描述里带 "AT" 的串口（典型 Quectel 多口中，AT 口响应
    // `AT` 最快，诊断/NMEA 口会拖慢或失败）。stable-sort 保持同组内原有 COM 号顺序。
    at_candidates.sort_by(|a, b| {
        let a_at = a.0.to_uppercase().contains("AT")
            || win_info
                .get(&a.0)
                .and_then(|info| info.description.as_ref())
                .map(|s| s.to_uppercase().contains("AT"))
                .unwrap_or(false);
        let b_at = b.0.to_uppercase().contains("AT")
            || win_info
                .get(&b.0)
                .and_then(|info| info.description.as_ref())
                .map(|s| s.to_uppercase().contains("AT"))
                .unwrap_or(false);
        b_at.cmp(&a_at)
    });

    log::info!("AT candidates (AT-preferred): {:?}", at_candidates);

    let mut handles = Vec::with_capacity(at_candidates.len());
    for (port_name, usb_ids) in &at_candidates {
        let pn = port_name.clone();
        let usb_ids = *usb_ids;
        log::info!("Probing port: {}", pn);
        handles.push(tokio::spawn(async move {
            let pn_block = pn.clone();
            let pn_probe = pn.clone();
            let pn_detect = pn.clone();
            let probe = tokio::task::spawn_blocking(
                move || -> Result<modem_hal::transport::SerialTransport, String> {
                    let mut transport =
                        modem_hal::transport::SerialTransport::new(&pn_block, 115200)?;
                    let response = transport.send_at("AT");
                    match response {
                        Ok(r) if r.trim().ends_with("OK") => Ok(transport),
                        Ok(r) => Err(format!("Port {} responded but not OK: {}", pn_block, r)),
                        Err(e) => Err(format!("Port {} AT probe failed: {}", pn_block, e)),
                    }
                },
            );
            let result = run_after_at_probe_timeout(
                AT_PROBE_TIMEOUT,
                async move {
                    probe
                        .await
                        .map_err(|e| format!("Port {} probe task panicked: {}", pn_probe, e))?
                },
                move |mut transport| async move {
                    tokio::task::spawn_blocking(move || {
                        let vendor_result =
                            ModemFactory::create_with_usb_ids(&mut transport, usb_ids);
                        Ok((transport, usb_ids, vendor_result))
                    })
                    .await
                    .map_err(|e| {
                        format!("Port {} vendor detection task panicked: {}", pn_detect, e)
                    })?
                },
            )
            .await;
            (pn, result)
        }));
    }

    let transport_arc = state.transport.clone();
    let vendor_arc = state.vendor.clone();
    let at_log = state.at_command_log.clone();
    let connected_port_arc = state.connected_port.clone();
    let connected_usb_ids_arc = state.connected_usb_ids.clone();

    for h in handles {
        let (pn, probe_result) = match h.await {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Probe task join error: {}", e);
                continue;
            }
        };
        let (transport, usb_ids, vendor_result) = match probe_result {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to open/probe {}: {}", pn, e);
                continue;
            }
        };
        match vendor_result {
            Ok(vendor) => {
                log::info!(
                    "Connected to AT port: {} (vendor: {:?})",
                    pn,
                    vendor.vendor()
                );
                *transport_arc
                    .lock()
                    .map_err(|e| format!("Lock poisoned: {}", e))? =
                    Some(wrap_transport(Box::new(transport), at_log.clone()));
                *vendor_arc
                    .lock()
                    .map_err(|e| format!("Lock poisoned: {}", e))? = Some(vendor);
                *connected_port_arc
                    .lock()
                    .map_err(|e| format!("Lock poisoned: {}", e))? = Some(pn.clone());

                *connected_usb_ids_arc
                    .lock()
                    .map_err(|e| format!("Lock poisoned: {}", e))? = usb_ids;
                log::debug!("auto_connect: stored connected_usb_ids = {:?}", usb_ids);
                return Ok(pn);
            }
            Err(e) => {
                log::warn!("Port {} AT ok but vendor detection failed: {}", pn, e);
                continue;
            }
        }
    }

    let attempted_ports: Vec<String> = at_candidates
        .into_iter()
        .map(|(port_name, _)| port_name)
        .collect();
    Err(format!("所有候选端口均无法打开: {:?}", attempted_ports))
}

// ── Connection management ──

#[tauri::command]
pub(crate) async fn connect_serial(
    port_name: String,
    baud_rate: u32,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport_state = state.transport.clone();
    let vendor_state = state.vendor.clone();
    let conn_port_state = state.connected_port.clone();
    let conn_usb_state = state.connected_usb_ids.clone();
    let at_log_state = state.at_command_log.clone();

    let port_name_clone = port_name.clone();
    tokio::task::spawn_blocking(move || {
        let usb_ids = lookup_usb_ids_for_port(&port_name_clone);
        let mut transport =
            modem_hal::transport::SerialTransport::new(&port_name_clone, baud_rate)?;
        let vendor = ModemFactory::create_with_usb_ids(&mut transport, usb_ids);
        let id = format!("serial_{}", port_name_clone);

        let v = match vendor {
            Ok(v) => v,
            Err(e) => return Err(format!("连接成功但无法识别模组型号: {}", e)),
        };

        *transport_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? =
            Some(wrap_transport(Box::new(transport), at_log_state));

        log::info!("Detected vendor: {:?}", v.vendor());
        *vendor_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = Some(v);

        *conn_port_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = Some(port_name_clone.clone());
        *conn_usb_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = usb_ids;
        log::info!(
            "Connected to serial port {} at {} baud",
            port_name_clone,
            baud_rate
        );
        Ok(id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub(crate) async fn connect_tcp(
    host: String,
    port: u16,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport_state = state.transport.clone();
    let vendor_state = state.vendor.clone();
    let conn_port_state = state.connected_port.clone();
    let conn_usb_state = state.connected_usb_ids.clone();
    let at_log_state = state.at_command_log.clone();

    let host_clone = host.clone();
    tokio::task::spawn_blocking(move || {
        let mut transport = modem_hal::transport::TcpTransport::new(&host_clone, port)?;
        let vendor = ModemFactory::create(&mut transport);
        let id = format!("tcp_{}:{}", host_clone, port);

        let v = match vendor {
            Ok(v) => v,
            Err(e) => return Err(format!("连接成功但无法识别模组型号: {}", e)),
        };

        *conn_port_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *conn_usb_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *transport_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? =
            Some(wrap_transport(Box::new(transport), at_log_state));

        log::info!("Detected vendor: {:?}", v.vendor());
        *vendor_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = Some(v);

        log::info!("Connected to TCP {}:{}", host_clone, port);
        Ok(id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub(crate) async fn disconnect(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let connected_port = state.connected_port.clone();
    let connected_usb_ids = state.connected_usb_ids.clone();
    let data_cid = state.data_cid.clone();

    tokio::task::spawn_blocking(move || {
        let mut t = transport
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        if let Some(ref mut transport) = *t {
            transport.close();
        }
        *t = None;
        *vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *connected_port
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *connected_usb_ids
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = None;
        data_cid.store(1, Ordering::Relaxed);
        Ok("Disconnected".to_string())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NetworkAdapter {
    pub name: String,
    pub description: String,
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    pub gateway: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, PartialEq, Eq)]
struct ProcessLaunchPlan {
    program: String,
    args: Vec<String>,
    hide_window: bool,
}

#[cfg(target_os = "windows")]
fn windows_network_adapter_query_plan() -> ProcessLaunchPlan {
    ProcessLaunchPlan {
        program: "powershell".to_string(),
        args: vec![
            "-NoProfile".to_string(),
            "-Command".to_string(),
            r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8;
Get-NetAdapter | Where-Object {
    $_.Status -eq 'Up' -and
    $_.Virtual -eq $false -and
    $_.InterfaceDescription -notmatch 'Wi-Fi|Wireless|WLAN|802\.11|Bluetooth|Loopback|TAP|VPN|Virtual|VMware|VirtualBox|Hyper-V|Wintun|Tunnel'
} | ForEach-Object {
    $config = Get-NetIPConfiguration -InterfaceIndex $_.InterfaceIndex;
    [PSCustomObject]@{
        Name = $_.Name;
        InterfaceDescription = $_.InterfaceDescription;
        IPv4Address = $config.IPv4Address.IPAddress;
        IPv4DefaultGateway = $config.IPv4DefaultGateway.NextHop
    }
} | Where-Object { $_.IPv4Address -and $_.IPv4DefaultGateway } | ConvertTo-Json"#
                .to_string(),
        ],
        hide_window: true,
    }
}

#[cfg(target_os = "windows")]
fn build_process_command(plan: &ProcessLaunchPlan) -> std::process::Command {
    let mut command = std::process::Command::new(&plan.program);
    command.args(&plan.args);
    if plan.hide_window {
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

#[tauri::command]
pub(crate) async fn list_network_adapters() -> Result<Vec<NetworkAdapter>, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let output = build_process_command(&windows_network_adapter_query_plan())
                .output()
                .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

            if !output.status.success() {
                let err_msg = String::from_utf8_lossy(&output.stderr);
                log::error!("PowerShell network adapter query failed: {}", err_msg);
                return Err(format!("Query failed: {}", err_msg));
            }

            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let trimmed = stdout_str.trim();
            if trimmed.is_empty() {
                return Ok(vec![]);
            }

            #[derive(serde::Deserialize, Debug, Clone)]
            #[serde(untagged)]
            enum PowerShellOutput {
                Single(NetworkAdapterRaw),
                Array(Vec<NetworkAdapterRaw>),
            }

            #[derive(serde::Deserialize, Debug, Clone)]
            struct NetworkAdapterRaw {
                #[serde(rename = "Name")]
                name: String,
                #[serde(rename = "InterfaceDescription")]
                description: String,
                #[serde(rename = "IPv4Address")]
                ip_address: Option<String>,
                #[serde(rename = "IPv4DefaultGateway")]
                gateway: Option<String>,
            }

            match serde_json::from_str::<PowerShellOutput>(trimmed) {
                Ok(PowerShellOutput::Array(list)) => {
                    let adapters = list
                        .into_iter()
                        .map(|item| NetworkAdapter {
                            name: item.name,
                            description: item.description,
                            ip_address: item.ip_address,
                            gateway: item.gateway,
                        })
                        .collect();
                    Ok(adapters)
                }
                Ok(PowerShellOutput::Single(item)) => Ok(vec![NetworkAdapter {
                    name: item.name,
                    description: item.description,
                    ip_address: item.ip_address,
                    gateway: item.gateway,
                }]),
                Err(e) => {
                    log::warn!("Failed to parse PowerShell JSON: {}, raw: {}", e, trimmed);
                    Ok(vec![])
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(vec![])
        }
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub(crate) async fn connect_websocket(
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport_state = state.transport.clone();
    let vendor_state = state.vendor.clone();
    let conn_port_state = state.connected_port.clone();
    let conn_usb_state = state.connected_usb_ids.clone();
    let at_log_state = state.at_command_log.clone();

    let host_clone = host.clone();
    let user_clone = username.clone();
    let pass_clone = password.clone();

    tokio::task::spawn_blocking(move || {
        let mut transport = modem_hal::transport::WebSocketTransport::new(
            &host_clone,
            port,
            user_clone.as_deref(),
            pass_clone.as_deref(),
        )?;

        let vendor = ModemFactory::create(&mut transport);
        let id = format!("ws_{}:{}", host_clone, port);

        let v = match vendor {
            Ok(v) => v,
            Err(e) => return Err(format!("连接成功但无法识别模组型号: {}", e)),
        };

        *conn_port_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *conn_usb_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *transport_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? =
            Some(wrap_transport(Box::new(transport), at_log_state));

        log::info!("Detected vendor over WebSocket: {:?}", v.vendor());
        *vendor_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = Some(v);

        log::info!(
            "Connected to WebSocket {}:{}{}",
            host_clone,
            port,
            user_clone
                .as_deref()
                .map(|user| format!(" as {}", user))
                .unwrap_or_default()
        );
        Ok(id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[cfg(test)]
mod tests {
    use super::parse_vid_pid_from_identifier;
    #[cfg(target_os = "windows")]
    use super::windows_network_adapter_query_plan;

    #[test]
    fn parse_vid_pid_from_usb_hardware_id() {
        assert_eq!(
            parse_vid_pid_from_identifier(r"USB\VID_2C7C&PID_0900&MI_04"),
            (Some(0x2C7C), Some(0x0900))
        );
        assert_eq!(
            parse_vid_pid_from_identifier(r"usb\vid_2c7c&pid_0900"),
            (Some(0x2C7C), Some(0x0900))
        );
        assert_eq!(
            parse_vid_pid_from_identifier("ROOT\\SOMETHING"),
            (None, None)
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn network_adapter_query_plan_hides_console_window() {
        let plan = windows_network_adapter_query_plan();
        assert_eq!(plan.program, "powershell");
        assert!(plan.hide_window);
        assert!(plan.args.iter().any(|arg| arg == "-NoProfile"));
        assert!(plan.args.iter().any(|arg| arg == "-Command"));
        assert!(plan.args.iter().any(|arg| arg.contains("Get-NetAdapter")));
    }
}

