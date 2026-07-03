# Tasks: 5G Modem调试工具

**Input**: Design documents from `specs/001-modem-debug-tool/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are NOT included by default - only add if explicitly requested in feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Project structure**: `src/`, `tests/`, `src-tauri/` at repository root
- Paths assume: `src/` (TypeScript), `src-tauri/` (Rust), `tests/` (Bun test)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Initialize Bun + TypeScript project with package.json in src/
- [X] T002 Initialize Tauri 2.x project in src-tauri/
- [X] T003 [P] Configure TypeScript (tsconfig.json) and Tauri config (tauri.conf.json)
- [X] T004 [P] Set up project structure: src/{cli,core,desktop,lib}, tests/, src-tauri/src/
- [X] T005 Install core dependencies: @tauri-apps/api, typescript, bun types

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Create TypeScript type definitions in src/core/types/index.ts (Connection, ModemStatus, NetworkInfo, HardwareInfo, ATCommand)
- [X] T007 [P] Implement connection state machine in src/core/connections/connection-manager.ts
- [X] T008 [P] Create AT command response parser in src/core/parser/at-parser.ts
- [X] T009 Implement Tauri Rust backend for serial port access in src-tauri/src/serial.rs
- [X] T010 [P] Implement Tauri Rust backend for Ethernet TCP/UDP in src-tauri/src/network.rs
- [X] T011 Create CLI entry point in src/cli/index.ts with argument parsing
- [X] T012 Set up logging infrastructure in src/lib/logger.ts

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 连接模组 (Priority: P1) 🎯 MVP

**Goal**: Enable users to connect to 5G modems via USB Serial, Ethernet, or TTL interfaces

**Independent Test**: User can connect to a real or mocked modem and verify connection success

### Implementation for User Story 1

- [X] T013 [P] [US1] Implement USB Serial connection handler in src/core/connections/usb-serial.ts
- [X] T014 [P] [US1] Implement Ethernet TCP connection handler in src/core/connections/ethernet-tcp.ts
- [X] T015 [P] [US1] Implement TTL (UART) connection handler in src/core/connections/ttl-uart.ts
- [X] T016 [US1] Create CLI connect command in src/cli/commands/connect.ts
- [X] T017 [US1] Create CLI disconnect command in src/cli/commands/disconnect.ts
- [X] T018 [US1] Create CLI list-ports command in src/cli/commands/list-ports.ts
- [ ] T019 [US1] Implement connection health monitoring in src/core/connections/health-monitor.ts
- [ ] T020 [US1] Add auto-reconnection logic for disconnected connections

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - 获取模组状态信息 (Priority: P1)

**Goal**: Query and display modem status including network registration, signal strength, connection mode

**Independent Test**: User can query modem status and verify display of network registration, RSRP/RSRQ, connection mode

### Implementation for User Story 2

- [X] T021 [P] [US2] Create modem status query service in src/core/modem/status-service.ts
- [X] T022 [P] [US2] Implement AT command executor in src/core/modem/at-executor.ts
- [X] T023 [US2] Create CLI status command in src/cli/commands/status.ts
- [ ] T024 [US2] Implement real-time status monitoring with polling in src/core/modem/status-monitor.ts

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - AT命令调试 (Priority: P1)

**Goal**: Send AT commands to modem and view responses, support command history and scripts

**Independent Test**: User can send AT command and verify response display

### Implementation for User Story 3

- [X] T025 [P] [US3] Implement AT command service in src/core/modem/at-service.ts
- [X] T026 [P] [US3] Create command history manager in src/core/modem/history-manager.ts
- [X] T027 [US3] Create CLI at command in src/cli/commands/at.ts
- [ ] T028 [US3] Create CLI at-script command in src/cli/commands/at-script.ts
- [X] T029 [US3] Create CLI history command in src/cli/commands/history.ts

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - 蜂窝网络信息查询 (Priority: P2)

**Goal**: Query detailed cellular network information including operator, band, bandwidth, neighbor cells

**Independent Test**: User can query network info and verify display of carrier, band, bandwidth

### Implementation for User Story 4

- [ ] T030 [P] [US4] Implement network info query service in src/core/modem/network-service.ts
- [ ] T031 [US4] Create CLI network-info command in src/cli/commands/network-info.ts

---

## Phase 7: User Story 5 - 硬件信息查询 (Priority: P2)

**Goal**: Query hardware information including model, firmware version, system status

**Independent Test**: User can query hardware info and verify display of model, firmware

### Implementation for User Story 5

- [ ] T032 [P] [US5] Implement hardware info query service in src/core/modem/hardware-service.ts
- [ ] T033 [US5] Create CLI hardware-info command in src/cli/commands/hardware-info.ts

---

## Phase 8: User Story 6 - 模组配置 (Priority: P2)

**Goal**: Configure modem parameters including network mode, APN settings

**Independent Test**: User can configure modem and verify configuration takes effect

### Implementation for User Story 6

- [ ] T034 [P] [US6] Implement modem configuration service in src/core/modem/config-service.ts
- [ ] T035 [US6] Create CLI config command in src/cli/commands/config.ts

---

## Phase 9: User Story 7 - CLI模式支持 (Priority: P1)

**Goal**: Full CLI functionality for headless and SSH scenarios with JSON/human output

**Independent Test**: User can run all core commands via CLI in headless environment

### Implementation for User Story 7

- [ ] T036 [P] [US7] Implement output formatter (JSON/human) in src/lib/formatter.ts
- [ ] T037 [P] [US7] Add --json and --human global flags to CLI
- [ ] T038 [US7] Create CLI help system with --help flag in src/cli/commands/help.ts
- [ ] T039 [US7] Add version command in src/cli/commands/version.ts
- [ ] T040 [US7] Implement interactive CLI mode with readline in src/cli/interactive.ts

**Checkpoint**: At this point, CLI mode should be fully functional

---

## Phase 10: User Story 8 - 跨平台桌面应用 (Priority: P2)

**Goal**: Desktop GUI application for macOS, Windows, Ubuntu

**Independent Test**: User can run desktop app on each platform and verify UI functionality

### Implementation for User Story 8

- [ ] T041 [P] [US8] Set up Tauri desktop frontend in src/desktop/index.html
- [ ] T042 [P] [US8] Create connection panel component in src/desktop/components/ConnectionPanel.tsx
- [ ] T043 [P] [US8] Create status display panel in src/desktop/components/StatusPanel.tsx
- [ ] T044 [P] [US8] Create AT command terminal panel in src/desktop/components/AtTerminal.tsx
- [ ] T045 [P] [US8] Create network info panel in src/desktop/components/NetworkInfoPanel.tsx
- [ ] T046 [P] [US8] Create hardware info panel in src/desktop/components/HardwareInfoPanel.tsx
- [ ] T047 [US8] Implement desktop main app in src/desktop/main.ts
- [ ] T048 [US8] Build desktop app and verify it runs on target platforms

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T049 [P] Add unit tests for core modules in tests/unit/
- [ ] T050 [P] Add integration tests for connection types in tests/integration/
- [ ] T051 Add error handling improvements across all commands
- [ ] T052 Optimize AT command response time to meet <2s requirement
- [ ] T053 Update quickstart.md with verified commands
- [ ] T054 Build and verify CLI works in headless mode (SSH test)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational - May integrate with US1 but should be independently testable
- **User Story 3 (P1)**: Can start after Foundational - Depends on US1 for connection, but AT commands testable with mock
- **User Story 4 (P2)**: Can start after Foundational - Depends on US1 for connection
- **User Story 5 (P2)**: Can start after Foundational - Depends on US1 for connection
- **User Story 6 (P2)**: Can start after Foundational - Depends on US1 for connection
- **User Story 7 (P1)**: Can start after Foundational - Integrates all prior CLI commands
- **User Story 8 (P2)**: Can start after Foundational - Depends on US1-US7 implementation

### Within Each User Story

- Models before services
- Services before commands
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, user stories 1, 2, 3 can start in parallel
- All tasks marked [P] within a user story can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all connection handlers in parallel:
Task: "Implement USB Serial connection handler in src/core/connections/usb-serial.ts"
Task: "Implement Ethernet TCP connection handler in src/core/connections/ethernet-tcp.ts"
Task: "Implement TTL (UART) connection handler in src/core/connections/ttl-uart.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 7 (CLI Mode) → Test independently → Deploy/Demo
6. Add User Stories 4, 5, 6 → Test independently → Deploy/Demo
7. Add User Story 8 (Desktop) → Final release

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Connection)
   - Developer B: User Story 2 (Status)
   - Developer C: User Story 3 (AT Commands)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Summary

- **Total Tasks**: 54
- **User Stories**: 8
- **Parallel Opportunities**: 20+ tasks can run in parallel
- **MVP Scope**: Phase 1-3 (Setup + Foundational + US1) - connection capability
- **MVP Test**: Connect to modem and verify connection success
