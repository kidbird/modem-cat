# AGENTS.md

## Build Commands

```bash
# Tauri Desktop (Rust + web frontend)
cd src-tauri && cargo build --release

# CLI (Bun/TypeScript)
bun run src/cli/index.ts        # Run directly
bun build src/cli/index.ts      # Build to dist/
bun test                        # Run tests
bun run typecheck               # TypeScript check
```

## Critical: Exe Lock on Rebuild

**The Tauri app must be fully quit before rebuilding.** On Windows, the running `.exe` holds a file lock that blocks `cargo build`. The app minimizes to system tray on close — right-click the tray icon and choose "退出" to truly exit.

## Architecture

### Desktop Frontend
- Single file: `src/desktop/index.html` (all HTML/CSS/JS inline)
- Served by Tauri from `frontendDist: "../src/desktop"` in tauri.conf.json

### Rust Backend
- `src-tauri/src/lib.rs` — main app logic, Tauri commands, system tray setup
- `src-tauri/src/at_adapter.rs` — AT command execution
- `src-tauri/src/serial.rs` — serial port handling
- `src-tauri/src/transport.rs` — AT transport layer

### CLI
- `src/cli/index.ts` — entry point
- `src/cli/commands/` — command modules

## Desktop App Behavior

- **Close button**: Hides window to system tray instead of exiting. Right-click tray icon for menu.
- **Tray menu**: "控制面板" (show window), "退出" (quit app)
- **Auto-connect**: On startup, automatically scans ports and connects to modem AT port

## Platform Notes

- Windows-only build scripts (`build.bat`, `build-tauri.bat`) use hardcoded VS/MSVC paths
- `winreg` crate for Windows registry access (port friendly name lookup)
- `serialport` crate v4 for serial communication

## Tauri v2 Specifics

- Uses `tauri::Manager` trait for `webview_windows()`, `tray_by_id()`, etc.
- System tray configured in `tauri.conf.json` under `app.trayIcon`
- Tray menu built programmatically with `tauri::menu::MenuBuilder`
- Window close intercepted via `.on_window_event()` to hide instead of close
