# AGENTS.md

This file provides guidance to CodeBuddy Code when working with code in this repository.

## Project Overview

modem-cat is a 5G modem debugging desktop tool. Tauri 2.x app with Rust backend and single-file web frontend.

## Build Commands

```bash
# Desktop app (Tauri)
cd src-tauri && cargo build --release

# Full workspace (both crates)
cargo build --workspace

# Run integration tests (requires physical modem connected)
cd src-tauri && cargo test

# Run all Rust tests
cargo test --workspace
```

## Critical: Exe Lock on Rebuild

**The Tauri app must be fully quit before rebuilding.** On Windows, the running `.exe` holds a file lock that blocks `cargo build`. The app minimizes to system tray on close — right-click the tray icon and choose "退出" to truly exit.

## Architecture

```
src/desktop/index.html          ← Single-file frontend (HTML/CSS/JS, no framework)
        │
     Tauri IPC (invoke)
        │
src-tauri/src/lib.rs            ← Tauri commands, AppState, tray, window management
        │
src-tauri/src/at_adapter.rs     ← AT command query/write business logic
        │
src-tauri/src/at_parser.rs      ← AT response parsing (Quectel-specific)
        │
modem-hal/src/                  ← Shared HAL crate (transport abstraction, vendor detection)
        │
   serialport / TCP → 5G Modem
```

### Frontend (`src/desktop/`)
- Single `index.html` (~98KB), all HTML/CSS/JS inline, no build step
- Uses `window.__TAURI__.invoke()` for IPC (withGlobalTauri enabled)
- UI pages: Status, Cellular, AT Debug, Hardware Info, IP Info

### Tauri Backend (`src-tauri/src/`)
- `lib.rs` — all `#[tauri::command]` handlers, `AppState` (transport, data_cid, connected_port), system tray
- `at_adapter.rs` — orchestrates AT queries, calls modem-hal transport + at_parser
- `at_parser.rs` — pure response parsing functions (Quectel AT command responses)

### modem-hal (`modem-hal/src/`)
Standalone Rust HAL crate. Vendor-agnostic interface over serial AT commands.
- `modem_vendor.rs` — `ModemVendor` trait (all modem operations, default `query_modem_status` implementation)
- `modem_factory.rs` — `ModemFactory::create()` detects vendor from AT+CGMM response
- `types.rs` — shared data types (ModemStatus, SignalInfo, BandConfig, etc.)
- `transport/mod.rs` — `AtTransport` trait
- `transport/serial.rs` — `SerialTransport` (serialport v4)
- `transport/tcp.rs` — `TcpTransport`
- `vendors/quectel/` — Quectel modems: `mod.rs` (main adapter), `parser.rs`, `band_db.rs`, `qualcomm.rs`, `unisoc.rs`
- `vendors/tdtech/` — TdTech MT5700M-CN: `mod.rs`, `parser.rs`, `dial.rs` (AT^ prefix commands)
- Feature flags: `serial` (default), `napi-feature` (napi-rs `ModemHandle` for Bun/TS native addon)

### Vendor Detection

`ModemFactory::create()` queries `AT+CGMM` and matches model string:
- `MT5700` → TdTech (`AT^` commands)
- `RG200U / RM500U / RG501U / RM501U` → Quectel UniSoc
- `RG520N / RM520N / RG525F / RG530F / RM530N / RG540F / RM540N` → Quectel Qualcomm

## Desktop App Behavior

- **Close button**: Hides window to system tray instead of exiting. Right-click tray icon for menu.
- **Tray menu**: "控制面板" (show window), "退出" (quit app)
- **Auto-connect**: On startup, automatically scans ports and connects to modem AT port

## Tauri IPC Commands (defined in lib.rs)

- **Connection**: `list_ports`, `auto_connect_at`, `connect_serial`, `connect_tcp`, `disconnect`
- **Status queries**: `get_modem_status`, `get_hardware_info`, `get_ip_info`, `get_traffic`, `get_apn_list`, `get_neighbor_cells`, `get_bands`, `get_feature_toggles`, `get_network_mode`
- **Configuration**: `set_apn_config`, `set_network_mode_cmd`, `set_bands`, `set_feature_toggle`, `connect_data`, `disconnect_data`
- **Diagnostics**: `send_raw_at`, `reboot_modem`, `factory_reset`

## Platform Notes

- Windows-only build scripts (`build.bat`, `build-tauri.bat`) use hardcoded VS/MSVC paths
- `winreg` crate for Windows registry access (port friendly name lookup), conditionally compiled via `cfg(target_os = "windows")`
- `serialport` crate v4 for serial communication
- Rust workspace: members are `modem-hal` and `src-tauri` (defined in root `Cargo.toml`)

## Tauri v2 Specifics

- Uses `tauri::Manager` trait for `webview_windows()`, `tray_by_id()`, etc.
- System tray configured in `tauri.conf.json` under `app.trayIcon`
- Tray menu built programmatically with `tauri::menu::MenuBuilder`
- Window close intercepted via `.on_window_event()` to hide instead of close

## Key Conventions

- AT parser functions are pure: take `&str` input, return parsed structs — no I/O
- at_adapter functions take `&mut dyn AtTransport` and compose multiple AT queries
- Adding a new vendor: implement `ModemVendor` trait, add detection in `modem_factory.rs`, add vendor module under `vendors/`
- Frontend has no build tooling — edit `index.html` directly
