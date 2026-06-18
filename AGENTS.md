# AGENTS.md

> 仓库级共享入口。`CLAUDE.md` 只委托到本文件；本文件只保留任务路由、根约束、文档 owner 规则。

## Required Read Order

1. 本文件
2. `docs/CONTEXT_PACK.md`
3. 按 `docs/CONTEXT_PACK.md` 的任务路由继续读取 owner 文档
4. 不要把架构、AT、API、构建细节回填到本文件

## Task Routing

| Task Type | Workdir | Read Next |
|---|---|---|
| Frontend (HTML/CSS/JS) | `src/desktop/` | `docs/CONTEXT_PACK.md` + `docs/CODE_MAP.md` + `docs/CALL_FLOW.md` |
| Tauri Backend (IPC/State/Queue/MQTT) | `src-tauri/src/{lib,mqtt}.rs` | `docs/CONTEXT_PACK.md` + `docs/ARCHITECTURE.md` + `docs/CODE_MAP.md` |
| License / Factory / Firmware | `src-tauri/src/{license,factory,dloader}.rs` | `docs/CONTEXT_PACK.md` + `docs/ARCHITECTURE.md` + `docs/CODE_MAP.md` + `docs/TECH_STACK.md` |
| HAL / Parser / Vendor / Transport | `modem-hal/src/` | `docs/CONTEXT_PACK.md` + `docs/AT_COMMANDS.md` + `docs/ARCHITECTURE.md` + `docs/REVIEW.md` |
| Build / Deploy / Docs | repo root | `docs/CONTEXT_PACK.md` + `docs/BUILD.md` + `docs/TECH_STACK.md` + 对应 owner 文档 |

## Hard Constraints

- **唯一 AT 队列**：所有 live modem I/O 只能复用 `AppState.transport` 持有的 `AtTransport`，最终统一汇聚到 `AtTransport::send_at`。禁止新增第二条 AT 发送队列、后台直连 transport、或绕过现有串行化路径的 helper。
- **唯一调用路径**：同一业务只允许一条 live IPC / transport / parser 主路径。旧 handler、旧模块、旧脚本、旧文档在替换时必须同步删除。
- **实时状态禁止 fallback**：状态查询或配置读取失败必须直接报错。禁止二次改发 AT、`unwrap_or_default()` / `unwrap_or(0)`、伪默认值、静默吞错。
- **单一真相源**：前端状态以 `state` 为准；后端 modem 状态以 `AppState` 为准；License / Factory / Firmware 各自以其受管状态 owner 为准。不要维护第二份 live 镜像。
- **AT 命令来源受限**：只允许来自 `docs/` AT 手册或用户明确给出的修正；禁止按“相似型号 / 论坛片段 / 其他平台分支”猜命令。
- **用户可控 AT 参数必须校验**：所有字符串 AT 参数统一走 `validate_at_string` / `validate_raw_at_command` / `validate_cid`，AT 拼接统一用 `format!()`。
- **锁与敏感信息安全**：Mutex 锁路径禁止 `.unwrap()`；PLMN / APN / token / password 等敏感字段不得设置公开默认值，进入 AT 日志前必须经 `redact_at_command`。
- **文档分层**：`AGENTS.md` 与 `CLAUDE.md` 只保留入口性质内容；实现细节写进 `docs/CONTEXT_PACK.md` 与 owner 文档。
- **验证基线**：改动完成后至少跑 `cargo test --workspace` 和 `cargo build -p modem-hal`；完整桌面构建说明见 `docs/BUILD.md`。

## Ownership Rules

- `docs/ARCHITECTURE.md`：架构、状态边界、AT 队列、连接模式、后台模块
- `docs/AT_COMMANDS.md`：AT 命令合同、平台差异、禁止复制的历史兼容路径
- `docs/CODE_MAP.md`：前端触发点 → IPC → 后端执行面
- `docs/CALL_FLOW.md`：主要 UI → IPC → HAL 调用链
- `docs/REVIEW.md`：当前 live 技术债与文档/代码偏差
- `docs/BUILD.md` / `docs/TECH_STACK.md`：工具链、构建、依赖、运行方式
- 架构、AT、调用路径、构建方式变化必须同一次更新对应 owner 文档
