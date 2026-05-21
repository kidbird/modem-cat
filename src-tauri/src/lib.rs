use std::sync::{Arc, Mutex};

use modem_hal::transport::AtTransport;
use modem_hal::ModemVendor;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Emitter;
use tauri::Manager;

pub mod commands;
pub mod monitor;
pub mod ports;

use commands::*;
use monitor::start_port_monitor;
use ports::*;

pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    pub vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
    pub data_cid: Arc<Mutex<i32>>,
    pub active_cids: Arc<Mutex<Vec<i32>>>,
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
        self.log.lock().unwrap().push(command.to_string());
        self.inner.send_at(command)
    }
    fn close(&mut self) {
        self.inner.close();
    }
}

pub fn wrap_transport(
    transport: Box<dyn AtTransport>,
    log: Arc<Mutex<Vec<String>>>,
) -> Box<dyn AtTransport> {
    Box::new(LoggingTransport {
        inner: transport,
        log,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Modem Cat application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.webview_windows().get("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .manage(AppState {
            transport: Arc::new(Mutex::new(None)),
            vendor: Arc::new(Mutex::new(None)),
            data_cid: Arc::new(Mutex::new(1)),
            active_cids: Arc::new(Mutex::new(Vec::new())),
            connected_port: Arc::new(Mutex::new(None)),
            at_command_log: Arc::new(Mutex::new(Vec::new())),
        })
        .setup(|app| {
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

            let show_item =
                tauri::menu::MenuItemBuilder::with_id("show_window", "控制面板").build(app)?;
            let quit_item =
                tauri::menu::MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let tray_menu = tauri::menu::MenuBuilder::new(app)
                .item(&show_item)
                .item(&quit_item)
                .build()?;

            let tray_icon = app.tray_by_id("main").expect("tray not configured");
            tray_icon.set_menu(Some(tray_menu)).unwrap();

            tray_icon.on_menu_event(|app, event| match event.id.as_ref() {
                "show_window" => {
                    if let Some(window) = app.webview_windows().get("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
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
                        window.show().unwrap();
                        window.set_focus().unwrap();
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
            // Modem queries
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
            get_5glan,
            get_sim_slot,
            // Write operations
            set_apn_config,
            delete_apn_config,
            set_apn_active,
            set_5glan,
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
            set_sim_slot,
            send_raw_at,
            pop_at_commands,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
