//! Firmware download integration: drives the 32-bit `r26-cli` sidecar.
//!
//! modem-cat stays 64-bit and never links the Unisoc DLLs. It spawns the
//! self-contained 32-bit `r26-cli.exe` sidecar, decoding its JSON event
//! output into Tauri `firmware-event`s for the UI.

use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::process::{Command, CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

// -- Sidecar version gate ----------------------------------------------------

/// Version the vendored sidecar must report via `r26-cli --version`.
const SIDECAR_EXPECTED_VERSION: &str = "0.1.0";

/// Cached once per process -- the sidecar binary can't change while we run.
static SIDECAR_VERSION_CHECK: OnceLock<Result<(), String>> = OnceLock::new();

const SIDECAR_RUNTIME_DLL: &str = "vcruntime140.dll";
const SIDECAR_RUNTIME_RESOURCE_DIR: &str = "r26-runtime";

fn resolve_sidecar_runtime_dir_from_paths_with_repo_fallback(
    resource_dir: Option<&Path>,
    exe_dir: Option<&Path>,
    include_repo_fallback: bool,
) -> Result<PathBuf, String> {
    let mut candidates = Vec::new();

    if let Some(exe_dir) = exe_dir {
        candidates.push(exe_dir.to_path_buf());
    }
    if let Some(resource_dir) = resource_dir {
        candidates.push(resource_dir.join(SIDECAR_RUNTIME_RESOURCE_DIR));
    }

    if include_repo_fallback {
        let manifest_runtime_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join(SIDECAR_RUNTIME_RESOURCE_DIR);
        if !candidates.iter().any(|candidate| candidate == &manifest_runtime_dir) {
            candidates.push(manifest_runtime_dir);
        }
    }

    candidates
        .into_iter()
        .find(|dir| dir.join(SIDECAR_RUNTIME_DLL).is_file())
        .ok_or_else(|| {
            format!(
                "未找到刷机组件依赖 {SIDECAR_RUNTIME_DLL}；请重新执行 build.ps1 或 scripts/build-helper.ps1 以打包 x86 VC 运行库"
            )
        })
}

fn resolve_sidecar_runtime_dir_from_paths(
    resource_dir: Option<&Path>,
    exe_dir: Option<&Path>,
) -> Result<PathBuf, String> {
    resolve_sidecar_runtime_dir_from_paths_with_repo_fallback(resource_dir, exe_dir, true)
}

fn resolve_sidecar_runtime_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let resource_dir = app.path().resource_dir().ok();
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));
    resolve_sidecar_runtime_dir_from_paths(resource_dir.as_deref(), exe_dir.as_deref())
}

fn sidecar_runtime_path_env(runtime_dir: &Path) -> Result<OsString, String> {
    let mut paths = vec![runtime_dir.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    std::env::join_paths(paths).map_err(|e| format!("构造刷机组件 PATH 失败: {e}"))
}

fn sidecar_command(app: &AppHandle) -> Result<Command, String> {
    let runtime_dir = resolve_sidecar_runtime_dir(app)?;
    let path = sidecar_runtime_path_env(&runtime_dir)?;
    app.shell()
        .sidecar("r26-cli")
        .map(|command| command.env("PATH", path))
        .map_err(|e| format!("无法定位刷机组件: {e}"))
}

/// Verify the vendored sidecar reports the expected version.
async fn ensure_sidecar_version(app: &AppHandle) -> Result<(), String> {
    if let Some(cached) = SIDECAR_VERSION_CHECK.get() {
        return cached.clone();
    }
    let result = async {
        let output = sidecar_command(app)?
            .args(["--version"])
            .output()
            .await
            .map_err(|e| format!("启动刷机组件失败: {e}"))?;
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if version.ends_with(SIDECAR_EXPECTED_VERSION) {
            Ok(())
        } else {
            Err(format!(
                "刷机组件版本不匹配：期望 {SIDECAR_EXPECTED_VERSION}，实际 \"{version}\"。\
                 请运行 scripts/update-dloader-cli.ps1 重新同步"
            ))
        }
    }
    .await;
    let _ = SIDECAR_VERSION_CHECK.set(result.clone());
    result
}

// -- DTOs mirroring r26-core's serialized safety types -----------------------

/// Risk level of one PAC file entry.
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Safe,
    NvWrite,
    RfCalibration,
    Erase,
    EraseAll,
    PhaseCheck,
    Unknown(String),
}

impl RiskLevel {
    fn as_str(&self) -> &str {
        match self {
            Self::Safe => "Safe",
            Self::NvWrite => "NvWrite",
            Self::RfCalibration => "RfCalibration",
            Self::Erase => "Erase",
            Self::EraseAll => "EraseAll",
            Self::PhaseCheck => "PhaseCheck",
            Self::Unknown(s) => s,
        }
    }
}

impl Serialize for RiskLevel {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RiskLevel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(match s.as_str() {
            "Safe" => Self::Safe,
            "NvWrite" => Self::NvWrite,
            "RfCalibration" => Self::RfCalibration,
            "Erase" => Self::Erase,
            "EraseAll" => Self::EraseAll,
            "PhaseCheck" => Self::PhaseCheck,
            _ => Self::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRiskDto {
    pub file_id: String,
    pub file_type: String,
    pub risk_level: RiskLevel,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyReportDto {
    pub total_files: usize,
    pub safe_files: Vec<String>,
    pub nv_files: Vec<FileRiskDto>,
    pub rf_cali_files: Vec<FileRiskDto>,
    pub erase_files: Vec<FileRiskDto>,
    pub phasecheck_files: Vec<FileRiskDto>,
    pub has_risks: bool,
    pub touches_rf_calibration: bool,
    pub summary: String,
}

/// The decision for how to flash a given PAC.
#[derive(Debug, Clone, PartialEq)]
pub enum FlashDecision {
    Blocked { reason: String },
    Proceed { allow_flags: Vec<&'static str> },
}

/// Factory-safe flashing policy.
pub fn plan_flash(report: &SafetyReportDto) -> FlashDecision {
    if report.touches_rf_calibration || !report.rf_cali_files.is_empty() {
        return FlashDecision::Blocked {
            reason: "此 PAC 含射频校准分区，出于保护已禁止刷写".to_string(),
        };
    }
    if !report.phasecheck_files.is_empty() {
        return FlashDecision::Blocked {
            reason: "此 PAC 含 PhaseCheck 生产数据分区，已禁止刷写".to_string(),
        };
    }
    let mut allow_flags = Vec::new();
    if !report.erase_files.is_empty() {
        allow_flags.push("--allow-erase");
    }
    if !report.nv_files.is_empty() {
        allow_flags.push("--allow-nv-write");
    }
    FlashDecision::Proceed { allow_flags }
}

// -- Cross-process event contract (typed) ------------------------------------

/// Local mirror of r26-cli's `EngineEvent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FirmwareEvent {
    Log {
        level: String,
        message: String,
    },
    Progress {
        port: u32,
        percent: f32,
        file_id: String,
    },
    StateChange {
        from: String,
        to: String,
    },
    Error {
        code: u32,
        message: String,
    },
    Completed {
        port: u32,
        result: DownloadResultDto,
    },
    SafetyReportReady {
        report: serde_json::Value,
    },
    SafetyViolation {
        violation: serde_json::Value,
    },
    SafetyConfirmed {
        category: String,
    },
    PacLoadProgress {
        percent: u32,
    },
    Terminated {
        code: Option<i32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResultDto {
    pub port: u32,
    pub success: bool,
    pub chip_uid: Option<String>,
    pub flash_uid: Option<String>,
    pub duration_ms: u64,
    pub error: Option<String>,
}

/// Run `r26-cli pac-info <path> --json` and parse the safety report.
async fn run_pac_info(app: &AppHandle, path: &str) -> Result<SafetyReportDto, String> {
    let output = sidecar_command(app)?
        .args(["pac-info", path, "--json"])
        .output()
        .await
        .map_err(|e| format!("启动刷机组件失败: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "PAC 分析失败: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout
        .lines()
        .rev()
        .find(|l| l.trim_start().starts_with('{'))
        .ok_or("刷机组件未返回安全报告")?;
    serde_json::from_str::<SafetyReportDto>(line.trim())
        .map_err(|e| format!("解析安全报告失败: {e}"))
}

// -- IPC commands ------------------------------------------------------------

/// Open a native file picker for a `.pac` file.
#[tauri::command]
pub async fn pick_pac_file(app: AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog()
        .file()
        .add_filter("PAC 固件包", &["pac"])
        .pick_file(move |f| {
            let _ = tx.send(f);
        });
    let picked = rx.await.map_err(|e| e.to_string())?;
    Ok(picked.map(|p| p.to_string()))
}

/// Analyze a PAC file and return its safety report.
#[tauri::command]
pub async fn pac_info(app: AppHandle, path: String) -> Result<SafetyReportDto, String> {
    ensure_sidecar_version(&app).await?;
    run_pac_info(&app, &path).await
}

/// Holds the currently-running download sidecar so `stop` can kill it.
#[derive(Default)]
pub struct DloaderState {
    pub child: Arc<Mutex<Option<CommandChild>>>,
}

fn lock_child_slot<'a>(
    child: &'a Arc<Mutex<Option<CommandChild>>>,
) -> Result<std::sync::MutexGuard<'a, Option<CommandChild>>, String> {
    child
        .lock()
        .map_err(|e| format!("dloader.child lock poisoned: {e}"))
}

/// Start the firmware download: analyze PAC, enforce policy, spawn sidecar.
#[tauri::command]
pub async fn start_firmware_download(app: AppHandle, path: String) -> Result<(), String> {
    ensure_sidecar_version(&app).await?;

    // Re-analyze + policy gate (TOCTOU protection).
    let report = run_pac_info(&app, &path).await?;
    let allow_flags = match plan_flash(&report) {
        FlashDecision::Blocked { reason } => return Err(reason),
        FlashDecision::Proceed { allow_flags } => allow_flags,
    };

    let mut args: Vec<String> = vec![
        "download".into(),
        path,
        "--port".into(),
        "0".into(),
        "--json".into(),
    ];
    for f in &allow_flags {
        args.push((*f).to_string());
    }

    let state = app.state::<DloaderState>();
    let child_slot = state.child.clone();
    let mut rx = {
        let mut guard = lock_child_slot(&state.child)?;
        if guard.is_some() {
            return Err("下载正在进行中".to_string());
        }
        let (rx, child) = sidecar_command(&app)?
            .args(args)
            .spawn()
            .map_err(|e| format!("启动刷机失败: {e}"))?;
        *guard = Some(child);
        rx
    };

    // Forward stdout lines as typed `firmware-event`s.
    let app_for_task = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let fw_event = match serde_json::from_str::<FirmwareEvent>(trimmed) {
                        Ok(ev) => ev,
                        Err(e) if trimmed.starts_with('{') => FirmwareEvent::Log {
                            level: "warn".to_string(),
                            message: format!("无法识别的事件（契约漂移？{e}）: {trimmed}"),
                        },
                        Err(_) => FirmwareEvent::Log {
                            level: "info".to_string(),
                            message: trimmed.to_string(),
                        },
                    };
                    let _ = app_for_task.emit("firmware-event", fw_event);
                }
                CommandEvent::Stderr(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        let _ = app_for_task.emit(
                            "firmware-event",
                            FirmwareEvent::Log {
                                level: "stderr".to_string(),
                                message: trimmed.to_string(),
                            },
                        );
                    }
                }
                CommandEvent::Error(err) => {
                    let _ = app_for_task.emit(
                        "firmware-event",
                        FirmwareEvent::Error {
                            code: 0,
                            message: err,
                        },
                    );
                }
                CommandEvent::Terminated(payload) => {
                    match lock_child_slot(&child_slot) {
                        Ok(mut guard) => *guard = None,
                        Err(e) => log::warn!("Failed to clear dloader child slot: {}", e),
                    }
                    let _ = app_for_task.emit(
                        "firmware-event",
                        FirmwareEvent::Terminated { code: payload.code },
                    );
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(())
}

/// Kill the running download sidecar, if any.
#[tauri::command]
pub fn stop_firmware_download(state: State<'_, DloaderState>) -> Result<(), String> {
    if let Some(child) = lock_child_slot(&state.child)?.take() {
        child.kill().map_err(|e| format!("停止失败: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let suffix = COUNTER.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("modem-cat-{prefix}-{timestamp}-{suffix}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn touch(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, b"stub").expect("write stub file");
    }

    fn risk(id: &str) -> FileRiskDto {
        FileRiskDto {
            file_id: id.into(),
            file_type: "".into(),
            risk_level: RiskLevel::Safe,
            reason: "".into(),
        }
    }
    fn report() -> SafetyReportDto {
        SafetyReportDto {
            total_files: 0,
            safe_files: vec![],
            nv_files: vec![],
            rf_cali_files: vec![],
            erase_files: vec![],
            phasecheck_files: vec![],
            has_risks: false,
            touches_rf_calibration: false,
            summary: String::new(),
        }
    }

    #[test]
    fn safe_pac_proceeds_with_no_flags() {
        assert_eq!(
            plan_flash(&report()),
            FlashDecision::Proceed {
                allow_flags: vec![]
            }
        );
    }

    #[test]
    fn erase_and_nv_auto_allowed() {
        let mut r = report();
        r.erase_files = vec![risk("EraseFlash")];
        r.nv_files = vec![risk("NV_NORFLASH")];
        assert_eq!(
            plan_flash(&r),
            FlashDecision::Proceed {
                allow_flags: vec!["--allow-erase", "--allow-nv-write"]
            }
        );
    }

    #[test]
    fn rf_calibration_is_blocked() {
        let mut r = report();
        r.touches_rf_calibration = true;
        r.rf_cali_files = vec![risk("LTE_CALI")];
        assert!(matches!(plan_flash(&r), FlashDecision::Blocked { .. }));
    }

    #[test]
    fn phasecheck_is_blocked() {
        let mut r = report();
        r.phasecheck_files = vec![risk("PhaseCheck")];
        assert!(matches!(plan_flash(&r), FlashDecision::Blocked { .. }));
    }

    #[test]
    fn unknown_risk_level_round_trips() {
        let lvl: RiskLevel = serde_json::from_str("\"FutureRisk\"").unwrap();
        assert_eq!(lvl, RiskLevel::Unknown("FutureRisk".into()));
        assert_eq!(serde_json::to_string(&lvl).unwrap(), "\"FutureRisk\"");
        let known: RiskLevel = serde_json::from_str("\"NvWrite\"").unwrap();
        assert_eq!(known, RiskLevel::NvWrite);
    }

    #[test]
    fn sidecar_runtime_prefers_exe_dir_when_dll_is_already_next_to_sidecar() {
        let temp_root = unique_temp_dir("dloader-exe-runtime");
        let exe_dir = temp_root.join("app");
        let resource_dir = temp_root.join("resources");
        touch(&exe_dir.join("vcruntime140.dll"));
        touch(&resource_dir.join("r26-runtime").join("vcruntime140.dll"));

        let resolved = resolve_sidecar_runtime_dir_from_paths(Some(&resource_dir), Some(&exe_dir))
            .expect("resolve runtime dir");

        assert_eq!(resolved, exe_dir);
        fs::remove_dir_all(temp_root).expect("cleanup temp dir");
    }

    #[test]
    fn sidecar_runtime_falls_back_to_resource_dir_when_exe_dir_is_missing_dll() {
        let temp_root = unique_temp_dir("dloader-resource-runtime");
        let exe_dir = temp_root.join("app");
        let resource_dir = temp_root.join("resources");
        touch(&resource_dir.join("r26-runtime").join("vcruntime140.dll"));

        let resolved = resolve_sidecar_runtime_dir_from_paths(Some(&resource_dir), Some(&exe_dir))
            .expect("resolve runtime dir");

        assert_eq!(resolved, resource_dir.join("r26-runtime"));
        fs::remove_dir_all(temp_root).expect("cleanup temp dir");
    }

    #[test]
    fn sidecar_runtime_errors_when_no_packaged_runtime_is_available() {
        let temp_root = unique_temp_dir("dloader-missing-runtime");
        let exe_dir = temp_root.join("app");
        let resource_dir = temp_root.join("resources");

        let error = resolve_sidecar_runtime_dir_from_paths_with_repo_fallback(
            Some(&resource_dir),
            Some(&exe_dir),
            false,
        )
        .expect_err("runtime should be missing");

        assert!(error.contains("vcruntime140.dll"));
        fs::remove_dir_all(temp_root).expect("cleanup temp dir");
    }

    #[test]
    fn firmware_event_parses_engine_event_json() {
        let line = r#"{"Progress":{"port":3,"percent":52.5,"file_id":"FDL2"}}"#;
        match serde_json::from_str::<FirmwareEvent>(line).unwrap() {
            FirmwareEvent::Progress {
                port,
                percent,
                file_id,
            } => {
                assert_eq!(port, 3);
                assert_eq!(file_id, "FDL2");
                assert!((percent - 52.5).abs() < f32::EPSILON);
            }
            other => panic!("wrong variant: {other:?}"),
        }
        let done = r#"{"Completed":{"port":3,"result":{"port":3,"success":true,"chip_uid":null,"flash_uid":null,"duration_ms":61000,"error":null}}}"#;
        assert!(matches!(
            serde_json::from_str::<FirmwareEvent>(done).unwrap(),
            FirmwareEvent::Completed { .. }
        ));
        let drifted = r#"{"Progress":{"port":3,"percent":52.5,"file_name":"FDL2"}}"#;
        assert!(serde_json::from_str::<FirmwareEvent>(drifted).is_err());
    }
}
