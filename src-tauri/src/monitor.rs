use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use modem_hal::transport::AtTransport;
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
pub(crate) struct PortChangeEvent {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

pub(crate) fn start_port_monitor(app_handle: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || {
            loop {
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
            }
        })
        .expect("无法创建 USB 监控线程");
}

pub(crate) fn start_connection_heartbeat(
    app_handle: tauri::AppHandle,
    transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    connected_port: Arc<Mutex<Option<String>>>,
) {
    std::thread::Builder::new()
        .name("connection-heartbeat".into())
        .spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(4));

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
