# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

modem-cat is a 5G modem debugging tool with three interfaces:
- **CLI (TS)**: Bun/TypeScript command-line tool (`src/cli/`)
- **Desktop**: Tauri app with Rust backend (`src-tauri/`) and web frontend (`src/desktop/`)
- **Embedded CLI**: Static Rust binary for Linux (`modem-cli-embedded/`)

The core modem logic lives in `modem-hal/`, a standalone Rust crate consumed by both the Tauri backend and the embedded CLI.

## Commands

```bash
# CLI (TypeScript)
bun run src/cli/index.ts           # Run CLI directly
bun build src/cli/index.ts         # Build CLI to dist/
bun test                           # Run tests
bun run typecheck                  # TypeScript type checking

# Desktop app
cd src-tauri && cargo build --release   # Build Tauri desktop app

# Rust workspace (all crates)
cargo build --workspace            # Build everything
cargo test --workspace             # Run all Rust tests

# Embedded CLI
cargo build -p modem-cli-embedded --release --target aarch64-unknown-linux-gnu
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
- `napi-feature` — compiles napi-rs `ModemHandle` class for Bun/TS native addon

### Desktop App (`src-tauri/`)
Tauri 2.x Rust backend. Delegates modem logic to `modem-hal`.
- `src/at_adapter.rs` — calls modem-hal transport + at_parser to build status structs
- `src/at_parser.rs` — AT response parsers (Quectel-specific, used by at_adapter)
- `src/lib.rs` — all Tauri `#[tauri::command]` handlers

### Embedded CLI (`modem-cli-embedded/`)
Minimal clap CLI outputting JSON. Targets aarch64-unknown-linux-gnu (musl) for embedded Linux.
Subcommands: `status`, `signal`, `connect <cid>`, `disconnect <cid>`.

### CLI / Core (`src/cli/`, `src/core/`)
TypeScript CLI using Bun. Entry `src/cli/index.ts` dispatches to `src/cli/commands/`.
Core services in `src/core/connections/` and `src/core/modem/`.

## Tech Stack

- **modem-hal**: Rust, `serialport 4`, `serde`, optional `napi 2`
- **Desktop**: Tauri 2.x (Rust + web frontend), `@tauri-apps/api`
- **Embedded CLI**: Rust, `clap 4`, statically linked via musl
- **TS CLI**: Bun 1.0+ with TypeScript 5.x

## Vendor Detection

`ModemFactory::create()` queries `AT+CGMM` and matches model string:
- `MT5700` → TdTech (`AT^` commands)
- `RG200U / RM500U / RG501U / RM501U` → Quectel UniSoc
- `RG520N / RM520N / RG525F / RG530F / RM530N / RG540F / RM540N` → Quectel Qualcomm
