use std::sync::{Arc, Mutex};

pub mod at_adapter;
pub mod at_parser;
pub mod network;
pub mod serial;
pub mod transport;
pub mod types;

use transport::AtTransport;
use types::*;

pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    pub data_cid: Arc<Mutex<i32>>,
}

// ── Port listing ──

/// On Windows, read the friendly name for all COM ports from the registry.
/// Returns a HashMap<port_name, (friendly_name, manufacturer)>.
///
/// Strategy: scan HKLM\SYSTEM\CurrentControlSet\Enum recursively for subkeys
/// that have a "FriendlyName" value and a child "Device Parameters" key with
/// a "PortName" value matching "COMx".
#[cfg(target_os = "windows")]
fn get_windows_all_port_info() -> std::collections::HashMap<String, (Option<String>, Option<String>)> {
    use std::collections::HashMap;
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    let mut result = HashMap::new();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let enum_key = match hklm.open_subkey(r"SYSTEM\CurrentControlSet\Enum") {
        Ok(k) => k,
        Err(_) => return result,
    };

    // Iterate through bus types (e.g. USB, PCI, ACPI)
    for bus in enum_key.enum_keys().flatten() {
        let bus_key = match enum_key.open_subkey(&bus) {
            Ok(k) => k,
            Err(_) => continue,
        };

        // Iterate through device instances (e.g. VID_2C7C&PID_0125)
        for device in bus_key.enum_keys().flatten() {
            let device_key = match bus_key.open_subkey(&device) {
                Ok(k) => k,
                Err(_) => continue,
            };

            // Iterate through function instances (e.g. 0000, 0001)
            for func in device_key.enum_keys().flatten() {
                let func_key = match device_key.open_subkey(&func) {
                    Ok(k) => k,
                    Err(_) => continue,
                };

                // Check if this has a "Device Parameters" subkey with PortName
                let dp_key = match func_key.open_subkey("Device Parameters") {
                    Ok(k) => k,
                    Err(_) => continue,
                };

                let port_name: String = match dp_key.get_value("PortName") {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                // Read FriendlyName and Manufacturer from the func key
                let friendly_name: Option<String> = func_key.get_value("FriendlyName").ok();
                let manufacturer: Option<String> = func_key.get_value("Manufacturer").ok();

                result.insert(port_name, (friendly_name, manufacturer));
            }
        }
    }

    result
}

#[cfg(not(target_os = "windows"))]
fn get_windows_all_port_info() -> std::collections::HashMap<String, (Option<String>, Option<String>)> {
    std::collections::HashMap::new()
}

#[tauri::command]
fn list_ports() -> Result<Vec<PortInfo>, String> {
    let ports = serialport::available_ports()
        .map_err(|e| format!("Failed to list ports: {}", e))?;

    // Get Windows WMI info for all ports at once (single PowerShell call)
    let win_info = get_windows_all_port_info();

    let result: Vec<PortInfo> = ports
        .into_iter()
        .map(|port| {
            let mut description: Option<String> = None;
            let mut manufacturer: Option<String> = None;

            // First try serialport crate info
            if let serialport::SerialPortType::UsbPort(info) = &port.port_type {
                description = info.product.clone();
                manufacturer = info.manufacturer.clone();
            }

            // Overlay with WMI info (more reliable on Windows)
            if let Some((win_caption, win_mfg)) = win_info.get(&port.port_name) {
                if win_caption.is_some() {
                    description = win_caption.clone();
                }
                if win_mfg.is_some() && manufacturer.is_none() {
                    manufacturer = win_mfg.clone();
                }
            }

            let is_at_port = is_at_port(&port.port_name, &description.as_ref(), &manufacturer.as_ref());

            let display_name = build_display_name(&port.port_name, &description, &manufacturer, is_at_port);

            PortInfo {
                port_name: port.port_name,
                description,
                manufacturer,
                is_at_port,
                display_name,
            }
        })
        .collect();

    Ok(result)
}

/// Build a human-readable display name for the port dropdown.
/// Shows the friendly name (e.g. "Quectel AT Command Port") without the COM port prefix,
/// since the value already contains the port name.
fn build_display_name(
    port_name: &str,
    description: &Option<String>,
    manufacturer: &Option<String>,
    is_at_port: bool,
) -> String {
    // If we have a WMI Caption like "Quectel AT Command Port (COM3)", use it directly
    if let Some(desc) = description {
        if !desc.is_empty() {
            // Remove trailing (COMx) from caption if present, since the port_name is already known
            let cleaned = regex_remove_com_suffix(desc);
            if is_at_port {
                return format!("{} [AT]", cleaned);
            }
            return cleaned;
        }
    }

    // Build from manufacturer + fallback
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
            format!("{} - AT端口", parts.join(" - "))
        }
    } else {
        if parts.is_empty() {
            port_name.to_string()
        } else {
            parts.join(" - ")
        }
    }
}

/// Remove trailing "(COMx)" suffix from a string, e.g. "Quectel AT Port (COM3)" -> "Quectel AT Port"
fn regex_remove_com_suffix(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.ends_with(')') {
        if let Some(open) = trimmed.rfind('(') {
            let inside = &trimmed[open + 1..trimmed.len() - 1];
            // Check if it looks like "COMn"
            if inside.starts_with("COM") && inside[3..].chars().all(|c| c.is_ascii_digit()) {
                return trimmed[..open].trim().to_string();
            }
        }
    }
    s.to_string()
}

/// Determine if a serial port is likely an AT command port.
///
/// Priority: exact description match for "AT" keyword.
/// We check for common patterns in modem port descriptions:
///   - "AT Command Port", "AT Port", "AT Interface"
///   - Quectel RM/RG series with specific port naming
fn is_at_port(_port_name: &str, description: &Option<&String>, manufacturer: &Option<&String>) -> bool {
    let desc_upper = description.map_or_else(String::new, |s| s.to_uppercase());
    let mfg_upper = manufacturer.map_or_else(String::new, |s| s.to_uppercase());

    // Strong match: description explicitly contains "AT" as a standalone keyword
    // e.g., "AT Command Port", "AT Port", "AT Interface", "AT Modem"
    if desc_upper.split(|c: char| !c.is_alphabetic())
        .any(|word| word == "AT")
    {
        return true;
    }

    // Known modem manufacturers — their ports are likely AT capable
    let is_modem_manufacturer = mfg_upper.contains("QUECTEL")
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
        // For modem manufacturers, check if description suggests this is the AT port
        // (not NMEA, not DM, not Diag)
        if desc_upper.contains("NMEA") || desc_upper.contains("DIAG") || desc_upper.contains("DEBUG") {
            return false;
        }
        // Modem manufacturer + description contains "MODEM" or "COMMAND" → likely AT
        if desc_upper.contains("MODEM") || desc_upper.contains("COMMAND") {
            return true;
        }
        // No description but modem manufacturer — mark as AT candidate
        // (will be verified with actual AT probe)
        if desc_upper.is_empty() {
            return true;
        }
    }

    false
}

/// Auto-detect and connect to an AT port.
/// Scans all port types (not just USB) and uses WMI info on Windows for
/// reliable identification, matching the logic in `list_ports`.
#[tauri::command]
async fn auto_connect_at(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let ports = serialport::available_ports()
        .map_err(|e| format!("Failed to list ports: {}", e))?;

    let win_info = get_windows_all_port_info();

    let mut at_candidates: Vec<String> = Vec::new();

    for port in &ports {
        let mut description: Option<String> = None;
        let mut manufacturer: Option<String> = None;

        // First try serialport crate info
        if let serialport::SerialPortType::UsbPort(info) = &port.port_type {
            description = info.product.clone();
            manufacturer = info.manufacturer.clone();
        }

        // Overlay with WMI info (more reliable on Windows)
        if let Some((win_caption, win_mfg)) = win_info.get(&port.port_name) {
            if win_caption.is_some() {
                description = win_caption.clone();
            }
            if win_mfg.is_some() && manufacturer.is_none() {
                manufacturer = win_mfg.clone();
            }
        }

        if is_at_port(&port.port_name, &description.as_ref(), &manufacturer.as_ref()) {
            at_candidates.push(port.port_name.clone());
        }
    }

    if at_candidates.is_empty() {
        return Err("未找到AT端口".to_string());
    }

    log::info!("AT candidates: {:?}", at_candidates);

    let transport_arc = state.transport.clone();
    for port_name in &at_candidates {
        log::info!("Probing port: {}", port_name);

        let pn = port_name.clone();
        let result = tokio::task::spawn_blocking(move || {
            // Open, send AT, verify OK, and return the transport
            let mut transport = transport::SerialTransport::new(&pn, 115200)?;
            let response = transport.send_at("AT");
            match response {
                Ok(r) if r.trim().ends_with("OK") => Ok(transport),
                Ok(r) => Err(format!("Port {} responded but not OK: {}", pn, r)),
                Err(e) => Err(format!("Port {} AT probe failed: {}", pn, e)),
            }
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?;

        match result {
            Ok(transport) => {
                log::info!("Connected to AT port: {}", port_name);
                *transport_arc.lock().unwrap() = Some(Box::new(transport));
                return Ok(port_name.clone());
            }
            Err(e) => {
                log::warn!("Failed to open {}: {}", port_name, e);
                continue;
            }
        }
    }

    Err(format!("所有候选端口均无法打开: {:?}", at_candidates))
}

// ── Connection management ──

#[tauri::command]
fn connect_serial(
    port_name: String,
    baud_rate: u32,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport = transport::SerialTransport::new(&port_name, baud_rate)?;
    let id = format!("serial_{}", port_name);
    *state.transport.lock().unwrap() = Some(Box::new(transport));
    log::info!("Connected to serial port {} at {} baud", port_name, baud_rate);
    Ok(id)
}

#[tauri::command]
fn connect_tcp(
    host: String,
    port: u16,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport = transport::TcpTransport::new(&host, port)?;
    let id = format!("tcp_{}:{}", host, port);
    *state.transport.lock().unwrap() = Some(Box::new(transport));
    log::info!("Connected to TCP {}:{}", host, port);
    Ok(id)
}

#[tauri::command]
fn disconnect(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut t = state.transport.lock().unwrap();
    if let Some(ref mut transport) = *t {
        transport.close();
    }
    *t = None;
    Ok("Disconnected".to_string())
}

// ── High-level modem queries (all async to avoid blocking UI) ──

#[tauri::command]
async fn get_modem_status(state: tauri::State<'_, AppState>) -> Result<ModemStatus, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_modem_status(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_hardware_info(state: tauri::State<'_, AppState>) -> Result<HardwareInfo, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_hardware_info(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_ip_info(state: tauri::State<'_, AppState>) -> Result<IpInfo, String> {
    let transport = state.transport.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        let cid = *data_cid.lock().unwrap();
        at_adapter::query_ip_info(t, if cid > 0 { cid } else { 1 })
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_apn_list(state: tauri::State<'_, AppState>) -> Result<Vec<ApnEntry>, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_apn_list(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_neighbor_cells(state: tauri::State<'_, AppState>) -> Result<NeighborCells, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        let (lte, nr) = at_adapter::query_neighbor_cells(t)?;
        Ok(NeighborCells { lte, nr })
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_qos_info(state: tauri::State<'_, AppState>) -> Result<QosInfo, String> {
    let transport = state.transport.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        let cid = *data_cid.lock().unwrap();
        at_adapter::query_qos(t, if cid > 0 { cid } else { 1 })
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_network_mode(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_network_mode(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

// ── Write operations (async to avoid blocking UI) ──

#[tauri::command]
async fn set_apn_config(
    cid: i32,
    context_type: i32,
    apn: String,
    username: String,
    password: String,
    auth_type: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::set_apn(t, cid, context_type, &apn, &username, &password, auth_type)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn delete_apn_config(
    cid: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::delete_apn(t, cid)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn connect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        let cid = *data_cid.lock().unwrap();
        let cid = if cid > 0 { cid } else { 1 };
        at_adapter::connect_data(t, cid)?;
        *data_cid.lock().unwrap() = cid;
        Ok(())
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn disconnect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        let cid = *data_cid.lock().unwrap();
        let cid = if cid > 0 { cid } else { 1 };
        at_adapter::disconnect_data(t, cid)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_network_mode_cmd(
    mode: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::set_network_mode(t, &mode)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_nr5g_band_cmd(
    band: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::set_nr5g_band(t, &band)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn reboot_modem(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::reboot_modem(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn factory_reset(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::factory_reset(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn send_raw_at(
    command: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::send_raw_at(t, &command)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_bands(state: tauri::State<'_, AppState>) -> Result<BandConfig, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_bands(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_bands(
    lte: String,
    nr: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::set_bands(t, &lte, &nr)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn reset_all_bands(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::reset_all_bands(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_feature_toggles(state: tauri::State<'_, AppState>) -> Result<FeatureToggles, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_feature_toggles(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_feature_toggle(
    feature: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        let val = if enabled { 1 } else { 0 };
        match feature.as_str() {
            "pcieMode" => at_adapter::set_qcfg_toggle(t, "pcie/mode", val),
            "ethernet" => at_adapter::set_qcfg_toggle(t, "ethernet", val),
            "proxyArp" => at_adapter::set_qcfg_toggle(t, "proxyarp", val),
            "uartAt" => at_adapter::set_qcfg_toggle(t, "uartat", val),
            "ethAt" => at_adapter::set_qcfg_toggle(t, "eth_at", val),
            "adb" => at_adapter::set_adb(t, enabled),
            _ => Err(format!("Unknown feature: {}", feature)),
        }
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_traffic(state: tauri::State<'_, AppState>) -> Result<TrafficInfo, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_traffic(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_usbnet_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::query_usbnet(t)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_usbnet_mode(
    mode: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut guard = transport.lock().unwrap();
        let t = guard.as_deref_mut().ok_or("Not connected")?;
        at_adapter::set_usbnet(t, mode)
    }).await.map_err(|e| format!("Task error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Modem Cat application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            transport: Arc::new(Mutex::new(None)),
            data_cid: Arc::new(Mutex::new(1)),
        })
        .invoke_handler(tauri::generate_handler![
            // Port / connection
            list_ports,
            auto_connect_at,
            connect_serial,
            connect_tcp,
            disconnect,
            // High-level queries
            get_modem_status,
            get_hardware_info,
            get_ip_info,
            get_apn_list,
            get_neighbor_cells,
            get_qos_info,
            get_network_mode,
            get_bands,
            get_feature_toggles,
            get_usbnet_mode,
            get_traffic,
            // Write operations
            set_apn_config,
            delete_apn_config,
            connect_data,
            disconnect_data,
            set_network_mode_cmd,
            set_nr5g_band_cmd,
            set_bands,
            reset_all_bands,
            set_feature_toggle,
            set_usbnet_mode,
            reboot_modem,
            factory_reset,
            send_raw_at,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
