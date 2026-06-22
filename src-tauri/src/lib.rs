use std::collections::VecDeque;
use std::sync::atomic::AtomicI32;
use std::sync::{Arc, Mutex};

use modem_hal::transport::AtTransport;
use modem_hal::ModemVendor;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Emitter;
use tauri::Manager;


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
    /// Loaded license payload (None = unlicensed or invalid).
    pub license: Arc<Mutex<Option<modem_license::LicensePayload>>>,
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
            let $c = data_cid.load(std::sync::atomic::Ordering::Relaxed);
            $body
        })
        .await
        .map_err(|e| format!("Task error: {}", e))?
    }};
}

mod connection;
mod handlers;
mod monitor;
mod mqtt;
pub mod license;
pub mod factory;
pub mod dloader;

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
            connection::run_after_at_probe_timeout(
                std::time::Duration::from_millis(10),
                async { Ok::<_, String>("transport") },
                move |transport| async move {
                    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
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
        assert!(connection::is_at_port(
            "COM5",
            &Some(&"Qualcomm HS-USB Android Modem".to_string()),
            &Some(&"Qualcomm".to_string())
        ));
        assert!(connection::is_at_port(
            "COM5",
            &Some(&"Qualcomm HS-USB Modem".to_string()),
            &Some(&"Qualcomm Incorporated".to_string())
        ));
        assert!(connection::is_at_port(
            "COM5",
            &Some(&"Quectel USB AT Port".to_string()),
            &Some(&"Quectel".to_string())
        ));

        // Test DM/Diag/QDLoader/NMEA ports (should be false)
        assert!(!connection::is_at_port(
            "COM4",
            &Some(&"Qualcomm HS-USB Diagnostics 9008".to_string()),
            &Some(&"Qualcomm".to_string())
        ));
        assert!(!connection::is_at_port(
            "COM4",
            &Some(&"Qualcomm HS-USB DM Port".to_string()),
            &Some(&"Qualcomm".to_string())
        ));
        assert!(!connection::is_at_port(
            "COM4",
            &Some(&"Quectel USB DM Port".to_string()),
            &Some(&"Quectel".to_string())
        ));
        assert!(!connection::is_at_port(
            "COM3",
            &Some(&"Quectel USB NMEA Port".to_string()),
            &Some(&"Quectel".to_string())
        ));
        assert!(!connection::is_at_port(
            "COM3",
            &Some(&"Qualcomm HS-USB QDLoader 9008".to_string()),
            &Some(&"Qualcomm".to_string())
        ));
    }

    #[tokio::test]
    async fn test_list_network_adapters() {
        let res = connection::list_network_adapters().await;
        println!("Network adapters: {:?}", res);
        assert!(res.is_ok());
    }
}

// ── Timing ──

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Modem Cat application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
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
            license: Arc::new(Mutex::new(None)),
        })
        .manage(factory::FactoryState::new())
        .manage(dloader::DloaderState::default())
        .setup(|app| {
            // ── Load license ──
            let state = app.state::<AppState>();
            let license_payload = license::init_license(&app.handle());
            license::update_license_state(&state.license, license_payload);

            // ── Build menu bar ──
            let about = MenuItemBuilder::with_id("about", "关于 Modem Cat").build(app)?;
            let load_license =
                MenuItemBuilder::with_id("load_license", "加载 License...").build(app)?;
            let license_status =
                MenuItemBuilder::with_id("license_status", "License 状态").build(app)?;
            let help_menu = SubmenuBuilder::new(app, "帮助")
                .item(&about)
                .separator()
                .item(&load_license)
                .item(&license_status)
                .build()?;
            let menu = MenuBuilder::new(app).item(&help_menu).build()?;
            app.set_menu(menu)?;
            app.on_menu_event(|handle, event| {
                match event.id().as_ref() {
                    "about" => {
                        let _ = handle.emit("show-about", ());
                    }
                    "load_license" => {
                        let _ = handle.emit("show-load-license", ());
                    }
                    "license_status" => {
                        let _ = handle.emit("show-license-status", ());
                    }
                    _ => {}
                }
            });

            monitor::start_port_monitor(app.handle().clone());

            let state = app.state::<AppState>();
            monitor::start_connection_heartbeat(
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
            connection::list_ports,
            connection::auto_connect_at,
            connection::connect_serial,
            connection::connect_tcp,
            connection::disconnect,
            connection::list_network_adapters,
            connection::connect_websocket,
            // High-level queries
            handlers::get_modem_status,
            handlers::get_hardware_info,
            handlers::get_ip_info,
            handlers::get_apn_list,
            handlers::get_neighbor_cells,
            handlers::get_qos_info,
            handlers::get_network_mode,
            handlers::get_bands,
            handlers::get_feature_toggles,
            handlers::get_usbnet_mode,
            handlers::get_traffic,
            handlers::get_qualcomm_config,
            // Write operations
            handlers::set_apn_config,
            handlers::delete_apn_config,
            handlers::set_apn_active,
            handlers::get_5glan,
            handlers::set_5glan,
            handlers::configure_qualcomm_5glan,
            handlers::enable_eth_pdu,
            handlers::connect_qualcomm_5glan,
            handlers::query_qualcomm_5glan_status,
            handlers::get_app_version,
            handlers::get_vlan,
            handlers::set_vlan,
            handlers::connect_data,
            handlers::disconnect_data,
            handlers::set_network_mode_cmd,
            handlers::set_nr5g_band_cmd,
            handlers::set_bands,
            handlers::reset_all_bands,
            handlers::set_feature_toggle,
            handlers::set_usbnet_mode,
            handlers::get_nat_mode,
            handlers::set_nat_mode,
            handlers::set_qualcomm_config,
            handlers::reboot_modem,
            handlers::set_cfun,
            handlers::factory_reset,
            handlers::get_sim_slot,
            handlers::set_sim_slot,
            handlers::send_raw_at,
            handlers::pop_at_commands,
            // Cell lock / PLMN lock
            handlers::query_cell_lock,
            handlers::set_cell_lock,
            handlers::clear_cell_lock,
            handlers::set_plmn_lock,
            handlers::clear_plmn_lock,
            handlers::set_mqtt_enabled,
            handlers::get_mqtt_enabled,
            // License
            license::get_license_status,
            license::load_license_file,
            // Factory mode
            factory::init_factory,
            factory::factory_get_base_data,
            factory::factory_get_current_product,
            factory::factory_set_product,
            factory::factory_get_current_sn,
            factory::factory_get_code_set,
            factory::factory_increment_sequence,
            factory::factory_set_device_ip,
            factory::factory_write_sn_to_device,
            factory::factory_get_device_info,
            factory::factory_save_execute_data,
            factory::factory_save_device_record,
            factory::factory_add_brand,
            factory::factory_remove_brand,
            factory::factory_add_product_type,
            factory::factory_remove_product_type,
            factory::factory_add_factory,
            factory::factory_remove_factory,
            // Firmware download
            dloader::pick_pac_file,
            dloader::pac_info,
            dloader::start_firmware_download,
            dloader::stop_firmware_download,
        ])
        .on_window_event(|window, event| {
            // Kill sidecar if window closes mid-flash.
            if matches!(
                event,
                tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed
            ) {
                if let Some(child) = window
                    .state::<dloader::DloaderState>()
                    .child
                    .lock()
                    .unwrap()
                    .take()
                {
                    let _ = child.kill();
                }
            }
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
