# Modem-Cat Constitution

<!-- SYNC IMPACT REPORT v0.1.0 → v1.0.0 -->
<!-- Version change: 0.1.0 → 1.0.0 (MAJOR - initial constitution) -->
<!-- Added: 5 core principles, 2 additional sections -->
<!-- Templates requiring updates: plan-template.md ✅, spec-template.md ✅, tasks-template.md ✅ -->

## Core Principles

### I. CLI-First Architecture
Every feature must function in CLI mode before desktop UI. Core operations (AT commands, status queries, configuration) MUST be accessible via command-line interface. Text in/out protocol: stdin/args → stdout, errors → stderr. Support both JSON and human-readable output formats.

**Rationale**: User explicitly requires CLI mode for SSH and headless scenarios. CLI is the foundation; desktop is an enhancement.

### II. Cross-Platform Support
MUST support macOS, Windows, and Ubuntu (Linux desktop). Platform-specific code must be abstracted behind unified APIs. Connection interfaces (USB serial, Ethernet, TTL) must work consistently across all platforms.

**Rationale**: Target users work across different operating systems. No platform should have degraded functionality.

### III. Multiple Connection Interfaces
System MUST support USB Serial, Ethernet (TCP/UDP), and TTL (UART) connections. Each interface type requires dedicated connection manager with consistent command/response semantics. Connection health monitoring and auto-reconnection for unstable links.

**Rationale**: User specified these three interfaces as essential for different debugging scenarios.

### IV. AT Command Interface
AT command execution is the primary debugging mechanism. System MUST provide: raw AT command input/output, command history, response parsing for common AT responses, and support for AT command scripts/batches.

**Rationale**: AT commands are the industry standard for modem debugging. This is the core value proposition.

### V. Observability & Monitoring
Real-time status monitoring for modem state, cellular network info, signal quality, and hardware health. Structured logging required for all operations. Operation timestamps and duration tracking.

**Rationale**: User needs to "monitor module running status" - observability is essential for debugging.

## Technology Stack

**Language/Version**: Python 3.10+ (cross-platform, rich serial/Ethernet libraries)  
**UI Framework**: Tauri or Electron (cross-platform desktop) or rich CLI (Textual)  
**Connection Libraries**: pyserial (USB/TTL), socket (Ethernet)  
**Testing**: pytest with serial Ethernet mock support  
**Target Platform**: Desktop (macOS/Windows/Ubuntu) + CLI (headless)  

**Additional Constraints**:
- No cloud dependencies - all operations local
- Offline-capable - no internet required for core functionality
- Minimal dependencies - reduce install complexity

## Development Workflow

**Code Review**: All PRs require review before merge  
**Testing Gates**: 
- Unit tests for core modules
- Integration tests for each connection interface type
- Platform-specific testing on at least macOS and Linux (Windows if available)  
**CLI Validation**: Every feature tested in CLI mode before desktop UI  
**Breaking Changes**: Version bumping per semantic versioning - MAJOR for breaking CLI/API changes  

## Governance

This constitution supersedes all other practices. Amendments require:
1. Documentation of proposed change
2. Rationale explaining why the change is necessary
3. Migration plan if applicable

All PRs/reviews must verify compliance with core principles. Complexity must be justified against CLI-first and cross-platform requirements.

**Version**: 1.0.0 | **Ratified**: 2026-04-04 | **Last Amended**: 2026-04-04
