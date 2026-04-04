use serialport::SerialPortType;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::State;

use crate::{AppState, ConnectionState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub path: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
}

#[tauri::command]
pub fn list_ports() -> Result<Vec<PortInfo>, String> {
    let ports = serialport::available_ports()
        .map_err(|e| format!("Failed to list ports: {}", e))?;

    let result: Vec<PortInfo> = ports
        .into_iter()
        .map(|port| {
            let (description, manufacturer) = match port.port_type {
                SerialPortType::UsbPort(info) => (
                    info.product.map(|s| s),
                    info.manufacturer.map(|s| s),
                ),
                SerialPortType::PciPort => (Some("PCI Port".to_string()), None),
                SerialPortType::BluetoothPort => (Some("Bluetooth".to_string()), None),
                SerialPortType::Unknown => (None, None),
            };

            PortInfo {
                path: port.port_name,
                description,
                manufacturer,
            }
        })
        .collect();

    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConnectParams {
    pub port: String,
    pub baud_rate: u32,
    pub data_bits: Option<u8>,
    pub stop_bits: Option<u8>,
    pub parity: Option<String>,
}

#[tauri::command]
pub fn connect_serial(
    port_name: String,
    baud_rate: u32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut builder = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(1000));

    let _port = builder.open()
        .map_err(|e| format!("Failed to open port {}: {}", port_name, e))?;

    let connection_id = format!("serial_{}", port_name);

    let mut conn = state.connection.lock().unwrap();
    *conn = Some(ConnectionState {
        id: connection_id.clone(),
        connected: true,
    });

    log::info!("Connected to serial port {} at {} baud", port_name, baud_rate);
    Ok(connection_id)
}

#[tauri::command]
pub fn disconnect_serial(state: State<'_, AppState>) -> Result<String, String> {
    let mut conn = state.connection.lock().unwrap();
    if let Some(ref c) = *conn {
        let msg = format!("Disconnected from {}", c.id);
        *conn = None;
        Ok(msg)
    } else {
        Err("No active connection".to_string())
    }
}

#[tauri::command]
pub fn send_at_command(
    command: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let conn = state.connection.lock().unwrap();

    if conn.is_none() {
        return Err("Not connected".to_string());
    }

    // In a real implementation, this would send the command to the serial port
    // and read the response. For now, we return a mock response.
    let response = format!("{}\r\nOK\r\n", command);
    Ok(response)
}
