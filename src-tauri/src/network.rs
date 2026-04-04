use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use tauri::State;
use std::time::Duration;

use crate::{AppState, ConnectionState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConnectParams {
    pub host: String,
    pub port: u16,
}

#[tauri::command]
pub fn connect_tcp(
    host: String,
    port: u16,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("Invalid address: {}", e))?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

    stream.set_read_timeout(Some(Duration::from_secs(2))).ok();

    let connection_id = format!("tcp_{}:{}", host, port);

    let mut conn = state.connection.lock().unwrap();
    *conn = Some(ConnectionState {
        id: connection_id.clone(),
        connected: true,
    });

    log::info!("Connected to TCP {}:{}", host, port);
    Ok(connection_id)
}

#[tauri::command]
pub fn disconnect_tcp(state: State<'_, AppState>) -> Result<String, String> {
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
pub fn send_tcp_command(
    command: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let conn = state.connection.lock().unwrap();

    if conn.is_none() {
        return Err("Not connected".to_string());
    }

    // In a real implementation, this would send the command via TCP
    // and read the response. For now, we return a mock response.
    let response = format!("{}\r\nOK\r\n", command);
    Ok(response)
}
