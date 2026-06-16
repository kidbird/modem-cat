use std::collections::{HashSet, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use modem_hal::transport::AtTransport;
use modem_hal::types::*;
use modem_hal::validate_at_string;
use modem_hal::validate_cid;
use modem_hal::validate_raw_at_command;
use modem_hal::ModemFactory;
use modem_hal::ModemVendor;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Emitter;
use tauri::Manager;

mod mqtt;

pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    pub vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
    /// Currently-connected PDP context ID. Stored as `AtomicI32` (not `Mutex`)
    /// because every read/write is a single integer and the field is only ever
    /// touched while transport + vendor are already held — no need to nest a
    /// third std::Mutex and risk std::Mutex deadlock (REVIEW.md #8).
    pub data_cid: Arc<AtomicI32>,
    /// The serial port name when connected via serial/AT (None if TCP or disconnected).
    /// Used by the USB monitor to know if the active port was unplugged.
    pub connected_port: Arc<Mutex<Option<String>>>,
    /// Log of AT commands sent internally (not from raw AT terminal).
    /// Populated by LoggingTransport, consumed by pop_at_commands.
    pub at_command_log: Arc<Mutex<VecDeque<String>>>,
    /// Handle of the active background MQTT connection loop task.
    pub mqtt_task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

/// Transport wrapper that logs every sent AT command to a shared log.
struct LoggingTransport {
    inner: Box<dyn AtTransport>,
    log: Arc<Mutex<VecDeque<String>>>,
}

/// Maximum AT log entries kept in memory. Older entries are evicted FIFO when
/// the ring is full. Frontend's `pop_at_commands` IPC drains the entire queue.
const AT_LOG_CAPACITY: usize = 1000;

impl AtTransport for LoggingTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        // Send FIRST, log AFTER with the actual outcome — that way the log
        // reflects what really happened on the wire, not what we *hoped* to
        // send. Previously the log was pushed before send_at ran, so a
        // disconnected-port or write error left a phantom "successful" entry.
        let result = self.inner.send_at(command);
        // try_lock so a slow consumer holding the log Mutex never blocks the
        // AT path; if we can't grab it, drop the log entry (the next
        // successful command will replace it).
        if let Ok(mut log) = self.log.try_lock() {
            let redacted = modem_hal::transport::redact_at_command(command);
            let entry = match &result {
                Ok(_) => redacted,
                Err(e) => format!("{}  ⟵ {}", redacted, e),
            };
            if log.len() >= AT_LOG_CAPACITY {
                log.pop_front();
            }
            log.push_back(entry);
        }
        result
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
    log: Arc<Mutex<VecDeque<String>>>,
) -> Box<dyn AtTransport> {
    Box::new(LoggingTransport {
        inner: transport,
        log,
    })
}

// ── Command helpers: eliminate repetitive lock/spawn_blocking boilerplate ──

/// Lock transport + vendor, run the closure, return `Result<_, String>`.
/// Used by ~34 read/write commands that follow the same pattern.
macro_rules! with_vendor {
    ($state:expr, |$t:ident, $v:ident| $body:expr) => {{
        let transport = $state.transport.clone();
        let vendor = $state.vendor.clone();
        tokio::task::spawn_blocking(move || {
            let mut tguard = transport
                .lock()
                .map_err(|e| format!("Lock poisoned: {}", e))?;
            let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
            let $t = tguard.as_deref_mut().ok_or("Not connected")?;
            let $v = vguard.as_deref_mut().ok_or("No vendor detected")?;
            $body
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }};
}

/// Same as `with_vendor!` but also reads `data_cid` (lock-free via AtomicI32)
/// and binds it as `$c`. The atomic means the macro no longer holds a third
/// Mutex while transport + vendor are held, so connect_data can no longer
/// deadlock with any concurrent IPC handler that touches data_cid.
macro_rules! with_vendor_cid {
    ($state:expr, |$t:ident, $v:ident, $c:ident| $body:expr) => {{
        let transport = $state.transport.clone();
        let vendor = $state.vendor.clone();
        let data_cid = $state.data_cid.clone();
        tokio::task::spawn_blocking(move || {
            let mut tguard = transport
                .lock()
                .map_err(|e| format!("Lock poisoned: {}", e))?;
            let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
            let $t = tguard.as_deref_mut().ok_or("Not connected")?;
            let $v = vguard.as_deref_mut().ok_or("No vendor detected")?;
            // Lock-free load: AtomicI32 needs no Mutex, so we never nest
            // a third lock under transport + vendor (REVIEW.md #8).
            let $c = data_cid.load(Ordering::Relaxed);
            $body
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }};
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
                return format!("{} ({}) [AT]", cleaned, port_name);
            }
            return format!("{} ({})", cleaned, port_name);
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
            format!("{} ({}) - AT端口", parts.join(" - "), port_name)
        }
    } else {
        if parts.is_empty() {
            port_name.to_string()
        } else {
            format!("{} ({})", parts.join(" - "), port_name)
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
        // For modem manufacturers, check if description suggests this is the AT port
        // (not NMEA, not DM, not Diag, not Debug, not QDLoader)
        let desc_words: Vec<&str> = desc_upper.split(|c: char| !c.is_alphabetic()).collect();
        if desc_upper.contains("NMEA")
            || desc_upper.contains("DIAG")
            || desc_upper.contains("DEBUG")
            || desc_upper.contains("QDLOADER")
            || desc_words.contains(&"DM")
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

/// Per-port timeout for the AT probe. The probe does open() + send_at("AT") +
/// wait for OK; an unresponsive or busy port can hang serialport::open() for
/// many seconds, so cap each probe at 2 s. Total wall time is ~2 s for N
/// ports (parallel) vs the old ~3-8 s × N (serial) — REVIEW.md #15.
const AT_PROBE_TIMEOUT: Duration = Duration::from_secs(2);

async fn run_after_at_probe_timeout<T, U, Probe, Detect, DetectFuture>(
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

    // Parallel probe: spawn one tokio task per candidate. Each task wraps only
    // the blocking serial open + AT handshake in a 2 s timeout. Vendor
    // detection runs after the AT probe succeeds and uses the transport's
    // normal serial read timeout, so slow AT+CGMM responses can still connect.
    let mut handles = Vec::with_capacity(at_candidates.len());
    for port_name in &at_candidates {
        let pn = port_name.clone();
        log::info!("Probing port: {}", pn);
        handles.push(tokio::spawn(async move {
            // `pn_block` is moved into spawn_blocking; the outer `pn` lives
            // through the async block so we can return it in the tuple below.
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
                        let vendor_result = ModemFactory::create(&mut transport);
                        Ok((transport, vendor_result))
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

    // Collect every probe's result; pick the first successful (transport +
    // vendor) pair. Losers' transports drop naturally when their tasks end.
    for h in handles {
        let (pn, probe_result) = match h.await {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Probe task join error: {}", e);
                continue;
            }
        };
        let (transport, vendor_result) = match probe_result {
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
                return Ok(pn);
            }
            Err(e) => {
                log::warn!("Port {} AT ok but vendor detection failed: {}", pn, e);
                continue;
            }
        }
    }

    Err(format!("所有候选端口均无法打开: {:?}", at_candidates))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn vendor_detection_is_not_limited_by_at_probe_timeout() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("tokio runtime");
        let detection_ran = Arc::new(AtomicBool::new(false));
        let detection_ran_for_task = detection_ran.clone();

        let result = runtime.block_on(async move {
            run_after_at_probe_timeout(
                Duration::from_millis(10),
                async { Ok::<_, String>("transport") },
                move |transport| async move {
                    tokio::time::sleep(Duration::from_millis(30)).await;
                    detection_ran_for_task.store(true, Ordering::SeqCst);
                    Ok::<_, String>((transport, "vendor"))
                },
            )
            .await
        });

        assert_eq!(result.unwrap(), ("transport", "vendor"));
        assert!(detection_ran.load(Ordering::SeqCst));
    }

    #[test]
    fn test_is_at_port() {
        // Test AT ports (should be true)
        assert!(is_at_port("COM5", &Some(&"Qualcomm HS-USB Android Modem".to_string()), &Some(&"Qualcomm".to_string())));
        assert!(is_at_port("COM5", &Some(&"Qualcomm HS-USB Modem".to_string()), &Some(&"Qualcomm Incorporated".to_string())));
        assert!(is_at_port("COM5", &Some(&"Quectel USB AT Port".to_string()), &Some(&"Quectel".to_string())));

        // Test DM/Diag/QDLoader/NMEA ports (should be false)
        assert!(!is_at_port("COM4", &Some(&"Qualcomm HS-USB Diagnostics 9008".to_string()), &Some(&"Qualcomm".to_string())));
        assert!(!is_at_port("COM4", &Some(&"Qualcomm HS-USB DM Port".to_string()), &Some(&"Qualcomm".to_string())));
        assert!(!is_at_port("COM4", &Some(&"Quectel USB DM Port".to_string()), &Some(&"Quectel".to_string())));
        assert!(!is_at_port("COM3", &Some(&"Quectel USB NMEA Port".to_string()), &Some(&"Quectel".to_string())));
        assert!(!is_at_port("COM3", &Some(&"Qualcomm HS-USB QDLoader 9008".to_string()), &Some(&"Qualcomm".to_string())));
    }

    #[tokio::test]
    async fn test_list_network_adapters() {
        let res = list_network_adapters().await;
        println!("Network adapters: {:?}", res);
        assert!(res.is_ok());
    }
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
        let mut transport =
            modem_hal::transport::SerialTransport::new(&port_name_clone, baud_rate)?;
        let vendor = ModemFactory::create(&mut transport);
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

        let v = match vendor {
            Ok(v) => v,
            Err(e) => return Err(format!("连接成功但无法识别模组型号: {}", e)),
        };

        *conn_port_state
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
async fn disconnect(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let connected_port = state.connected_port.clone();
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

#[tauri::command]
async fn list_network_adapters() -> Result<Vec<NetworkAdapter>, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("powershell")
                .args(&[
                    "-NoProfile",
                    "-Command",
                    "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; \
                     Get-NetAdapter | Where-Object { \
                         $_.Status -eq 'Up' -and \
                         $_.Virtual -eq $false -and \
                         $_.InterfaceDescription -notmatch 'Wi-Fi|Wireless|WLAN|802\\.11|Bluetooth|Loopback|TAP|VPN|Virtual|VMware|VirtualBox|Hyper-V|Wintun|Tunnel' \
                     } | ForEach-Object { \
                         $config = Get-NetIPConfiguration -InterfaceIndex $_.InterfaceIndex; \
                         [PSCustomObject]@{ \
                             Name = $_.Name; \
                             InterfaceDescription = $_.InterfaceDescription; \
                             IPv4Address = $config.IPv4Address.IPAddress; \
                             IPv4DefaultGateway = $config.IPv4DefaultGateway.NextHop \
                         } \
                     } | Where-Object { $_.IPv4Address -and $_.IPv4DefaultGateway } | ConvertTo-Json",
                ])
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
                    let adapters = list.into_iter().map(|item| NetworkAdapter {
                        name: item.name,
                        description: item.description,
                        ip_address: item.ip_address,
                        gateway: item.gateway,
                    }).collect();
                    Ok(adapters)
                }
                Ok(PowerShellOutput::Single(item)) => {
                    Ok(vec![NetworkAdapter {
                        name: item.name,
                        description: item.description,
                        ip_address: item.ip_address,
                        gateway: item.gateway,
                    }])
                }
                Err(e) => {
                    log::warn!("Failed to parse PowerShell JSON: {}, raw: {}", e, trimmed);
                    Ok(vec![])
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(vec![
                NetworkAdapter {
                    name: "Mock Ethernet 1".to_string(),
                    description: "Mock Realtek PCIe GBE Controller".to_string(),
                    ip_address: Some("192.168.1.100".to_string()),
                    gateway: Some("192.168.1.1".to_string()),
                },
                NetworkAdapter {
                    name: "Mock USB NDIS".to_string(),
                    description: "Mock Remote NDIS Internet Sharing Device".to_string(),
                    ip_address: Some("192.168.8.100".to_string()),
                    gateway: Some("192.168.8.1".to_string()),
                }
            ])
        }
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn connect_websocket(
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport_state = state.transport.clone();
    let vendor_state = state.vendor.clone();
    let conn_port_state = state.connected_port.clone();
    let at_log_state = state.at_command_log.clone();

    let host_clone = host.clone();
    let user_clone = username.clone();
    let pass_clone = password.clone();

    tokio::task::spawn_blocking(move || {
        let user = user_clone.as_deref().unwrap_or("admin");
        let pass = pass_clone.as_deref().unwrap_or("admin");

        let mut transport = modem_hal::transport::WebSocketTransport::new(
            &host_clone,
            port,
            Some(user),
            Some(pass),
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
        *transport_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? =
            Some(wrap_transport(Box::new(transport), at_log_state));

        log::info!("Detected vendor over WebSocket: {:?}", v.vendor());
        *vendor_state
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))? = Some(v);

        log::info!("Connected to WebSocket {}:{} as {}", host_clone, port, user);
        Ok(id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

// ── High-level modem queries (all async to avoid blocking UI) ──

#[tauri::command]
async fn get_modem_status(state: tauri::State<'_, AppState>) -> Result<ModemStatus, String> {
    with_vendor!(state, |t, v| v.query_modem_status(t))
}

#[tauri::command]
async fn get_hardware_info(state: tauri::State<'_, AppState>) -> Result<HardwareInfo, String> {
    with_vendor!(state, |t, v| v.query_hardware_info(t))
}

#[tauri::command]
async fn get_ip_info(state: tauri::State<'_, AppState>) -> Result<IpInfo, String> {
    with_vendor_cid!(state, |t, v, cid| v
        .query_ip_info(t, if cid > 0 { cid } else { 1 }))
}

#[tauri::command]
async fn get_apn_list(state: tauri::State<'_, AppState>) -> Result<Vec<ApnEntry>, String> {
    with_vendor!(state, |t, v| v.query_apn_list(t))
}

#[tauri::command]
async fn get_neighbor_cells(state: tauri::State<'_, AppState>) -> Result<NeighborCells, String> {
    with_vendor!(state, |t, v| v.query_neighbor_cells(t))
}

#[tauri::command]
async fn get_qos_info(state: tauri::State<'_, AppState>) -> Result<QosInfo, String> {
    with_vendor_cid!(state, |t, v, cid| v
        .query_qos(t, if cid > 0 { cid } else { 1 }))
}

#[tauri::command]
async fn get_network_mode(state: tauri::State<'_, AppState>) -> Result<String, String> {
    with_vendor!(state, |t, v| v.query_network_mode(t))
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
    validate_cid(cid)?;
    validate_at_string(&apn)?;
    validate_at_string(&username)?;
    validate_at_string(&password)?;
    with_vendor!(state, |t, v| v.set_apn(
        t,
        cid,
        context_type,
        &apn,
        &username,
        &password,
        auth_type
    ))
}

#[tauri::command]
async fn delete_apn_config(cid: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    validate_cid(cid)?;
    with_vendor!(state, |t, v| v.delete_apn(t, cid))
}

#[tauri::command]
async fn set_apn_active(
    cid: i32,
    active: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    with_vendor!(state, |t, v| v.set_apn_active(t, cid, active))
}

#[tauri::command]
async fn connect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        // Lock-free read: if it's still at the default 0 (uninitialised),
        // pick 1; otherwise use the stored CID.
        let cid = {
            let cur = data_cid.load(Ordering::Relaxed);
            if cur > 0 {
                cur
            } else {
                1
            }
        };
        v.connect_data(t, cid)?;
        // Lock-free write of the same CID back so subsequent calls
        // re-use the actually-connected one.
        data_cid.store(cid, Ordering::Relaxed);
        Ok(())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn get_5glan(state: tauri::State<'_, AppState>) -> Result<Vec<L5GanEntry>, String> {
    with_vendor!(state, |t, v| v.query_5glan(t))
}

#[tauri::command]
async fn set_5glan(
    cid: i32,
    enabled: bool,
    vlan_id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    with_vendor!(state, |t, v| v.set_5glan(t, cid, enabled, vlan_id))
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn configure_qualcomm_5glan(
    cid: i32,
    apn: String,
    snssai: String,
    profile_id: i32,
    vlan_start: i32,
    vlan_end: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    validate_at_string(&apn)?;
    validate_at_string(&snssai)?;
    with_vendor!(state, |t, v| v.configure_qualcomm_5glan(
        t, cid, &apn, &snssai, profile_id, vlan_start, vlan_end
    ))
}

#[tauri::command]
async fn enable_eth_pdu(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.enable_eth_pdu(t))
}

#[tauri::command]
async fn connect_qualcomm_5glan(
    rule_id: i32,
    cid: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.connect_qualcomm_5glan(t, rule_id, cid))
}

#[tauri::command]
async fn query_qualcomm_5glan_status(
    state: tauri::State<'_, AppState>,
) -> Result<Qualcomm5GlanStatus, String> {
    with_vendor!(state, |t, v| v.query_qualcomm_5glan_status(t))
}

#[tauri::command]
async fn get_vlan(state: tauri::State<'_, AppState>) -> Result<Vec<i32>, String> {
    with_vendor!(state, |t, v| v.query_vlan(t))
}

#[tauri::command]
async fn set_vlan(
    vlan_id: i32,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_vlan(t, vlan_id, enabled))
}

#[tauri::command]
async fn disconnect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor_cid!(state, |t, v, cid| v
        .disconnect_data(t, if cid > 0 { cid } else { 1 }))
}

#[tauri::command]
async fn set_network_mode_cmd(
    mode: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&mode)?;
    with_vendor!(state, |t, v| v.set_network_mode(t, &mode))
}

#[tauri::command]
async fn set_nr5g_band_cmd(band: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    validate_at_string(&band)?;
    with_vendor!(state, |t, v| v.set_nr5g_bands(t, &band))
}

#[tauri::command]
async fn reboot_modem(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.reboot(t))
}

#[tauri::command]
async fn set_cfun(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_cfun(t, mode))
}

#[tauri::command]
async fn factory_reset(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.factory_reset(t))
}

#[tauri::command]
async fn get_sim_slot(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_sim_slot(t))
}

#[tauri::command]
async fn set_sim_slot(slot: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.switch_sim_slot(t, slot))
}

#[tauri::command]
async fn send_raw_at(command: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    // SECURITY: send_raw_at is a powerful escape hatch (front-end can issue
    // arbitrary AT). It MUST validate complete AT commands while still allowing
    // quoted AT syntax such as AT+QCFG="ims".
    // See REVIEW.md#2.
    validate_raw_at_command(&command)?;
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        t.send_at(&command)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn query_cell_lock(state: tauri::State<'_, AppState>) -> Result<Vec<CellLockEntry>, String> {
    with_vendor!(state, |t, v| v.query_cell_lock(t))
}

#[tauri::command]
async fn set_cell_lock(
    arfcn: String,
    pci: String,
    scs: String,
    band: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&arfcn)?;
    validate_at_string(&pci)?;
    validate_at_string(&scs)?;
    validate_at_string(&band)?;
    with_vendor!(state, |t, v| v.set_cell_lock(t, &arfcn, &pci, &scs, &band))
}

#[tauri::command]
async fn clear_cell_lock(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.clear_cell_lock(t))
}

#[tauri::command]
async fn set_plmn_lock(
    plmn: String,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&plmn)?;
    let pw = password.ok_or_else(|| "PLMN lock requires a password (the device-specific unlock code, NOT the public default)".to_string())?;
    validate_at_string(&pw)?;
    with_vendor!(state, |t, v| v.set_plmn_lock(t, &plmn, &pw))
}

#[tauri::command]
async fn clear_plmn_lock(
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let pw = password.ok_or_else(|| "PLMN unlock requires a password".to_string())?;
    validate_at_string(&pw)?;
    with_vendor!(state, |t, v| v.clear_plmn_lock(t, &pw))
}

/// Return all AT commands logged by internal operations since last call, then clear the log.
#[tauri::command]
fn pop_at_commands(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut log = state
        .at_command_log
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;
    // drain(..) empties the VecDeque in place and yields owned elements.
    Ok(log.drain(..).collect())
}

#[tauri::command]
async fn get_bands(state: tauri::State<'_, AppState>) -> Result<BandConfig, String> {
    with_vendor!(state, |t, v| v.query_bands_with_spec(t))
}

#[tauri::command]
async fn set_bands(
    lte: String,
    nr: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&lte)?;
    validate_at_string(&nr)?;
    with_vendor!(state, |t, v| v.set_bands(t, &lte, &nr))
}

#[tauri::command]
async fn reset_all_bands(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.reset_all_bands(t))
}

#[tauri::command]
async fn get_feature_toggles(state: tauri::State<'_, AppState>) -> Result<FeatureToggles, String> {
    with_vendor!(state, |t, v| v.query_feature_toggles(t))
}

#[tauri::command]
async fn set_feature_toggle(
    feature: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&feature)?;
    with_vendor!(state, |t, v| v.set_feature_toggle(t, &feature, enabled))
}

#[tauri::command]
async fn get_qualcomm_config(state: tauri::State<'_, AppState>) -> Result<QualcommConfig, String> {
    with_vendor!(state, |t, v| v.query_qualcomm_config(t))
}

#[tauri::command]
async fn set_qualcomm_config(
    param: String,
    value: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&param)?;
    validate_at_string(&value)?;
    with_vendor!(state, |t, v| v.set_qualcomm_config(t, &param, &value))
}

#[tauri::command]
async fn get_traffic(state: tauri::State<'_, AppState>) -> Result<TrafficInfo, String> {
    with_vendor!(state, |t, v| v.query_traffic(t))
}

#[tauri::command]
async fn get_usbnet_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_usbnet_mode(t))
}

#[tauri::command]
async fn set_usbnet_mode(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_usbnet_mode(t, mode))
}

#[tauri::command]
async fn get_nat_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_nat_mode(t))
}

#[tauri::command]
async fn set_nat_mode(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_nat_mode(t, mode))
}

#[tauri::command]
async fn set_mqtt_enabled(enabled: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut task_guard = state.mqtt_task.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
    if enabled {
        if task_guard.is_none() {
            log::info!("MQTT: Enabling remote connection...");
            let transport = state.transport.clone();
            let vendor = state.vendor.clone();
            let handle = tokio::spawn(async move {
                mqtt::run_mqtt_loop(transport, vendor).await;
            });
            *task_guard = Some(handle);
        }
    } else {
        if let Some(handle) = task_guard.take() {
            log::info!("MQTT: Disabling remote connection...");
            handle.abort();
        }
    }
    Ok(())
}

#[tauri::command]
async fn get_mqtt_enabled(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let task_guard = state.mqtt_task.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
    Ok(task_guard.is_some())
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

                        let current_names: HashSet<String> =
                            ports.iter().map(|p| p.port_name.clone()).collect();

                        let added: Vec<String> =
                            current_names.difference(&previous_ports).cloned().collect();
                        let removed: Vec<String> =
                            previous_ports.difference(&current_names).cloned().collect();
                        previous_ports = current_names;

                        if added.is_empty() && removed.is_empty() {
                            continue;
                        }

                        log::info!(
                            "[USB监控] 端口变化 — 新增: {:?}, 移除: {:?}",
                            added,
                            removed
                        );

                        if let Err(e) = app_handle.emit(
                            "port-changed",
                            PortChangeEvent {
                                added: added.clone(),
                                removed: removed.clone(),
                            },
                        ) {
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

                // Use try_lock so heartbeat NEVER blocks IPC handlers.
                // If another thread holds the transport/port lock (long AT command
                // in flight, IPC handler, reconnect) we simply skip this tick and
                // retry on the next 4 s cycle. Without try_lock the heartbeat
                // could block 3-8 s behind a single AT command and miss the
                // USB-disconnect window.
                let port_name = match connected_port.try_lock() {
                    Ok(g) => g.clone(),
                    Err(_) => continue,
                };
                let Some(port) = port_name else { continue };

                let alive = match transport.try_lock() {
                    Ok(g) => match g.as_deref() {
                        Some(t) => t.is_alive(),
                        None => continue,
                    },
                    Err(_) => continue,
                };

                if !alive {
                    log::warn!("[心跳] 检测到硬件断连，端口: {}", port);
                    // Clear connected_port immediately so subsequent ticks don't re-fire
                    if let Ok(mut g) = connected_port.try_lock() {
                        *g = None;
                    }
                    if let Err(e) = app_handle.emit(
                        "port-changed",
                        PortChangeEvent {
                            added: vec![],
                            removed: vec![port],
                        },
                    ) {
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
            data_cid: Arc::new(AtomicI32::new(1)),
            connected_port: Arc::new(Mutex::new(None)),
            at_command_log: Arc::new(Mutex::new(VecDeque::new())),
            mqtt_task: Arc::new(Mutex::new(None)),
        })
        .setup(|app| {
            // ── Build menu bar ──
            let about = MenuItemBuilder::with_id("about", "关于 Modem Cat").build(app)?;
            let help_menu = SubmenuBuilder::new(app, "帮助").item(&about).build()?;
            let menu = MenuBuilder::new(app).item(&help_menu).build()?;
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

            let show_item =
                tauri::menu::MenuItemBuilder::with_id("show_window", "控制面板").build(app)?;
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

            tray_icon.on_menu_event(|app, event| match event.id.as_ref() {
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
            });

            tray_icon.on_tray_icon_event(|tray, event| {
                if let tauri::tray::TrayIconEvent::Click {
                    button: tauri::tray::MouseButton::Left,
                    ..
                } = event
                {
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
            list_network_adapters,
            connect_websocket,
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
            configure_qualcomm_5glan,
            enable_eth_pdu,
            connect_qualcomm_5glan,
            query_qualcomm_5glan_status,
            get_app_version,
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
            set_mqtt_enabled,
            get_mqtt_enabled,
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
