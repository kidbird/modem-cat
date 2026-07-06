# AGENTS.md

> 仓库级强约束入口。`CLAUDE.md` 只委托到本文件；实现细节只允许落在 `docs/` owner 文档与代码里。

## Required Read Order

1. 本文件
2. `docs/CONTEXT_PACK.md`
3. 按 `docs/CONTEXT_PACK.md` 选择对应 owner 文档
4. 再读取将被修改或评审的目标文件

## Scope Gate

在开始修改或 review 之前，先按目标路径归类；跨多个区域时，相关 owner 文档都必须读。

| Area | Paths | Required Docs |
|---|---|---|
| Frontend | `src/desktop/` | `docs/CODE_MAP.md`, `docs/CALL_FLOW.md`, `docs/TECH_STACK.md` |
| Tauri Connection / IPC / Monitor / MQTT | `src-tauri/src/{lib,handlers,connection,monitor,mqtt}.rs` | `docs/ARCHITECTURE.md`, `docs/CODE_MAP.md`, `docs/CALL_FLOW.md`, `docs/REVIEW.md` |
| Factory / Firmware | `src-tauri/src/{factory,dloader}.rs` | `docs/ARCHITECTURE.md`, `docs/CODE_MAP.md`, `docs/TECH_STACK.md`, `docs/REVIEW.md` |
| HAL / Parser / Vendor / Transport | `modem-hal/src/` | `docs/AT_COMMANDS.md`, `docs/ARCHITECTURE.md`, `docs/REVIEW.md` |
| Build / Docs / Guardrails | repo root, `docs/`, `scripts/verify-docs.sh` | `docs/BUILD.md`, `docs/TECH_STACK.md`, 对应 owner 文档 |

## Hard Constraints

- **唯一 AT 队列**：所有 live modem I/O 只能复用 `AppState.transport` 持有的 `AtTransport`，最终统一汇聚到 `AtTransport::send_at`。禁止新增第二条 AT 发送队列、后台直连 transport、或绕过现有串行化路径的 helper。
- **唯一业务主路径**：同一业务只允许一条 live IPC / transport / parser 主路径。替换旧路径时，旧 handler、旧模块、旧脚本、旧文档必须同一次删除或改写。
- **实时读取必须严格失败**：状态查询、配置读取、认证配置读取失败时必须直接返回错误。禁止二次改发 AT、`unwrap_or_default()`、`unwrap_or(0)`、伪默认值、静默吞错。
- **单一真相源**：前端 live 状态以 `state` 为准；后端 modem live 状态以 `AppState` 为准；Factory / Firmware 各自以其 owner state 为准。禁止维护第二份 live 镜像。
- **AT 命令来源受限**：只允许来自 `docs/` AT 手册、owner 文档，或用户明确给出的修正；禁止按相似型号、论坛片段、历史分支猜命令。
- **AT / 认证输入必须显式校验**：用户可控字符串 AT 参数统一走 `validate_at_string` / `validate_raw_at_command` / `validate_cid`；AT 拼接统一用 `format!()`；WebSocket / MQTT / 设备认证信息不得偷补公开默认值。
- **敏感信息禁止公开默认值**：PLMN / APN / token / password / WebSocket 凭据 / MQTT 凭据等敏感字段，未提供时必须保持空缺或报错，禁止 fallback 到 `"admin"`、空密码、固定 broker 凭据等默认值。
- **运行时锁路径禁止 panic**：生产代码里的 Mutex 锁路径禁止 `.unwrap()`；必须返回错误或记录明确日志。
- **文档分层必须保持**：`AGENTS.md` / `CLAUDE.md` 只保留入口、边界和强约束；架构、AT 合同、调用链、构建细节写进各自 owner 文档。
- **文档和护栏同改**：凡是改动 live 架构、IPC、AT 合同、连接方式、认证方式、构建验证方式，必须同一次更新对应 owner 文档和 `scripts/verify-docs.sh`。
- **review 纪律**：review / cleanup 任务必须给出可定位的文件证据；已修复的技术债从 `docs/REVIEW.md` 直接移除，不保留“已修历史年表”。
- **验证基线**：改动完成后至少运行 `cargo test --workspace`、`cargo build -p modem-hal`、`bash scripts/verify-docs.sh`。
- **构建产物统一输出**：所有构建打包的产物（MSI/NSIS 安装包、便携版可执行文件、sidecar、portable ZIP 等）必须统一放到 `dist/` 根目录下；禁止再按类别创建 `dist/installer/`、`dist/portable/` 这类分组子目录。若 `dist/modem-cat.exe` 需要作为 fixed WebView2 便携主程序直接运行，则只允许保留配套的根级 `dist/webview2-runtime/` 目录。打包脚本见 `build.ps1`，构建约束见 `docs/BUILD.md`。

## Ownership Rules

- `docs/ARCHITECTURE.md`：架构边界、状态 owner、AT 队列、后台模块、认证边界
- `docs/AT_COMMANDS.md`：正式 AT 合同、平台差异、允许的快捷 AT 边界
- `docs/CODE_MAP.md`：前端触发点、IPC 名称、后端执行面映射
- `docs/CALL_FLOW.md`：关键 UI / IPC / HAL 调用链、队列与日志时序
- `docs/REVIEW.md`：当前仍存在的 live 技术债、代码/文档偏差
- `docs/BUILD.md` / `docs/TECH_STACK.md`：工具链、构建、依赖、运行和配置方式
- `docs/CODING.md`：输入校验、敏感信息、提交前检查约定

## Change Gate

满足任一条件时，代码改动不算完成：

- owner 文档还没同步
- `scripts/verify-docs.sh` 还没覆盖新的约束或路径
- 仍残留旧路径、旧文档、旧默认值、旧 fallback
- 验证命令没跑，或失败后没有继续收敛
