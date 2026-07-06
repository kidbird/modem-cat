use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::{connection, AppState};
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PortChangeEntry {
    pub port_name: String,
    pub timestamp: String,
    pub usb_vid: Option<u16>,
    pub usb_pid: Option<u16>,
    pub detected_model: Option<String>,
    pub detected_chipset: Option<String>,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PortChangeEvent {
    pub added: Vec<PortChangeEntry>,
    pub removed: Vec<PortChangeEntry>,
}

fn now_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn make_entry(port: &modem_hal::types::PortInfo, timestamp: &str) -> PortChangeEntry {
    PortChangeEntry {
        port_name: port.port_name.clone(),
        timestamp: timestamp.to_string(),
        usb_vid: port.usb_vid,
        usb_pid: port.usb_pid,
        detected_model: port.detected_model.clone(),
        detected_chipset: port.detected_chipset.clone(),
    }
}

pub(crate) fn start_port_monitor(app_handle: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || loop {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut previous_ports: HashMap<String, modem_hal::types::PortInfo> =
                    connection::snapshot_ports()
                        .map(|ports| {
                            ports
                                .into_iter()
                                .map(|port| (port.port_name.clone(), port))
                                .collect()
                        })
                        .unwrap_or_default();
                loop {
                    std::thread::sleep(Duration::from_secs(2));

                    let current_ports: HashMap<String, modem_hal::types::PortInfo> =
                        match connection::snapshot_ports() {
                            Ok(ports) => ports
                                .into_iter()
                                .map(|port| (port.port_name.clone(), port))
                                .collect(),
                            Err(e) => {
                                log::warn!("[USB监控] snapshot_ports 失败: {}", e);
                                continue;
                            }
                        };

                    let timestamp = now_timestamp();
                    let mut added: Vec<PortChangeEntry> = current_ports
                        .iter()
                        .filter(|(name, _)| !previous_ports.contains_key(*name))
                        .map(|(_, port)| make_entry(port, &timestamp))
                        .collect();
                    let mut removed: Vec<PortChangeEntry> = previous_ports
                        .iter()
                        .filter(|(name, _)| !current_ports.contains_key(*name))
                        .map(|(_, port)| make_entry(port, &timestamp))
                        .collect();

                    previous_ports = current_ports;

                    if added.is_empty() && removed.is_empty() {
                        continue;
                    }

                    added.sort_by(|left, right| left.port_name.cmp(&right.port_name));
                    removed.sort_by(|left, right| left.port_name.cmp(&right.port_name));

                    log::info!(
                        "[USB监控] 端口变化 — 新增: {:?}, 移除: {:?}",
                        added
                            .iter()
                            .map(|item| format!(
                                "{} {}:{} {}",
                                item.port_name,
                                item.usb_vid
                                    .map(|value| format!("{:04X}", value))
                                    .unwrap_or_else(|| "----".to_string()),
                                item.usb_pid
                                    .map(|value| format!("{:04X}", value))
                                    .unwrap_or_else(|| "----".to_string()),
                                item.detected_chipset.as_deref().unwrap_or("unknown")
                            ))
                            .collect::<Vec<_>>(),
                        removed
                            .iter()
                            .map(|item| format!(
                                "{} {}:{} {}",
                                item.port_name,
                                item.usb_vid
                                    .map(|value| format!("{:04X}", value))
                                    .unwrap_or_else(|| "----".to_string()),
                                item.usb_pid
                                    .map(|value| format!("{:04X}", value))
                                    .unwrap_or_else(|| "----".to_string()),
                                item.detected_chipset.as_deref().unwrap_or("unknown")
                            ))
                            .collect::<Vec<_>>()
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
        })
        .expect("无法创建 USB 监控线程");
}

pub(crate) fn start_connection_heartbeat(app_handle: tauri::AppHandle, state: AppState) {
    std::thread::Builder::new()
        .name("connection-heartbeat".into())
        .spawn(move || {
            // Consecutive failure counter — a single transient is_alive()
            // failure must NOT trigger a disconnect. Windows USB-serial
            // drivers can briefly return Err from bytes_to_read() while the
            // port is busy with an AT command. Require 2 consecutive
            // failures before declaring the hardware dead.
            let mut consecutive_failures: u32 = 0;
            const FAILURE_THRESHOLD: u32 = 2;

            loop {
                std::thread::sleep(Duration::from_secs(5));

                // If a disconnect is already in progress (user clicked or
                // USB-monitor detected), back off — don't race.
                if state.disconnecting.load(Ordering::Relaxed) {
                    consecutive_failures = 0;
                    continue;
                }

                let port_name = match state.connected_port.try_lock() {
                    Ok(g) => g.clone(),
                    Err(_) => continue,
                };
                let Some(port) = port_name else {
                    consecutive_failures = 0;
                    continue;
                };

                let alive = match state.transport.try_lock() {
                    Ok(g) => g.as_ref().map(|t| t.is_alive()).unwrap_or(false),
                    // Port busy with AT command — can't check, reset counter.
                    Err(_) => {
                        consecutive_failures = 0;
                        continue;
                    }
                };

                if alive {
                    consecutive_failures = 0;
                } else {
                    consecutive_failures += 1;
                    log::warn!(
                        "[心跳] is_alive() 失败 #{}/{} 端口: {}",
                        consecutive_failures,
                        FAILURE_THRESHOLD,
                        port
                    );
                    if consecutive_failures < FAILURE_THRESHOLD {
                        continue;
                    }

                    log::warn!(
                        "[心跳] 连续 {} 次失败，确认硬件断连，端口: {}",
                        consecutive_failures,
                        port
                    );
                    consecutive_failures = 0;
                    let usb_ids = match state.connected_usb_ids.lock() {
                        Ok(guard) => *guard,
                        Err(_) => None,
                    };
                    let removed_entry = PortChangeEntry {
                        port_name: port.clone(),
                        timestamp: now_timestamp(),
                        usb_vid: usb_ids.map(|(vid, _)| vid),
                        usb_pid: usb_ids.map(|(_, pid)| pid),
                        detected_model: usb_ids
                            .and_then(|(vid, pid)| {
                                modem_hal::ModemFactory::detect_model_from_vid_pid(vid, pid)
                            })
                            .map(str::to_string),
                        detected_chipset: usb_ids
                            .and_then(|(vid, pid)| {
                                modem_hal::ModemFactory::detect_vendor_from_vid_pid(vid, pid)
                            })
                            .map(|vendor| vendor.as_str().to_string()),
                    };
                    // Coordinate with other disconnect paths.
                    state.disconnecting.store(true, Ordering::SeqCst);
                    // Best-effort: fully tear down transport + vendor + port.
                    // We can't .await across threads, so do it inline.
                    if let (Ok(mut tg), Ok(mut vg), Ok(mut pg), Ok(mut ug)) = (
                        state.transport.lock(),
                        state.vendor.lock(),
                        state.connected_port.lock(),
                        state.connected_usb_ids.lock(),
                    ) {
                        if let Some(ref mut t) = *tg {
                            t.force_shutdown();
                        }
                        *tg = None;
                        *vg = None;
                        *pg = None;
                        *ug = None;
                    }
                    state.disconnecting.store(false, Ordering::SeqCst);

                    if let Err(e) = app_handle.emit(
                        "port-changed",
                        PortChangeEvent {
                            added: vec![],
                            removed: vec![removed_entry],
                        },
                    ) {
                        log::warn!("[心跳] 发送断连事件失败: {}", e);
                    }
                }
            }
        })
        .expect("无法创建心跳线程");
}
