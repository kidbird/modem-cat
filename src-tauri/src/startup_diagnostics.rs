use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

static PANIC_HOOK_ONCE: Once = Once::new();

pub fn startup_log_path_from_env(
    local_app_data: Option<OsString>,
    temp_dir: Option<OsString>,
    current_dir: Option<PathBuf>,
) -> PathBuf {
    let base = local_app_data
        .map(PathBuf::from)
        .or_else(|| temp_dir.map(PathBuf::from))
        .or(current_dir)
        .unwrap_or_else(std::env::temp_dir);

    base.join("Modem Cat").join("logs").join("startup.log")
}

pub fn startup_log_path() -> PathBuf {
    startup_log_path_from_env(
        std::env::var_os("LOCALAPPDATA"),
        std::env::var_os("TEMP"),
        std::env::current_dir().ok(),
    )
}

pub fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create startup log dir failed: {e}"))?;
    }
    Ok(())
}

fn timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "0".to_string(),
    }
}

fn format_optional_path(label: &str, path: Option<&Path>) -> String {
    match path {
        Some(path) => format!("{label}: {}", path.display()),
        None => format!("{label}: <unavailable>"),
    }
}

pub fn runtime_layout_snapshot_from_paths(
    exe_path: Option<PathBuf>,
    current_dir: Option<PathBuf>,
    local_app_data: Option<OsString>,
    temp_dir: Option<OsString>,
) -> Vec<String> {
    let startup_log = startup_log_path_from_env(local_app_data, temp_dir, current_dir.clone());
    let exe_dir = exe_path
        .as_ref()
        .and_then(|path| path.parent().map(Path::to_path_buf));
    let fixed_runtime_dir = exe_dir.as_ref().map(|dir| dir.join("webview2-runtime"));
    let fixed_runtime_exe = fixed_runtime_dir
        .as_ref()
        .map(|dir| dir.join("msedgewebview2.exe"));

    let mut lines = vec![
        format_optional_path("startup exe path", exe_path.as_deref()),
        format_optional_path("startup exe dir", exe_dir.as_deref()),
        format_optional_path("startup current dir", current_dir.as_deref()),
        format!("startup log path: {}", startup_log.display()),
    ];

    if let Some(runtime_dir) = fixed_runtime_dir.as_ref() {
        lines.push(format!("fixed runtime dir: {}", runtime_dir.display()));
        lines.push(format!(
            "fixed runtime dir exists: {}",
            runtime_dir.is_dir()
        ));
    } else {
        lines.push("fixed runtime dir: <unavailable>".to_string());
    }

    if let Some(runtime_exe) = fixed_runtime_exe.as_ref() {
        lines.push(format!("fixed runtime exe: {}", runtime_exe.display()));
        lines.push(format!(
            "fixed runtime exe exists: {}",
            runtime_exe.is_file()
        ));
    } else {
        lines.push("fixed runtime exe: <unavailable>".to_string());
    }

    lines
}

pub fn runtime_layout_snapshot() -> Vec<String> {
    runtime_layout_snapshot_from_paths(
        std::env::current_exe().ok(),
        std::env::current_dir().ok(),
        std::env::var_os("LOCALAPPDATA"),
        std::env::var_os("TEMP"),
    )
}

pub fn append_runtime_layout_snapshot() -> Result<PathBuf, String> {
    let mut last_path = startup_log_path();
    for line in runtime_layout_snapshot() {
        last_path = append_startup_log(&line)?;
    }
    Ok(last_path)
}

pub fn append_startup_log(message: &str) -> Result<PathBuf, String> {
    let path = startup_log_path();
    ensure_parent_dir(&path)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("open startup log failed: {e}"))?;
    use std::io::Write as _;
    writeln!(file, "[{}] {}", timestamp(), message)
        .map_err(|e| format!("write startup log failed: {e}"))?;
    Ok(path)
}

pub fn init_env_logger() -> Result<PathBuf, String> {
    let path = startup_log_path();
    ensure_parent_dir(&path)?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("open env_logger file failed: {e}"))?;

    let mut builder =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    builder
        .target(env_logger::Target::Pipe(Box::new(file)))
        .format_timestamp_secs();
    builder
        .try_init()
        .map_err(|e| format!("init env_logger failed: {e}"))?;
    Ok(path)
}

pub fn install_panic_hook() {
    PANIC_HOOK_ONCE.call_once(|| {
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = append_startup_log(&format!("panic: {panic_info}"));
            default_hook(panic_info);
        }));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_log_prefers_local_app_data() {
        let path = startup_log_path_from_env(
            Some(OsString::from(r"C:\Users\tester\AppData\Local")),
            Some(OsString::from(r"C:\Temp")),
            Some(PathBuf::from(r"C:\Workspace")),
        );

        assert_eq!(
            path,
            PathBuf::from(r"C:\Users\tester\AppData\Local\Modem Cat\logs\startup.log")
        );
    }

    #[test]
    fn startup_log_falls_back_to_temp() {
        let path = startup_log_path_from_env(
            None,
            Some(OsString::from(r"C:\Temp")),
            Some(PathBuf::from(r"C:\Workspace")),
        );

        assert_eq!(path, PathBuf::from(r"C:\Temp\Modem Cat\logs\startup.log"));
    }

    #[test]
    fn startup_log_falls_back_to_current_dir() {
        let path = startup_log_path_from_env(None, None, Some(PathBuf::from(r"C:\Workspace")));

        assert_eq!(
            path,
            PathBuf::from(r"C:\Workspace\Modem Cat\logs\startup.log")
        );
    }

    #[test]
    fn ensure_parent_dir_creates_missing_tree() {
        let temp_root =
            std::env::temp_dir().join(format!("mc-startup-diag-test-{}", std::process::id()));
        let log_path = temp_root.join("logs").join("startup.log");

        let _ = std::fs::remove_dir_all(&temp_root);
        ensure_parent_dir(&log_path).expect("parent dir should be created");

        assert!(log_path.parent().expect("parent").exists());

        let _ = std::fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn runtime_layout_snapshot_reports_fixed_runtime_presence_next_to_exe() {
        let temp_root =
            std::env::temp_dir().join(format!("mc-startup-layout-test-{}", std::process::id()));
        let exe_path = temp_root.join("modem-cat.exe");
        let runtime_dir = temp_root.join("webview2-runtime");
        let runtime_exe = runtime_dir.join("msedgewebview2.exe");

        let _ = std::fs::remove_dir_all(&temp_root);
        std::fs::create_dir_all(&runtime_dir).expect("runtime dir");
        std::fs::write(&exe_path, b"").expect("exe placeholder");
        std::fs::write(&runtime_exe, b"").expect("runtime exe placeholder");

        let lines = runtime_layout_snapshot_from_paths(
            Some(exe_path.clone()),
            Some(temp_root.clone()),
            Some(OsString::from(r"C:\Users\tester\AppData\Local")),
            Some(OsString::from(r"C:\Temp")),
        );

        assert!(lines
            .iter()
            .any(|line| line.contains(&format!("startup exe path: {}", exe_path.display()))));
        assert!(lines
            .iter()
            .any(|line| line.contains("fixed runtime dir exists: true")));
        assert!(lines
            .iter()
            .any(|line| line.contains("fixed runtime exe exists: true")));

        let _ = std::fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn runtime_layout_snapshot_reports_missing_runtime_when_exe_is_known() {
        let temp_root = std::env::temp_dir().join(format!(
            "mc-startup-layout-missing-test-{}",
            std::process::id()
        ));
        let exe_path = temp_root.join("modem-cat.exe");

        let _ = std::fs::remove_dir_all(&temp_root);
        std::fs::create_dir_all(&temp_root).expect("temp root");
        std::fs::write(&exe_path, b"").expect("exe placeholder");

        let lines = runtime_layout_snapshot_from_paths(
            Some(exe_path.clone()),
            Some(temp_root.clone()),
            None,
            Some(OsString::from(r"C:\Temp")),
        );

        assert!(lines
            .iter()
            .any(|line| line.contains("fixed runtime dir exists: false")));
        assert!(lines
            .iter()
            .any(|line| line.contains("fixed runtime exe exists: false")));

        let _ = std::fs::remove_dir_all(&temp_root);
    }
}
