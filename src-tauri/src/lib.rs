use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use modem_hal::transport::AtTransport;
use modem_hal::types::*;
use modem_hal::ModemFactory;
use modem_hal::ModemVendor;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Emitter;
use tauri::Manager;

pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    pub vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
    pub data_cid: Arc<Mutex<i32>>,
    /// The serial port name when connected via serial/AT (None if TCP or disconnected).
    /// Used by the USB monitor to know if the active port was unplugged.
    pub connected_port: Arc<Mutex<Option<String>>>,
    /// Log of AT commands sent internally (not from raw AT terminal).
    /// Populated by LoggingTransport, consumed by pop_at_commands.
    pub at_command_log: Arc<Mutex<Vec<String>>>,
}

/// Transport wrapper that logs every sent AT command to a shared log.
struct LoggingTransport {
    inner: Box<dyn AtTransport>,
    log: Arc<Mutex<Vec<String>>>,
}

impl AtTransport for LoggingTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        let mut log = self.log.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        if log.len() < 1000 {
            log.push(modem_hal::transport::redact_at_command(command));
        }
        drop(log);
        self.inner.send_at(command)
    }
    fn close(&mut self) {
        self.inner.close();
    }
    fn is_alive(&self) -> bool {
        self.inner.is_alive()
    }
}

fn wrap_transport(
    transport: Box<dyn AtTransport>,
    log: Arc<Mutex<Vec<String>>>,
) -> Box<dyn AtTransport> {
    Box::new(LoggingTransport {
        inner: transport,
        log,
    })
}

// ── Port listing ──

/// On Windows, read the friendly name for all COM ports from the registry.
/// Returns a HashMap<port_name, (friendly_name, manufacturer)>.
///
/// Strategy: scan HKLM\SYSTEM\CurrentControlSet\Enum recursively for subkeys
/// that have a "FriendlyName" value and a child "Device Parameters" key with
/// a "PortName" value matching "COMx".
#[cfg(target_os = "windows")]
fn get_windows_all_port_info() -> std::collections::HashMap<String, (Option<String>, Option<String>)>
{
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
fn get_windows_all_port_info() -> std::collections::HashMap<String, (Option<String>, Option<String>)>
{
    std::collections::HashMap::new()
}

fn resolve_port_info(
    port: &serialport::SerialPortInfo,
    win_info: &std::collections::HashMap<String, (Option<String>, Option<String>)>,
) -> (Option<String>, Option<String>) {
    let mut description: Option<String> = None;
    let mut manufacturer: Option<String> = None;

    if let serialport::SerialPortType::UsbPort(info) = &port.port_type {
        description = info.product.clone();
        manufacturer = info.manufacturer.clone();
    }

    if let Some((win_caption, win_mfg)) = win_info.get(&port.port_name) {
        if win_caption.is_some() {
            description = win_caption.clone();
        }
        if win_mfg.is_some() && manufacturer.is_none() {
            manufacturer = win_mfg.clone();
        }
    }

    (description, manufacturer)
}

#[tauri::command]
fn list_ports() -> Result<Vec<PortInfo>, String> {
    let ports =
        serialport::available_ports().map_err(|e| format!("Failed to list ports: {}", e))?;

    // Get Windows WMI info for all ports at once (single PowerShell call)
    let win_info = get_windows_all_port_info();

    let result: Vec<PortInfo> = ports
        .into_iter()
        .map(|port| {
            let (description, manufacturer) = resolve_port_info(&port, &win_info);

            let is_at_port = is_at_port(
                &port.port_name,
                &description.as_ref(),
                &manufacturer.as_ref(),
            );

            let display_name =
                build_display_name(&port.port_name, &description, &manufacturer, is_at_port);

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
fn is_at_port(
    _port_name: &str,
    description: &Option<&String>,
    manufacturer: &Option<&String>,
) -> bool {
    let desc_upper = description.map_or_else(String::new, |s| s.to_uppercase());
    let mfg_upper = manufacturer.map_or_else(String::new, |s| s.to_uppercase());

    // Strong match: description explicitly contains "AT" as a standalone keyword
    // e.g., "AT Command Port", "AT Port", "AT Interface", "AT Modem"
    if desc_upper
        .split(|c: char| !c.is_alphabetic())
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
        if desc_upper.contains("NMEA")
            || desc_upper.contains("DIAG")
            || desc_upper.contains("DEBUG")
        {
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
    let ports =
        serialport::available_ports().map_err(|e| format!("Failed to list ports: {}", e))?;

    let win_info = get_windows_all_port_info();

    let mut at_candidates: Vec<String> = Vec::new();

    for port in &ports {
        let (description, manufacturer) = resolve_port_info(port, &win_info);

        if is_at_port(
            &port.port_name,
            &description.as_ref(),
            &manufacturer.as_ref(),
        ) {
            at_candidates.push(port.port_name.clone());
        }
    }

    if at_candidates.is_empty() {
        return Err("未找到AT端口".to_string());
    }

    log::info!("AT candidates: {:?}", at_candidates);

    let transport_arc = state.transport.clone();
    let vendor_arc = state.vendor.clone();
    for port_name in &at_candidates {
        log::info!("Probing port: {}", port_name);

        let pn = port_name.clone();
        let result = tokio::task::spawn_blocking(move || {
            // Open, send AT, verify OK, detect vendor, and return both
            let mut transport = modem_hal::transport::SerialTransport::new(&pn, 115200)?;
            let response = transport.send_at("AT");
            match response {
                Ok(r) if r.trim().ends_with("OK") => {
                    let vendor_result = ModemFactory::create(&mut transport);
                    Ok((transport, vendor_result))
                }
                Ok(r) => Err(format!("Port {} responded but not OK: {}", pn, r)),
                Err(e) => Err(format!("Port {} AT probe failed: {}", pn, e)),
            }
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?;

        match result {
            Ok((transport, vendor_result)) => {
                match vendor_result {
                    Ok(vendor) => {
                        log::info!("Connected to AT port: {} (vendor: {:?})", port_name, vendor.vendor());
                        *transport_arc.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(wrap_transport(
                            Box::new(transport),
                            state.at_command_log.clone(),
                        ));
                        *vendor_arc.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(vendor);
                        let cp = state.connected_port.clone();
                        *cp.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(port_name.clone());
                        return Ok(port_name.clone());
                    }
                    Err(e) => {
                        log::warn!("Port {} AT ok but vendor detection failed: {}", port_name, e);
                        continue;
                    }
                }
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
async fn connect_serial(
    port_name: String,
    baud_rate: u32,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport_state = state.transport.clone();
    let vendor_state = state.vendor.clone();
    let conn_port_state = state.connected_port.clone();
    let at_log_state = state.at_command_log.clone();
    
    let port_name_clone = port_name.clone();
    tokio::task::spawn_blocking(move || {
        let mut transport = modem_hal::transport::SerialTransport::new(&port_name_clone, baud_rate)?;
        let vendor = ModemFactory::create(&mut transport);
        let id = format!("serial_{}", port_name_clone);
        
        *transport_state.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(wrap_transport(
            Box::new(transport),
            at_log_state,
        ));
        
        match vendor {
            Ok(v) => {
                log::info!("Detected vendor: {:?}", v.vendor());
                *vendor_state.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(v);
            }
            Err(e) => {
                return Err(format!("连接成功但无法识别模组型号: {}", e));
            }
        }

        *conn_port_state.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(port_name_clone.clone());
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
async fn connect_tcp(
    host: String,
    port: u16,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport_state = state.transport.clone();
    let vendor_state = state.vendor.clone();
    let conn_port_state = state.connected_port.clone();
    let at_log_state = state.at_command_log.clone();
    
    let host_clone = host.clone();
    tokio::task::spawn_blocking(move || {
        let mut transport = modem_hal::transport::TcpTransport::new(&host_clone, port)?;
        let vendor = ModemFactory::create(&mut transport);
        let id = format!("tcp_{}:{}", host_clone, port);
        
        *conn_port_state.lock().map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *transport_state.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(wrap_transport(
            Box::new(transport),
            at_log_state,
        ));
        
        match vendor {
            Ok(v) => {
                log::info!("Detected vendor: {:?}", v.vendor());
                *vendor_state.lock().map_err(|e| format!("Lock poisoned: {}", e))? = Some(v);
            }
            Err(e) => {
                return Err(format!("连接成功但无法识别模组型号: {}", e));
            }
        }

        log::info!("Connected to TCP {}:{}", host_clone, port);
        Ok(id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn disconnect(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let connected_port = state.connected_port.clone();
    let data_cid = state.data_cid.clone();
    
    tokio::task::spawn_blocking(move || {
        let mut t = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        if let Some(ref mut transport) = *t {
            transport.close();
        }
        *t = None;
        *vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *connected_port.lock().map_err(|e| format!("Lock poisoned: {}", e))? = None;
        *data_cid.lock().map_err(|e| format!("Lock poisoned: {}", e))? = 1;
        Ok("Disconnected".to_string())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

// ── High-level modem queries (all async to avoid blocking UI) ──

#[tauri::command]
async fn get_modem_status(state: tauri::State<'_, AppState>) -> Result<ModemStatus, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_modem_status(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_hardware_info(state: tauri::State<'_, AppState>) -> Result<HardwareInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_hardware_info(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_ip_info(state: tauri::State<'_, AppState>) -> Result<IpInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        v.query_ip_info(t, if cid > 0 { cid } else { 1 })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_apn_list(state: tauri::State<'_, AppState>) -> Result<Vec<ApnEntry>, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_apn_list(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_neighbor_cells(state: tauri::State<'_, AppState>) -> Result<NeighborCells, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_neighbor_cells(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_qos_info(state: tauri::State<'_, AppState>) -> Result<QosInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        v.query_qos(t, if cid > 0 { cid } else { 1 })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_network_mode(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_network_mode(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
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
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_apn(t, cid, context_type, &apn, &username, &password, auth_type)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn delete_apn_config(cid: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.delete_apn(t, cid)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_apn_active(cid: i32, active: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_apn_active(t, cid, active)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn connect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let cid = if cid > 0 { cid } else { 1 };
        v.connect_data(t, cid)?;
        *data_cid.lock().map_err(|e| format!("Lock poisoned: {}", e))? = cid;
        Ok(())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_5glan(state: tauri::State<'_, AppState>) -> Result<Vec<L5GanEntry>, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_5glan(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_5glan(cid: i32, enabled: bool, vlan_id: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_5glan(t, cid, enabled, vlan_id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_vlan(state: tauri::State<'_, AppState>) -> Result<Vec<i32>, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_vlan(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_vlan(vlan_id: i32, enabled: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_vlan(t, vlan_id, enabled)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn disconnect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let cid = if cid > 0 { cid } else { 1 };
        v.disconnect_data(t, cid)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_network_mode_cmd(
    mode: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_network_mode(t, &mode)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_nr5g_band_cmd(band: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_nr5g_bands(t, &band)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn reboot_modem(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.reboot(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_cfun(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_cfun(t, mode)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn factory_reset(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.factory_reset(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_sim_slot(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_sim_slot(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_sim_slot(slot: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.switch_sim_slot(t, slot)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn send_raw_at(command: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        t.send_at(&command)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn query_cell_lock(state: tauri::State<'_, AppState>) -> Result<Vec<CellLockEntry>, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_cell_lock(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_cell_lock(
    arfcn: String,
    pci: String,
    scs: String,
    band: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_cell_lock(t, &arfcn, &pci, &scs, &band)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn clear_cell_lock(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.clear_cell_lock(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_plmn_lock(plmn: String, password: Option<String>, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let pw = password.unwrap_or_else(|| "12345678".to_string());
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_plmn_lock(t, &plmn, &pw)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn clear_plmn_lock(password: Option<String>, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let pw = password.unwrap_or_else(|| "12345678".to_string());
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.clear_plmn_lock(t, &pw)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

/// Return all AT commands logged by internal operations since last call, then clear the log.
#[tauri::command]
fn pop_at_commands(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut log = state.at_command_log.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
    Ok(std::mem::take(&mut *log))
}

#[tauri::command]
async fn get_bands(state: tauri::State<'_, AppState>) -> Result<BandConfig, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_bands_with_spec(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_bands(
    lte: String,
    nr: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_bands(t, &lte, &nr)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn reset_all_bands(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.reset_all_bands(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_feature_toggles(state: tauri::State<'_, AppState>) -> Result<FeatureToggles, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_feature_toggles(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_feature_toggle(
    feature: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_feature_toggle(t, &feature, enabled)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_qualcomm_config(state: tauri::State<'_, AppState>) -> Result<QualcommConfig, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_qualcomm_config(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_qualcomm_config(
    param: String,
    value: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_qualcomm_config(t, &param, &value)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_traffic(state: tauri::State<'_, AppState>) -> Result<TrafficInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_traffic(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_usbnet_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_usbnet_mode(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_usbnet_mode(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_usbnet_mode(t, mode)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_nat_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_nat_mode(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn set_nat_mode(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_nat_mode(t, mode)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

// ── USB hotplug monitor ──

#[derive(Clone, serde::Serialize)]
struct PortChangeEvent {
    added: Vec<String>,
    removed: Vec<String>,
}

/// Polls serial ports every 2 seconds and emits `port-changed` events to the
/// frontend when devices are added or removed. The frontend decides whether to
/// auto-connect (USB AT) or stay idle. This keeps the monitor stateless.
fn start_port_monitor(app_handle: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || {
            loop {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // Seed with current ports so a restart doesn't report every live port as "added".
                    let mut previous_ports: HashSet<String> = serialport::available_ports()
                        .map(|ps| ps.into_iter().map(|p| p.port_name).collect())
                        .unwrap_or_default();
                    loop {
                        std::thread::sleep(Duration::from_secs(2));

                        let ports = match serialport::available_ports() {
                            Ok(p) => p,
                            Err(e) => {
                                log::warn!("[USB监控] available_ports 失败: {}", e);
                                continue;
                            }
                        };

                        let current_names: HashSet<String> = ports.iter().map(|p| p.port_name.clone()).collect();

                        let added: Vec<String> = current_names.difference(&previous_ports).cloned().collect();
                        let removed: Vec<String> = previous_ports.difference(&current_names).cloned().collect();
                        previous_ports = current_names;

                        if added.is_empty() && removed.is_empty() {
                            continue;
                        }

                        log::info!("[USB监控] 端口变化 — 新增: {:?}, 移除: {:?}", added, removed);

                        if let Err(e) = app_handle.emit("port-changed", PortChangeEvent {
                            added: added.clone(),
                            removed: removed.clone(),
                        }) {
                            log::warn!("[USB监控] 发送事件失败: {}", e);
                        }
                    }
                }));
                if let Err(e) = result {
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "未知错误".to_string()
                    };
                    log::error!("[USB监控] 线程崩溃: {}，5秒后重启", msg);
                    std::thread::sleep(Duration::from_secs(5));
                }
            }
        })
        .expect("无法创建 USB 监控线程");
}

/// Checks transport liveness every 4 s. When `is_alive()` returns false the
/// USB device is gone — emit `port-changed` so the frontend disconnects.
/// Uses no AT bytes, so nothing appears in the AT log.
fn start_connection_heartbeat(
    app_handle: tauri::AppHandle,
    transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    connected_port: Arc<Mutex<Option<String>>>,
) {
    std::thread::Builder::new()
        .name("connection-heartbeat".into())
        .spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(4));

                let port_name = match connected_port.lock() {
                    Ok(g) => g.clone(),
                    Err(_) => continue,
                };
                let Some(port) = port_name else { continue };

                let alive = match transport.lock() {
                    Ok(g) => match g.as_deref() {
                        Some(t) => t.is_alive(),
                        None => continue,
                    },
                    Err(_) => continue,
                };

                if !alive {
                    log::warn!("[心跳] 检测到硬件断连，端口: {}", port);
                    // Clear connected_port immediately so subsequent ticks don't re-fire
                    if let Ok(mut g) = connected_port.lock() {
                        *g = None;
                    }
                    if let Err(e) = app_handle.emit("port-changed", PortChangeEvent {
                        added: vec![],
                        removed: vec![port],
                    }) {
                        log::warn!("[心跳] 发送断连事件失败: {}", e);
                    }
                }
            }
        })
        .expect("无法创建心跳线程");
}

// ── Timing ──

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Modem Cat application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // Second instance launched — bring existing window to front
            if let Some(window) = app.webview_windows().get("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .manage(AppState {
            transport: Arc::new(Mutex::new(None)),
            vendor: Arc::new(Mutex::new(None)),
            data_cid: Arc::new(Mutex::new(1)),
            connected_port: Arc::new(Mutex::new(None)),
            at_command_log: Arc::new(Mutex::new(Vec::new())),
        })
        .setup(|app| {
            // ── Build menu bar ──
            let about = MenuItemBuilder::with_id("about", "关于 Modem Cat")
                .build(app)?;
            let help_menu = SubmenuBuilder::new(app, "帮助")
                .item(&about)
                .build()?;
            let menu = MenuBuilder::new(app)
                .item(&help_menu)
                .build()?;
            app.set_menu(menu)?;
            app.on_menu_event(|handle, event| {
                if event.id() == "about" {
                    let _ = handle.emit("show-about", ());
                }
            });

            start_port_monitor(app.handle().clone());

            let state = app.state::<AppState>();
            start_connection_heartbeat(
                app.handle().clone(),
                state.transport.clone(),
                state.connected_port.clone(),
            );

            let show_item = tauri::menu::MenuItemBuilder::with_id("show_window", "控制面板").build(app)?;
            let quit_item = tauri::menu::MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = tauri::menu::MenuBuilder::new(app)
                .item(&show_item)
                .item(&quit_item)
                .build()?;

            let tray_icon = match app.tray_by_id("main") {
                Some(t) => t,
                None => {
                    log::error!("Tray icon 'main' not configured in tauri.conf.json");
                    return Ok(());
                }
            };
            if let Err(e) = tray_icon.set_menu(Some(menu)) {
                log::error!("Failed to set tray menu: {}", e);
            }

            tray_icon.on_menu_event(|app, event| {
                match event.id.as_ref() {
                    "show_window" => {
                        if let Some(window) = app.webview_windows().get("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            });

            tray_icon.on_tray_icon_event(|tray, event| {
                if let tauri::tray::TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                    let app = tray.app_handle();
                    if let Some(window) = app.webview_windows().get("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            });

            Ok(())
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
            get_qualcomm_config,
            // Write operations
            set_apn_config,
            delete_apn_config,
            set_apn_active,
            get_5glan,
            set_5glan,
            get_vlan,
            set_vlan,
            connect_data,
            disconnect_data,
            set_network_mode_cmd,
            set_nr5g_band_cmd,
            set_bands,
            reset_all_bands,
            set_feature_toggle,
            set_usbnet_mode,
            get_nat_mode,
            set_nat_mode,
            set_qualcomm_config,
            reboot_modem,
            set_cfun,
            factory_reset,
            get_sim_slot,
            set_sim_slot,
            send_raw_at,
            pop_at_commands,
            // Cell lock / PLMN lock
            query_cell_lock,
            set_cell_lock,
            clear_cell_lock,
            set_plmn_lock,
            clear_plmn_lock,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if let Err(e) = window.hide() {
                    log::warn!("Failed to hide window: {}", e);
                }
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
