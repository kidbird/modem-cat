# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

modem-cat is a 5G modem debugging tool with two interfaces:
- **CLI**: Bun/TypeScript command-line tool (`src/cli/`)
- **Desktop**: Tauri app with Rust backend (`src-tauri/`) and web frontend (`src/desktop/`)

## Commands

```bash
# CLI development
bun run src/cli/index.ts           # Run CLI directly
bun build src/cli/index.ts         # Build CLI to dist/

# Desktop app
cd src-tauri && cargo build --release   # Build Tauri desktop app

# TypeScript
bun test                           # Run tests
bun run typecheck                  # TypeScript type checking
```

## Architecture

### CLI (`src/cli/`)
Command-based CLI using Bun. Entry point `src/cli/index.ts` dispatches to commands in `src/cli/commands/`.

### Core Services (`src/core/`)
- `connections/connection-manager.ts` - Manages modem connections
- `modem/` - AT command execution, config, hardware info, network, status
- `parser/at-parser.ts` - Parses AT command responses

### Desktop App (`src-tauri/`)
Rust backend exposing Tauri commands for:
- Serial port listing, connection, AT command sending
- TCP network operations

The desktop frontend (`src/desktop/`) is a web UI loaded by Tauri.

## Tech Stack

- **CLI**: Bun 1.0+ with TypeScript 5.x
- **Desktop**: Tauri 2.x (Rust + web frontend)
- **Dependencies**: `serialport` (Rust), `@tauri-apps/api`

## Recent Changes

- 001-modem-debug-tool: Added Bun 1.0+ with TypeScript 5.x
