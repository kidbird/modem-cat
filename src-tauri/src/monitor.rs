use std::collections::HashSet;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::AppState;
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
pub(crate) struct PortChangeEvent {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

pub(crate) fn start_port_monitor(app_handle: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || loop {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
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

                    log::warn!("[心跳] 连续 {} 次失败，确认硬件断连，端口: {}", consecutive_failures, port);
                    consecutive_failures = 0;
                    // Coordinate with other disconnect paths.
                    state.disconnecting.store(true, Ordering::SeqCst);
                    // Best-effort: fully tear down transport + vendor + port.
                    // We can't .await across threads, so do it inline.
                    if let (Ok(mut tg), Ok(mut vg), Ok(mut pg)) = (
                        state.transport.lock(),
                        state.vendor.lock(),
                        state.connected_port.lock(),
                    ) {
                        if let Some(ref mut t) = *tg {
                            t.force_shutdown();
                        }
                        *tg = None;
                        *vg = None;
                        *pg = None;
                    }
                    state.disconnecting.store(false, Ordering::SeqCst);

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
