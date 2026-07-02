use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const PREFS_FILENAME: &str = "debug_terminal_prefs.json";
const DEBUG_OUTPUT_EVENT: &str = "debug-terminal-output";

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugTerminalPrefs {
    pub ssh_username: Option<String>,
    pub ssh_last_adapter: Option<String>,
    pub ssh_last_ip: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct DebugTerminalCapabilities {
    pub adb_supported: bool,
    pub ssh_supported: bool,
}

impl DebugTerminalCapabilities {
    pub fn for_current_platform() -> Self {
        Self {
            adb_supported: cfg!(target_os = "windows"),
            ssh_supported: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugTerminalOutputEvent {
    pub kind: String,
    pub stream: String,
    pub text: String,
}

pub type DebugNetworkAdapter = crate::connection::NetworkAdapter;

enum SessionCommand {
    Input(String),
    Close,
}

struct SessionHandle {
    id: u64,
    tx: Sender<SessionCommand>,
}

#[derive(Default)]
pub struct DebugTerminalState {
    next_session_id: AtomicU64,
    active_session: std::sync::Arc<Mutex<Option<SessionHandle>>>,
}

impl DebugTerminalState {
    fn alloc_session_id(&self) -> u64 {
        self.next_session_id.fetch_add(1, Ordering::Relaxed) + 1
    }
}

fn trim_optional(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

pub fn sanitize_prefs(prefs: DebugTerminalPrefs) -> DebugTerminalPrefs {
    DebugTerminalPrefs {
        ssh_username: trim_optional(prefs.ssh_username),
        ssh_last_adapter: trim_optional(prefs.ssh_last_adapter),
        ssh_last_ip: trim_optional(prefs.ssh_last_ip),
    }
}

fn lock_state<'a, T>(mutex: &'a Mutex<T>, label: &str) -> Result<MutexGuard<'a, T>, String> {
    mutex
        .lock()
        .map_err(|e| format!("{label} lock poisoned: {e}"))
}

fn emit_output(app: &AppHandle, kind: &'static str, stream: &str, text: impl Into<String>) {
    let text = text.into();
    if text.is_empty() {
        return;
    }
    let payload = DebugTerminalOutputEvent {
        kind: kind.to_string(),
        stream: stream.to_string(),
        text,
    };
    let _ = app.emit(DEBUG_OUTPUT_EVENT, payload);
}

fn clear_active_session(
    active_session: &std::sync::Arc<Mutex<Option<SessionHandle>>>,
    session_id: u64,
) {
    if let Ok(mut guard) = active_session.lock() {
        let should_clear = guard
            .as_ref()
            .map(|session| session.id == session_id)
            .unwrap_or(false);
        if should_clear {
            guard.take();
        }
    }
}

fn take_active_session(state: &DebugTerminalState) -> Result<Option<SessionHandle>, String> {
    Ok(lock_state(&state.active_session, "debug_terminal.active_session")?.take())
}

fn set_active_session(state: &DebugTerminalState, handle: SessionHandle) -> Result<(), String> {
    let mut guard = lock_state(&state.active_session, "debug_terminal.active_session")?;
    if guard.is_some() {
        return Err("已有调试会话正在运行，请先断开当前会话".to_string());
    }
    *guard = Some(handle);
    Ok(())
}

fn prefs_path(app: &AppHandle) -> Result<PathBuf, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {e}"))?;
    fs::create_dir_all(&data_dir).map_err(|e| format!("创建数据目录失败: {e}"))?;
    Ok(data_dir.join(PREFS_FILENAME))
}

fn load_prefs_from_disk(app: &AppHandle) -> Result<DebugTerminalPrefs, String> {
    let path = prefs_path(app)?;
    if !path.exists() {
        return Ok(DebugTerminalPrefs::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("读取调试配置失败: {e}"))?;
    let prefs = serde_json::from_str::<DebugTerminalPrefs>(&content)
        .map_err(|e| format!("解析调试配置失败: {e}"))?;
    Ok(sanitize_prefs(prefs))
}

fn save_prefs_to_disk(app: &AppHandle, prefs: &DebugTerminalPrefs) -> Result<(), String> {
    let path = prefs_path(app)?;
    let content =
        serde_json::to_string_pretty(prefs).map_err(|e| format!("序列化调试配置失败: {e}"))?;
    fs::write(path, content).map_err(|e| format!("写入调试配置失败: {e}"))
}

fn resolve_bundled_adb_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut candidates = Vec::new();

    if let Ok(resource_dir) = app.path().resource_dir() {
        candidates.push(resource_dir.join("adb").join("adb.exe"));
    }

    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            candidates.push(dir.join("adb").join("adb.exe"));
            candidates.push(dir.join("adb.exe"));
        }
    }

    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("adb")
            .join("adb.exe"),
    );

    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| {
            "未找到随包 ADB 组件，请先将 adb.exe 与依赖库放入 src-tauri/resources/adb/".to_string()
        })
}

fn spawn_reader_thread<R: Read + Send + 'static>(
    app: AppHandle,
    active_session: std::sync::Arc<Mutex<Option<SessionHandle>>>,
    session_id: u64,
    kind: &'static str,
    stream: &'static str,
    mut reader: R,
) {
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    emit_output(
                        &app,
                        kind,
                        stream,
                        String::from_utf8_lossy(&buf[..n]).to_string(),
                    );
                }
                Err(err) => {
                    emit_output(&app, kind, "system", format!("{stream} 读取失败: {err}"));
                    break;
                }
            }
        }
        clear_active_session(&active_session, session_id);
    });
}

fn spawn_adb_thread(
    app: AppHandle,
    active_session: std::sync::Arc<Mutex<Option<SessionHandle>>>,
    session_id: u64,
    adb_path: PathBuf,
    rx: std::sync::mpsc::Receiver<SessionCommand>,
) {
    thread::spawn(move || {
        let kind = "adb";
        let run_result = (|| -> Result<(), String> {
            let adb_dir = adb_path
                .parent()
                .ok_or("ADB 路径无效，缺少父目录")?
                .to_path_buf();
            let mut command = Command::new(&adb_path);
            command
                .arg("shell")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .current_dir(adb_dir);
            #[cfg(windows)]
            command.creation_flags(CREATE_NO_WINDOW);

            let mut child = command
                .spawn()
                .map_err(|e| format!("启动 adb shell 失败: {e}"))?;
            let mut stdin = child.stdin.take().ok_or("无法打开 ADB stdin")?;
            let stdout = child.stdout.take().ok_or("无法打开 ADB stdout")?;
            let stderr = child.stderr.take().ok_or("无法打开 ADB stderr")?;

            emit_output(&app, kind, "system", "ADB shell 已启动");
            spawn_reader_thread(
                app.clone(),
                active_session.clone(),
                session_id,
                kind,
                "stdout",
                stdout,
            );
            spawn_reader_thread(
                app.clone(),
                active_session.clone(),
                session_id,
                kind,
                "stderr",
                stderr,
            );

            loop {
                match rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(SessionCommand::Input(input)) => {
                        stdin
                            .write_all(input.as_bytes())
                            .and_then(|_| stdin.flush())
                            .map_err(|e| format!("写入 ADB shell 失败: {e}"))?;
                    }
                    Ok(SessionCommand::Close) => {
                        let _ = child.kill();
                        break;
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        if let Some(status) = child
                            .try_wait()
                            .map_err(|e| format!("轮询 ADB 状态失败: {e}"))?
                        {
                            let code = status
                                .code()
                                .map(|v| v.to_string())
                                .unwrap_or_else(|| "unknown".to_string());
                            emit_output(
                                &app,
                                kind,
                                "system",
                                format!("ADB shell 已退出 (code {code})"),
                            );
                            break;
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        let _ = child.kill();
                        break;
                    }
                }
            }
            Ok(())
        })();

        if let Err(err) = run_result {
            emit_output(&app, kind, "system", err);
        }
        clear_active_session(&active_session, session_id);
    });
}

fn is_ssh_would_block(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::WouldBlock
}

fn write_all_ssh(channel: &mut ssh2::Channel, input: &[u8]) -> Result<(), String> {
    let mut offset = 0usize;
    while offset < input.len() {
        match channel.write(&input[offset..]) {
            Ok(0) => thread::sleep(Duration::from_millis(10)),
            Ok(written) => offset += written,
            Err(err) if is_ssh_would_block(&err) => thread::sleep(Duration::from_millis(10)),
            Err(err) => return Err(format!("写入 SSH shell 失败: {err}")),
        }
    }
    channel
        .flush()
        .map_err(|e| format!("刷新 SSH shell 失败: {e}"))
}

fn spawn_ssh_thread(
    app: AppHandle,
    active_session: std::sync::Arc<Mutex<Option<SessionHandle>>>,
    session_id: u64,
    host: String,
    username: String,
    password: String,
    rx: std::sync::mpsc::Receiver<SessionCommand>,
) {
    thread::spawn(move || {
        let kind = "ssh";
        let run_result = (|| -> Result<(), String> {
            let tcp = TcpStream::connect((host.as_str(), 22))
                .map_err(|e| format!("连接 SSH 设备失败: {e}"))?;
            tcp.set_nodelay(true)
                .map_err(|e| format!("设置 SSH 套接字失败: {e}"))?;

            let mut session =
                ssh2::Session::new().map_err(|e| format!("创建 SSH 会话失败: {e}"))?;
            session.set_tcp_stream(tcp);
            session
                .handshake()
                .map_err(|e| format!("SSH 握手失败: {e}"))?;
            session
                .userauth_password(&username, &password)
                .map_err(|e| format!("SSH 用户名/密码认证失败: {e}"))?;
            if !session.authenticated() {
                return Err("SSH 认证未通过".to_string());
            }
            session.set_blocking(false);

            let mut channel = session
                .channel_session()
                .map_err(|e| format!("创建 SSH channel 失败: {e}"))?;
            channel
                .handle_extended_data(ssh2::ExtendedData::Merge)
                .map_err(|e| format!("配置 SSH stderr 合并失败: {e}"))?;
            channel
                .request_pty("xterm", None, Some((120, 36, 0, 0)))
                .map_err(|e| format!("申请 SSH PTY 失败: {e}"))?;
            channel
                .shell()
                .map_err(|e| format!("启动 SSH shell 失败: {e}"))?;

            emit_output(&app, kind, "system", "SSH shell 已连接");

            let mut stdout_buf = [0u8; 4096];
            loop {
                loop {
                    match rx.try_recv() {
                        Ok(SessionCommand::Input(input)) => {
                            write_all_ssh(&mut channel, input.as_bytes())?
                        }
                        Ok(SessionCommand::Close) => {
                            let _ = channel.close();
                            let _ = channel.wait_close();
                            emit_output(&app, kind, "system", "SSH shell 已断开");
                            return Ok(());
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => break,
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                            let _ = channel.close();
                            return Ok(());
                        }
                    }
                }

                match channel.read(&mut stdout_buf) {
                    Ok(0) => {
                        if channel.eof() {
                            let _ = channel.wait_close();
                            emit_output(&app, kind, "system", "SSH shell 已退出");
                            return Ok(());
                        }
                    }
                    Ok(n) => {
                        emit_output(
                            &app,
                            kind,
                            "stdout",
                            String::from_utf8_lossy(&stdout_buf[..n]).to_string(),
                        );
                    }
                    Err(err) if is_ssh_would_block(&err) => {}
                    Err(err) => return Err(format!("读取 SSH shell 输出失败: {err}")),
                }

                thread::sleep(Duration::from_millis(15));
            }
        })();

        if let Err(err) = run_result {
            emit_output(&app, kind, "system", err);
        }
        clear_active_session(&active_session, session_id);
    });
}

#[tauri::command]
pub fn get_debug_terminal_capabilities() -> DebugTerminalCapabilities {
    DebugTerminalCapabilities::for_current_platform()
}

#[tauri::command]
pub async fn list_debug_network_adapters() -> Result<Vec<DebugNetworkAdapter>, String> {
    crate::connection::list_network_adapters().await
}

#[tauri::command]
pub fn get_debug_terminal_prefs(app: AppHandle) -> Result<DebugTerminalPrefs, String> {
    load_prefs_from_disk(&app)
}

#[tauri::command]
pub fn save_debug_terminal_prefs(
    app: AppHandle,
    prefs: DebugTerminalPrefs,
) -> Result<DebugTerminalPrefs, String> {
    let prefs = sanitize_prefs(prefs);
    save_prefs_to_disk(&app, &prefs)?;
    Ok(prefs)
}

#[tauri::command]
pub fn start_adb_session(
    app: AppHandle,
    state: State<'_, DebugTerminalState>,
) -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
        let _ = state;
        return Err("ADB 调试仅支持 Windows".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let adb_path = resolve_bundled_adb_path(&app)?;
        let (tx, rx) = mpsc::channel();
        let session_id = state.alloc_session_id();
        let active_session = state.active_session.clone();
        set_active_session(&state, SessionHandle { id: session_id, tx })?;
        spawn_adb_thread(app, active_session, session_id, adb_path, rx);
        Ok(())
    }
}

#[tauri::command]
pub fn start_ssh_session(
    app: AppHandle,
    state: State<'_, DebugTerminalState>,
    host: String,
    username: String,
    password: String,
) -> Result<(), String> {
    let host = host.trim().to_string();
    let username = username.trim().to_string();
    if host.is_empty() {
        return Err("设备 IP 不能为空".to_string());
    }
    if username.is_empty() {
        return Err("SSH 用户名不能为空".to_string());
    }
    if password.is_empty() {
        return Err("SSH 密码不能为空".to_string());
    }

    let (tx, rx) = mpsc::channel();
    let session_id = state.alloc_session_id();
    let active_session = state.active_session.clone();
    set_active_session(&state, SessionHandle { id: session_id, tx })?;
    spawn_ssh_thread(
        app,
        active_session,
        session_id,
        host,
        username,
        password,
        rx,
    );
    Ok(())
}

#[tauri::command]
pub fn write_debug_terminal_input(
    state: State<'_, DebugTerminalState>,
    input: String,
) -> Result<(), String> {
    let guard = lock_state(&state.active_session, "debug_terminal.active_session")?;
    let session = guard.as_ref().ok_or("当前没有活动的调试会话")?;
    session
        .tx
        .send(SessionCommand::Input(input))
        .map_err(|e| format!("写入调试会话失败: {e}"))
}

#[tauri::command]
pub fn close_debug_terminal_session(state: State<'_, DebugTerminalState>) -> Result<(), String> {
    if let Some(session) = take_active_session(&state)? {
        session
            .tx
            .send(SessionCommand::Close)
            .map_err(|e| format!("关闭调试会话失败: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adb_capability_is_windows_only() {
        let caps = DebugTerminalCapabilities::for_current_platform();
        #[cfg(target_os = "windows")]
        assert!(caps.adb_supported);
        #[cfg(not(target_os = "windows"))]
        assert!(!caps.adb_supported);
    }

    #[test]
    fn prefs_clear_password_and_preserve_username() {
        let prefs = DebugTerminalPrefs {
            ssh_username: Some("root".into()),
            ssh_last_adapter: Some("Ethernet".into()),
            ssh_last_ip: Some("192.168.1.1".into()),
        };
        assert_eq!(prefs.ssh_username.as_deref(), Some("root"));
        assert_eq!(prefs.ssh_last_ip.as_deref(), Some("192.168.1.1"));
    }

    #[test]
    fn save_prefs_trims_empty_values() {
        let prefs = sanitize_prefs(DebugTerminalPrefs {
            ssh_username: Some("  ".into()),
            ssh_last_adapter: Some(" Ethernet ".into()),
            ssh_last_ip: Some(" 192.168.42.1 ".into()),
        });

        assert_eq!(prefs.ssh_username, None);
        assert_eq!(prefs.ssh_last_adapter.as_deref(), Some("Ethernet"));
        assert_eq!(prefs.ssh_last_ip.as_deref(), Some("192.168.42.1"));
    }

    #[test]
    fn only_one_active_session_is_allowed() {
        let state = DebugTerminalState::default();
        let (tx, _rx) = mpsc::channel();
        set_active_session(&state, SessionHandle { id: 1, tx }).expect("first session");

        let (tx2, _rx2) = mpsc::channel();
        let err = set_active_session(&state, SessionHandle { id: 2, tx: tx2 })
            .expect_err("second session must be rejected");
        assert!(err.contains("已有调试会话"));
    }

    #[test]
    fn output_event_payload_has_stream_and_text() {
        let payload = DebugTerminalOutputEvent {
            kind: "ssh".into(),
            stream: "stdout".into(),
            text: "hello".into(),
        };
        assert_eq!(payload.stream, "stdout");
        assert_eq!(payload.text, "hello");
    }

    #[test]
    fn adapter_labels_require_ip_and_gateway() {
        let adapter = DebugNetworkAdapter {
            name: "Ethernet".into(),
            description: "Intel".into(),
            ip_address: Some("192.168.1.10".into()),
            gateway: Some("192.168.1.1".into()),
        };
        assert_eq!(adapter.gateway.as_deref(), Some("192.168.1.1"));
    }
}
