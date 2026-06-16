# AGENTS.md

> 仓库级统一入口。所有 agent 必读。根 `CLAUDE.md` 委托到本文件。

## 必读顺序

1. 本文件
2. `docs/ARCHITECTURE.md`（架构 + 数据流 + Source Of Truth + Doc Owner）
3. 按任务类型：
   - 前端 → `docs/CODE_MAP.md` + `docs/CALL_FLOW.md`
   - 后端 IPC / HAL → `docs/AT_COMMANDS.md` + `docs/REVIEW.md`
   - 新厂商 → `docs/ARCHITECTURE.md §9` + `docs/AT_COMMANDS.md`
   - 构建 → `docs/BUILD.md`

## Hard Constraints

### 1. 流程/入口唯一性
- 同一功能一条主流程。禁止 fallback / 备用 handler / `unwrap_or_default()` 掩盖 AT 失败。
- 同一业务一个 IPC 入口。handler 统一走 `with_vendor!` / `with_vendor_cid!` 宏。

### 2. 状态唯一性
- 前端 `app.js::state` / 后端 `AppState` 各自唯一真相源。`localStorage` 仅存 theme/lang/ui-scale。
- 模组状态统一走 `get_modem_status`，其它 IPC 不重复查询。

### 3. AT 命令规范
- 来源：`docs/` AT 手册 + 用户明确给出。禁止从其他分支/型号推断。
- 同功能禁止 fallback AT；失败即报错。
- `unwrap_or_default()` / `unwrap_or(0)` 在 AT 响应处理中禁止。
- 用户可控字符串走 `validate_at_string` / `validate_raw_at_command` / `validate_cid`（详见 CODING.md）。

### 4. 字符串/Buffer/锁安全（Rust）
- Mutex 锁路径禁止 `.unwrap()`。统一走宏或 `.map_err()`。
- 魔数 buffer / 超时必须为命名常量，并注释来源。
- AT 命令拼接统一 `format!()`，禁止手动 `+` / `push_str`。

### 5. 代码生命周期
- 替换旧实现必须同步删除旧代码（不保留注释掉的备份）。
- 新增 IPC 必填 3 项：前端 caller、`invoke_handler!` 注册、`CODE_MAP.md` 记录。
- 删除功能同步清理前端调用/IPC handler/trait 方法/文档引用。

### 6. 文档维护
- 架构/API/流程变更必须同次改动更新对应 owner 文档。
- 文档禁 commit hash / 精确行号 / 已删除文件历史记录。
- `CLAUDE.md` 是兼容薄层，不重复维护。

### 7. 构建与测试
- `cargo test --workspace` 每次改动后必过。
- 前端无构建工具；ES Modules（`<script type="module">`）。

## 敏感信息

- 禁为 PLMN / APN / token 等敏感字段设公开默认（`"12345678"`、`"admin"`、空密码等）。用户未传时返回 `Err`。
- 所有进入 AT 日志的命令必须经 `redact_at_command`。
