# Research: 5G Modem调试工具

**Date**: 2026-04-04
**Feature**: 001-modem-debug-tool

## Technical Decisions

### 1. Bun + TypeScript Stack

**Decision**: Bun 1.0+ with TypeScript 5.x

**Rationale**:
- User explicitly requested Bun + TypeScript
- Bun provides excellent cross-platform support
- Fast startup time suitable for CLI tools
- Native FFI support for serial port access
- TypeScript for type safety

**Alternatives considered**:
- Node.js + TypeScript: More mature ecosystem but slower startup
- Python: Suggested in constitution but user preferred Bun

### 2. Tauri 2.x for Desktop

**Decision**: Tauri 2.x (Rust backend + WebView frontend)

**Rationale**:
- Cross-platform (macOS/Windows/Linux)
- Smaller binary size than Electron
- Native performance
- TypeScript support in frontend
- Claude Code uses similar architecture

**Alternatives considered**:
- Electron: Larger binary, more memory, but wider web compatibility
- Native UI (Qt/wxWidgets): Higher complexity, less web flexibility

### 3. Serial Port Access

**Decision**: Use Rust crate `serialport` via Tauri commands + FFI

**Rationale**:
- `serialport` crate provides cross-platform serial access
- Bun doesn't have mature serial library, so use Tauri backend
- Rust implementation is reliable and well-maintained

**Alternatives considered**:
- Pure Bun FFI: Unstable across platforms
- Node.js serialport: Would require separate runtime

### 4. CLI Framework

**Decision**: Use Tauri CLI + custom command handler

**Rationale**:
- Tauri has built-in CLI support
- Can share business logic with desktop
- Structured command output

**Alternatives considered**:
- @clack/prompts: Good for interactive prompts, but Tauri CLI sufficient

### 5. Architecture Pattern

**Decision**: Claude Code-inspired monorepo

**Rationale**:
- Clear separation between CLI and Desktop
- Shared core business logic in src/core/
- Platform-specific code isolated in src-tauri/
- TypeScript frontend for desktop UI
