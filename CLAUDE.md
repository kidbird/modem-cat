# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

modem-cat is a 5G modem debugging tool with two components:
- **Desktop**: Tauri app with Rust backend (`src-tauri/`) and web frontend (`src/desktop/index.html`)
- **modem-hal**: Standalone Rust HAL crate (`modem-hal/`) — consumed by the Tauri backend

## Commands

```bash
# Desktop app
cargo build --workspace            # Build all crates
cargo test --workspace             # Run all Rust tests
cd src-tauri && cargo build --release   # Release build for Tauri

# Development
cargo check --workspace            # Fast type-check without linking
```

## Architecture

### modem-hal (`modem-hal/`)
Standalone Rust HAL crate. Vendor-agnostic interface over serial AT commands.
- `src/modem_vendor.rs` — `ModemVendor` trait (all modem operations)
- `src/modem_factory.rs` — `ModemFactory::create()` detects vendor from AT+CGMM
- `src/types.rs` — shared data types (ModemStatus, SignalInfo, etc.)
- `src/transport/` — `AtTransport` trait + `SerialTransport` + `TcpTransport`
- `src/vendors/quectel/` — Qualcomm + UniSoc Quectel modems (RG520N, RG200U, …)
- `src/vendors/tdtech/` — TdTech MT5700M-CN (AT^ prefix commands)

Feature flags:
- `serial` (default) — enables `SerialTransport`

### Desktop App (`src-tauri/`)
Tauri 2.x Rust backend. Delegates modem logic to `modem-hal`.
- `src/lib.rs` — AppState, LoggingTransport, Tauri setup and all `#[tauri::command]` handlers
- `src/ports.rs` — port detection helpers + connection commands (list_ports, connect_serial, connect_tcp, disconnect)
- `src/commands.rs` — modem query/write commands
- `src/monitor.rs` — USB hotplug monitor thread

**AppState** holds two `Arc<Mutex<Option<Box<dyn _>>>>`:
- `transport` — the active `AtTransport` (serial or TCP), wrapped by `LoggingTransport`
- `vendor` — the detected `ModemVendor` (set by `ModemFactory::create()` on connect)

All Tauri commands acquire both mutexes in the same order (transport first, vendor second) to avoid deadlocks.

### Frontend (`src/desktop/index.html`)
Single-file HTML/CSS/JS frontend. Uses `@tauri-apps/api` for IPC.

## Tech Stack

- **modem-hal**: Rust, `serialport 4`, `serde`
- **Desktop backend**: Tauri 2.x, `tokio`, optional `winreg` (Windows registry for COM port names)
- **Desktop frontend**: Vanilla JS/HTML in a single file

## Vendor Detection

`ModemFactory::create()` queries `AT+CGMM` and matches model string:
- `MT5700` → TdTech (`AT^` commands)
- `RG200U / RM500U / RG501U / RM501U` → Quectel UniSoc
- `RG520N / RM520N / RG525F / RG530F / RM530N / RG540F / RM540N` → Quectel Qualcomm
