//! License loading, verification, and IPC for Modem Cat.

use crate::AppState;
use modem_license::{LicensePayload, LicenseStatus};
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Emitter;

/// Try to load and verify the license file from next to the executable.
pub fn init_license(_app: &AppHandle) -> Option<LicensePayload> {
    // Look for license.dat alongside the executable
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

    if let Some(dir) = &exe_dir {
        let path = dir.join("license.dat");
        if path.exists() {
            match modem_license::load_and_verify(&path) {
                Ok(payload) => {
                    log::info!(
                        "License 已激活: {} | factory={} firmware={}",
                        payload.licensee,
                        payload.features.factory_mode,
                        payload.features.firmware_download
                    );
                    return Some(payload);
                }
                Err(e) => {
                    log::warn!("License 验证失败: {e}");
                    return None;
                }
            }
        }
    }
    log::info!("未找到 License 文件，工厂模式和固件下载功能未激活");
    None
}

/// Reload license from a user-selected file path.
pub fn reload_license(path: &str) -> Result<LicensePayload, String> {
    modem_license::load_and_verify(path).map_err(|e| e.to_string())
}

/// Build a frontend-friendly status object from an optional payload.
pub fn status_from_payload(payload: &Option<LicensePayload>) -> LicenseStatus {
    match payload {
        Some(p) => LicenseStatus {
            valid: true,
            factory_mode: p.features.factory_mode,
            firmware_download: p.features.firmware_download,
            licensee: p.licensee.clone(),
            expires_at: p.expires_at,
            mac: p.mac.clone(),
            error: String::new(),
        },
        None => LicenseStatus {
            valid: false,
            factory_mode: false,
            firmware_download: false,
            licensee: String::new(),
            expires_at: 0,
            mac: String::new(),
            error: if std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("license.dat")))
                .map_or(true, |p| !p.exists())
            {
                "未找到 license.dat 文件".into()
            } else {
                "License 无效或已过期".into()
            },
        },
    }
}

/// Update the shared AppState license field.
pub fn update_license_state(
    license: &Arc<std::sync::Mutex<Option<LicensePayload>>>,
    new_value: Option<LicensePayload>,
) {
    if let Ok(mut guard) = license.lock() {
        *guard = new_value;
    } else {
        log::error!("Failed to lock license state for update");
    }
}

/// IPC: Return the current license status to the frontend.
///
/// Returns `Result` instead of panicking on a poisoned lock, so a concurrent
/// `load_license_file` panic cannot take down every subsequent IPC call
/// (AGENTS.md: "运行时锁路径禁止 panic").
#[tauri::command]
pub fn get_license_status(state: tauri::State<'_, AppState>) -> Result<LicenseStatus, String> {
    let guard = state
        .license
        .lock()
        .map_err(|e| format!("license lock poisoned: {e}"))?;
    Ok(status_from_payload(&*guard))
}

/// IPC: Load and verify a license file from a user-selected path.
#[tauri::command]
pub fn load_license_file(
    path: String,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<LicenseStatus, String> {
    let payload = reload_license(&path)?;
    let status = LicenseStatus {
        valid: true,
        factory_mode: payload.features.factory_mode,
        firmware_download: payload.features.firmware_download,
        licensee: payload.licensee.clone(),
        expires_at: payload.expires_at,
        mac: payload.mac.clone(),
        error: String::new(),
    };
    update_license_state(&state.license, Some(payload));
    let _ = app.emit("license-changed", &status);
    Ok(status)
}
