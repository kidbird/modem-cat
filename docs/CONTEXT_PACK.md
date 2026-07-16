# Context Pack

> 目的：让 `AGENTS.md` 保持简洁。先读本文件，再按任务需要加载 owner 文档，不要一次把所有文档塞进上下文。

## 1. Mandatory Load Order

### Frontend (HTML / CSS / JS)

1. `docs/CODE_MAP.md`
2. `docs/CALL_FLOW.md`
3. `docs/ARCHITECTURE.md`
4. 只有涉及具体 AT 语义时再读 `docs/AT_COMMANDS.md`

### Tauri Backend (IPC / AppState / Queue / Monitor / MQTT)

1. `docs/ARCHITECTURE.md`
2. `docs/CODE_MAP.md`
3. `docs/REVIEW.md`
4. 涉及具体命令语义时再读 `docs/AT_COMMANDS.md`

> 当前执行面不只在 `lib.rs`。实际 live 代码分散在 `src-tauri/src/lib.rs`（装配）、`handlers.rs`（业务 IPC）、`connection.rs`（连接 IPC）、`monitor.rs`（后台线程）、`mqtt.rs`（后台上报）。

### HAL / Parser / Vendor / Transport

1. `docs/AT_COMMANDS.md`
2. `docs/ARCHITECTURE.md`
3. `docs/REVIEW.md`
4. 需要精确手册语法时再读对应 Quectel 文档

### Build / Deploy / Toolchain

1. `docs/BUILD.md`
2. `docs/TECH_STACK.md`
3. 涉及架构边界时补读 `docs/ARCHITECTURE.md`

### Docs / Review / Cleanup

1. `docs/REVIEW.md`
2. 要改哪份 owner 文档，就读哪份 owner 文档

## 2. Source Of Truth

- `docs/ARCHITECTURE.md`
  说明 live 架构、状态边界、唯一 AT 队列、连接模式、后台线程。
- `docs/AT_COMMANDS.md`
  说明正式 AT 合同、平台差异、禁止复制的历史 fallback。
- `docs/CODE_MAP.md`
  说明前端触发点、IPC 名称、后端 live handler 映射（含 `handlers.rs` / `connection.rs` / `dloader.rs`）。
- `docs/CALL_FLOW.md`
  说明主要 UI → IPC → HAL 调用链。
- `docs/REVIEW.md`
  说明当前 live 技术债、文档漂移、必须优先收敛的问题。
- `docs/BUILD.md`
  说明构建命令、平台注意事项、产物路径。
- `docs/TECH_STACK.md`
  说明当前技术栈、依赖、运行方式、显式配置项（如 MQTT / WebSocket 认证边界）。

## 3. On-Demand Lookup Rules

- 需要**唯一队列 / 状态 owner / 后台线程**：先看 `docs/ARCHITECTURE.md`
- 需要**精确 IPC 名**：先看 `docs/CODE_MAP.md`
- 需要**精确 AT 语法**：先看 `docs/AT_COMMANDS.md`，还不够再看 Quectel 手册
- 需要**现存偏差 / 不该复制的旧逻辑**：先看 `docs/REVIEW.md`
- 需要**构建 / 依赖 / 运行方式**：先看 `docs/BUILD.md` 和 `docs/TECH_STACK.md`

## 4. Maintenance Rules

- 变更 live 架构、AT 合同、IPC 映射、构建方式时，必须同步更新对应 owner 文档。
- `AGENTS.md` 和 `CLAUDE.md` 只保留入口性质内容，不重复维护实现细节。
- 文档里不要手工维护行数、文件大小、符号数这类易漂移信息。
