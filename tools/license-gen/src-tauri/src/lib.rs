use modem_license::{canonical_json, License, LicenseFeatures, LicensePayload};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn get_mac_addresses() -> Result<Vec<String>, String> {
    match mac_address::get_mac_address() {
        Ok(Some(mac)) => {
            let bytes = mac.bytes();
            let primary = bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>();
            Ok(vec![primary])
        }
        Ok(None) => Err("未检测到网卡 MAC 地址".into()),
        Err(e) => Err(format!("获取 MAC 地址失败: {e}")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateRequest {
    mac: String,
    expires_at: String,
    factory_mode: bool,
    firmware_download: bool,
    licensee: String,
    note: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateResult {
    success: bool,
    message: String,
    path: Option<String>,
    preview: Option<String>,
}

#[tauri::command]
fn generate_license(
    app: tauri::AppHandle,
    req: GenerateRequest,
) -> Result<GenerateResult, String> {
    let expires_at: i64 = if req.expires_at == "0" || req.expires_at.is_empty() {
        0
    } else {
        let dt = chrono::NaiveDate::parse_from_str(&req.expires_at, "%Y-%m-%d")
            .map_err(|e| format!("日期格式错误: {e} (应为 YYYY-MM-DD)"))?;
        let dt = dt.and_hms_opt(23, 59, 59).unwrap();
        dt.and_utc().timestamp()
    };

    let now = chrono::Utc::now().timestamp();

    let payload = LicensePayload {
        mac: req.mac.to_uppercase(),
        issued_at: now,
        expires_at,
        features: LicenseFeatures {
            factory_mode: req.factory_mode,
            firmware_download: req.firmware_download,
        },
        licensee: req.licensee,
        note: req.note,
    };

    let payload_bytes = canonical_json(&payload).map_err(|e| e.to_string())?;
    let pkcs8 = load_private_key()?;
    let signature = modem_license::sign_message(&pkcs8, &payload_bytes)?;

    let license = License {
        version: 1,
        payload: modem_license::base64_encode(&payload_bytes),
        signature: modem_license::base64_encode(&signature),
    };

    let license_json =
        serde_json::to_string_pretty(&license).map_err(|e| format!("序列化失败: {e}"))?;

    let save_path = tauri::async_runtime::block_on(async {
        app.dialog()
            .file()
            .add_filter("License File", &["dat"])
            .set_file_name("license.dat")
            .blocking_save_file()
    });

    match save_path {
        Some(path) => {
            let path = path
                .to_string()
                .strip_prefix("file://")
                .unwrap_or(&path.to_string())
                .to_string();
            fs::write(&path, &license_json)
                .map_err(|e| format!("写入文件失败: {e}"))?;
            Ok(GenerateResult {
                success: true,
                message: format!("License 文件已保存到 {}", path),
                path: Some(path),
                preview: Some(license_json),
            })
        }
        None => Ok(GenerateResult {
            success: false,
            message: "已取消保存".into(),
            path: None,
            preview: Some(license_json),
        }),
    }
}

#[tauri::command]
fn verify_license_file(path: String) -> Result<String, String> {
    match modem_license::load_and_verify(&path) {
        Ok(payload) => {
            let expires = if payload.expires_at == 0 {
                "永不过期".to_string()
            } else {
                chrono::DateTime::from_timestamp(payload.expires_at, 0)
                    .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| payload.expires_at.to_string())
            };
            let mut features = Vec::new();
            if payload.features.factory_mode {
                features.push("工厂模式");
            }
            if payload.features.firmware_download {
                features.push("固件下载");
            }
            Ok(format!(
                "✅ License 有效\n\n授权方: {}\n有效期至: {}\n功能: {}\nMAC: {}",
                payload.licensee,
                expires,
                if features.is_empty() {
                    "无".into()
                } else {
                    features.join(", ")
                },
                payload.mac,
            ))
        }
        Err(e) => Ok(format!("❌ License 无效: {e}")),
    }
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn load_private_key() -> Result<Vec<u8>, String> {
    if let Ok(path) = std::env::var("MODEM_CAT_SK_PATH") {
        return fs::read(&path).map_err(|e| format!("读取私钥文件失败 ({path}): {e}"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sk_path = dir.join("keys").join("modem-cat.sk");
            if sk_path.exists() {
                return fs::read(&sk_path).map_err(|e| format!("读取私钥文件失败: {e}"));
            }
        }
    }
    let dev_path = std::path::Path::new("keys/modem-cat.sk");
    if dev_path.exists() {
        return fs::read(dev_path).map_err(|e| format!("读取私钥文件失败: {e}"));
    }
    Err("未找到私钥文件。请将私钥放到 keys/modem-cat.sk 或设置 MODEM_CAT_SK_PATH 环境变量".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![
            get_mac_addresses,
            generate_license,
            verify_license_file,
            get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running license-gen");
}
