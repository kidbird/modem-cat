use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod serial;
mod network;

pub use serial::*;
pub use network::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    pub id: String,
    pub connected: bool,
}

pub struct AppState {
    pub connection: Mutex<Option<ConnectionState>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    log::info!("Starting Modem Cat application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            connection: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            serial::list_ports,
            serial::connect_serial,
            serial::disconnect_serial,
            serial::send_at_command,
            network::connect_tcp,
            network::disconnect_tcp,
            network::send_tcp_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
