use modem_hal::transport::AtTransport;
use modem_hal::types::*;
use modem_hal::ModemFactory;

use crate::wrap_transport;
use crate::AppState;

// ── Windows registry helper ──

/// On Windows, read the friendly name for all COM ports from the registry.
/// Returns a HashMap<port_name, (friendly_name, manufacturer)>.
#[cfg(target_os = "windows")]
pub fn get_windows_all_port_info(
) -> std::collections::HashMap<String, (Option<String>, Option<String>)> {
    use std::collections::HashMap;
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

                result.insert(port_name, (friendly_name, manufacturer));
            }
        }
    }

    result
}

#[cfg(not(target_os = "windows"))]
pub fn get_windows_all_port_info(
) -> std::collections::HashMap<String, (Option<String>, Option<String>)> {
    std::collections::HashMap::new()
}

// ── Port display helpers ──

pub fn build_display_name(
    port_name: &str,
    description: &Option<String>,
    manufacturer: &Option<String>,
    is_at_port: bool,
) -> String {
    if let Some(desc) = description {
        if !desc.is_empty() {
            let cleaned = regex_remove_com_suffix(desc);
            if is_at_port {
                return format!("{} [AT]", cleaned);
            }
            return cleaned;
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
            format!("{} - AT端口", parts.join(" - "))
        }
    } else if parts.is_empty() {
        port_name.to_string()
    } else {
        parts.join(" - ")
    }
}

/// Remove trailing "(COMx)" suffix from a string.
pub fn regex_remove_com_suffix(s: &str) -> String {
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

/// Determine if a serial port is likely an AT command port.
pub fn is_at_port(
    _port_name: &str,
    description: &Option<&String>,
    manufacturer: &Option<&String>,
) -> bool {
    let desc_upper = description.map_or_else(String::new, |s| s.to_uppercase());
    let mfg_upper = manufacturer.map_or_else(String::new, |s| s.to_uppercase());

    // 1. 全局排除绝对不能用作 AT 的端口
    if desc_upper.contains("NMEA")
        || desc_upper.contains("DIAG")
        || desc_upper.contains("DEBUG")
        || desc_upper.contains("GNSS")
        || desc_upper.contains("LOG")
    {
        return false;
    }

    // 2. 检查是否属于知名调制解调器厂商（加入 QUALCOMM / QCOM）
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
        || mfg_upper.contains("BROADMOBI")
        || mfg_upper.contains("QUALCOMM")
        || mfg_upper.contains("QCOM");

    // 3. 如果属于调制解调器厂商，优先判定其是否为 AT/Modem 端口
    if is_modem_manufacturer {
        if desc_upper.contains("MODEM")
            || desc_upper.contains("COMMAND")
            || desc_upper.contains("AT")
        {
            return true;
        }
        // 对于调制解调器厂商，若描述为空，通常也是候选 AT 端口
        if desc_upper.is_empty() {
            return true;
        }
    }

    // 4. 兜底判定：如果描述中显式包含 "AT"、"MODEM" 或 "COMMAND" 字样，也认为是 AT 候选
    if desc_upper
        .split(|c: char| !c.is_alphabetic())
        .any(|word| word == "AT")
        || desc_upper.contains("MODEM")
        || desc_upper.contains("COMMAND")
    {
        return true;
    }

    false
}

// ── Tauri commands ──

#[tauri::command]
pub fn list_ports() -> Result<Vec<PortInfo>, String> {
    let ports =
        serialport::available_ports().map_err(|e| format!("Failed to list ports: {}", e))?;

    let win_info = get_windows_all_port_info();

    let result: Vec<PortInfo> = ports
        .into_iter()
        .map(|port| {
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

#[tauri::command]
pub async fn auto_connect_at(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let ports =
        serialport::available_ports().map_err(|e| format!("Failed to list ports: {}", e))?;

    let win_info = get_windows_all_port_info();

    let mut at_candidates: Vec<String> = Vec::new();

    for port in &ports {
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
                log::info!("Connected to AT port: {}", port_name);
                *transport_arc.lock().unwrap() = Some(wrap_transport(
                    Box::new(transport),
                    state.at_command_log.clone(),
                ));
                match vendor_result {
                    Ok(vendor) => {
                        log::info!("Detected vendor: {:?}", vendor.vendor());
                        *vendor_arc.lock().unwrap() = Some(vendor);
                    }
                    Err(e) => {
                        log::warn!("Failed to detect modem vendor: {}", e);
                    }
                }
                let cp = state.connected_port.clone();
                *cp.lock().unwrap() = Some(port_name.clone());
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

#[tauri::command]
pub fn connect_serial(
    port_name: String,
    baud_rate: u32,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut transport = modem_hal::transport::SerialTransport::new(&port_name, baud_rate)?;
    let vendor = ModemFactory::create(&mut transport);
    let id = format!("serial_{}", port_name);
    *state.transport.lock().unwrap() = Some(wrap_transport(
        Box::new(transport),
        state.at_command_log.clone(),
    ));
    if let Ok(v) = vendor {
        log::info!("Detected vendor: {:?}", v.vendor());
        *state.vendor.lock().unwrap() = Some(v);
    } else if let Err(e) = vendor {
        log::warn!("Vendor detection failed: {}", e);
    }
    *state.connected_port.lock().unwrap() = Some(port_name.clone());
    log::info!(
        "Connected to serial port {} at {} baud",
        port_name,
        baud_rate
    );
    Ok(id)
}

#[tauri::command]
pub fn connect_tcp(
    host: String,
    port: u16,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut transport = modem_hal::transport::TcpTransport::new(&host, port)?;
    let vendor = ModemFactory::create(&mut transport);
    let id = format!("tcp_{}:{}", host, port);
    *state.connected_port.lock().unwrap() = None;
    *state.transport.lock().unwrap() = Some(wrap_transport(
        Box::new(transport),
        state.at_command_log.clone(),
    ));
    if let Ok(v) = vendor {
        log::info!("Detected vendor: {:?}", v.vendor());
        *state.vendor.lock().unwrap() = Some(v);
    } else if let Err(e) = vendor {
        log::warn!("Vendor detection failed: {}", e);
    }
    log::info!("Connected to TCP {}:{}", host, port);
    Ok(id)
}

#[tauri::command]
pub fn disconnect(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut t = state.transport.lock().unwrap();
    if let Some(ref mut transport) = *t {
        transport.close();
    }
    *t = None;
    *state.vendor.lock().unwrap() = None;
    *state.connected_port.lock().unwrap() = None;
    Ok("Disconnected".to_string())
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
pub fn list_network_adapters() -> Result<Vec<NetworkAdapter>, String> {
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
}

#[tauri::command]
pub fn connect_websocket(
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let user = username.unwrap_or_else(|| "admin".to_string());
    let pass = password.unwrap_or_else(|| "admin".to_string());

    let mut transport = modem_hal::transport::WebSocketTransport::new(
        &host,
        port,
        Some(&user),
        Some(&pass),
    )?;

    let vendor = ModemFactory::create(&mut transport);
    let id = format!("ws_{}:{}", host, port);
    *state.connected_port.lock().unwrap() = None;
    *state.transport.lock().unwrap() = Some(wrap_transport(
        Box::new(transport),
        state.at_command_log.clone(),
    ));

    if let Ok(v) = vendor {
        log::info!("Detected vendor over WebSocket: {:?}", v.vendor());
        *state.vendor.lock().unwrap() = Some(v);
    } else if let Err(e) = vendor {
        log::warn!("Vendor detection failed over WebSocket: {}", e);
    }

    log::info!("Connected to WebSocket {}:{} as {}", host, port, user);
    Ok(id)
}

