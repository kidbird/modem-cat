# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

modem-cat is a 5G modem debugging desktop tool. Tauri 2.x app with Rust backend (`src-tauri/`) and single-file web frontend (`src/desktop/index.html`). The core modem logic lives in `modem-hal/`, a standalone Rust crate consumed by the Tauri backend.

## Commands

```bash
# Desktop app (Tauri)
cd src-tauri && cargo build --release

# Full workspace (both crates)
cargo build --workspace
cargo test --workspace

# Run tests for a single crate
cargo test -p modem-hal
cargo test -p modem-cat

# Run a specific test
cargo test -p modem-hal parse_qeng_serving_cell
```

## Windows Build Note

**The Tauri app must be fully quit before rebuilding.** The running `.exe` holds a file lock that blocks `cargo build`. The app minimizes to system tray on close — right-click the tray icon and choose "退出" to truly exit.

## Architecture

```
src/desktop/index.html          ← Single-file frontend (HTML/CSS/JS, no framework)
        │
     Tauri IPC (invoke)
        │
src-tauri/src/lib.rs            ← All Tauri commands, AppState, tray, window management
        │
modem-hal/src/                  ← Shared HAL crate (transport, vendor detection, AT parsing)
        │
   serialport / TCP → 5G Modem
```

### modem-hal (`modem-hal/`)
Standalone Rust HAL crate. Vendor-agnostic interface over serial AT commands.
- `src/modem_vendor.rs` — `ModemVendor` trait (28+ methods covering SIM, network, bands, data, diagnostics)
- `src/modem_factory.rs` — `ModemFactory::create()` detects vendor from `AT+CGMM`
- `src/types.rs` — shared data types (ModemStatus, SignalInfo, BandConfig, etc.)
- `src/transport/mod.rs` — `AtTransport` trait (`send_at`, `close`) + `MockTransport`
- `src/transport/serial.rs` — `SerialTransport` (serialport v4, 115200 baud)
- `src/transport/tcp.rs` — `TcpTransport`
- `src/vendors/quectel/` — Quectel modems: `mod.rs` (adapter), `parser.rs` (response parsing), `band_db.rs` (hardware band tables), `qualcomm.rs`/`unisoc.rs` (chip-specific data commands)
- `src/vendors/tdtech/` — TdTech MT5700M-CN: `mod.rs` (adapter), `parser.rs`, `dial.rs` (AT^ prefix commands)

Feature flags:
- `serial` (default) — enables `SerialTransport`
- `napi-feature` — compiles napi-rs `ModemHandle` class for Bun/TS native addon

### Tauri Backend (`src-tauri/`)
All backend logic is in `src/lib.rs` (~1000 lines). Delegates modem operations directly to `modem-hal` via `ModemVendor` trait.
- `AppState` holds `Arc<Mutex<Option<Box<dyn AtTransport>>>>` and `Arc<Mutex<Option<Box<dyn ModemVendor>>>>`
- All query/write commands follow the same pattern: `tokio::task::spawn_blocking` → lock transport + vendor → call vendor method
- `start_port_monitor` polls serial ports every 2s, emits `port-changed` Tauri events

### Frontend (`src/desktop/`)
Single `index.html` (~98KB), all HTML/CSS/JS inline, no build step. Uses `window.__TAURI__.invoke()` for IPC.

## Vendor Detection

`ModemFactory::detect_vendor_from_model()` matches model string from `AT+CGMM`:
- `MT5700` → TdTech (`AT^` commands)
- `RG200U` / `RM500U` → Quectel UniSoc (`AT+QNETDEVCTL` data commands)
- `RM520N` / `RM500Q` → Quectel Qualcomm (`AT+QMAP` data commands)
- Unknown models default to UniSoc adapter

## Key Conventions

- AT parser functions are pure: take `&str` input, return parsed structs — no I/O, easy to unit test
- Adding a new vendor: implement `ModemVendor` trait, add detection in `modem_factory.rs`, add module under `vendors/`
- Frontend has no build tooling — edit `index.html` directly
- All Rust tests are pure unit tests (no I/O, no hardware required)

## Tauri v2 Specifics

- Uses `tauri::Manager` trait for `webview_windows()`, `tray_by_id()`
- System tray configured in `tauri.conf.json`; menu built programmatically with `tauri::menu::MenuBuilder`
- Window close intercepted via `.on_window_event()` to hide to tray instead of exiting
- Custom `modemcat://` URI scheme protocol serves the embedded `index.html`
- `winreg` crate for Windows registry port friendly name lookup (`cfg(target_os = "windows")`)
