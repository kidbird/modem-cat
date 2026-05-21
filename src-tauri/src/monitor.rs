use std::collections::HashSet;
use std::time::Duration;
use tauri::Emitter;

#[derive(Clone, serde::Serialize)]
pub struct PortChangeEvent {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

/// Polls serial ports every 2 seconds and emits `port-changed` events to the
/// frontend when devices are added or removed.
pub fn start_port_monitor(app_handle: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut previous_ports: HashSet<String> = HashSet::new();
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

                    log::info!("[USB监控] 端口变化 — 新增: {:?}, 移除: {:?}", added, removed);

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
                log::error!("[USB监控] 线程崩溃: {}", msg);
            }
        })
        .expect("无法创建 USB 监控线程");
}
