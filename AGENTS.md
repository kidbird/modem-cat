# AGENTS.md

> 仓库级共享入口。根 `CLAUDE.md` 委托到本文件；本文件保留路由、硬约束、文档 owner 规则。

## Required Read Order

1. 本文件（路由 + 硬约束）
2. `docs/ARCHITECTURE.md`（整体架构 + 数据流）
3. `docs/CODE_MAP.md`（前端 UI → IPC → 后端 三列映射）
4. 按任务类型读取：
   - **前端改动** → `docs/CODE_MAP.md` §2 + `docs/CALL_FLOW.md`
   - **后端 IPC / HAL** → `docs/ARCHITECTURE.md` §3 + `docs/AT_COMMANDS.md`
   - **新厂商适配** → `docs/ARCHITECTURE.md` §9 + `docs/AT_COMMANDS.md`
   - **构建/部署** → `docs/BUILD.md`

## Task Routing

| Task Type | Workdir | Read Next |
|-----------|---------|-----------|
| **Frontend (HTML/CSS/JS)** | `src/desktop/` | `docs/CODE_MAP.md` + `docs/CALL_FLOW.md` |
| **Tauri Backend (IPC/State)** | `src-tauri/src/` | `docs/ARCHITECTURE.md` + `docs/CODE_MAP.md` |
| **HAL (modem-hal)** | `modem-hal/src/` | `docs/ARCHITECTURE.md` + `docs/AT_COMMANDS.md` |
| **New Vendor** | `modem-hal/src/vendors/` | `docs/ARCHITECTURE.md` §9 + `docs/AT_COMMANDS.md` |

## Source Of Truth

- `docs/ARCHITECTURE.md` — 架构、数据流、模块职责、线程模型（唯一权威）
- `docs/CODE_MAP.md` — 52 个 IPC 命令 × 前端触发点 × AT 解析函数 三列映射
- `docs/AT_COMMANDS.md` — AT 命令→解析函数→平台差异（与 `modem_factory.rs` 厂商检测严格对齐）
- `docs/CALL_FLOW.md` — IPC 调用全链路时序图
- `docs/REVIEW.md` — 已知问题清单与修复状态
- `docs/BUILD.md` — 构建命令与平台注意事项
- `docs/TECH_STACK.md` — 技术栈与依赖

---

## Hard Constraints

### 1. 单一流程 / 单一入口

- **同一功能只允许一条主处理流程**。禁止通过 fallback、备用 handler、`unwrap_or_default()` 掩盖 AT 命令失败。
- **同一业务只允许一个对外 IPC 入口**。不要为同一功能同时维护 `connect_serial` + `auto_connect_at` 之外第三套连接路径。
- IPC handler 已统一走 `with_vendor!` / `with_vendor_cid!` 宏，**禁止新增 handler 绕过宏直接操作 transport/vendor 锁**。

### 2. 单一真相源

- **前端状态**：`app.js` 中的 `state` 对象是唯一前端真相源。`localStorage` 只存 theme / lang / ui-scale 三项。
- **后端状态**：`AppState` (lib.rs) 中的 `transport` / `vendor` / `data_cid` / `connected_port` / `at_command_log` 是唯一后端真相源。
- **模组状态**：统一走 `get_modem_status` IPC，其它 IPC 查询结果如果已在 status 中包含则复用、不重复查询。
- **禁止**：在前端 `state` 之外维护第二份缓存副本；禁止在后端 `AppState` 之外维护第二份配置存储。

### 3. AT 命令规范

- **AT 命令来源只有两个**：
  - `docs/` 下的 AT 手册 Markdown 参考文档
  - 用户明确给出的 AT 命令或修正
- **禁止**从其他平台分支、其他厂商型号、"类似命令"、论坛片段推断 AT 命令。
- **禁止**为同一查询功能使用 fallback AT 命令：一条 AT 命令失败即报错，不要静默切到第二条命令。
- `unwrap_or_default()` / `unwrap_or(0)` 在 AT 响应处理中属于**隐藏 fallback**，新增代码禁止使用。
- 已有 silent fallback（见 REVIEW.md）记为技术债，修改相关功能时一并修复。

### 4. 字符串与 Buffer 安全（Rust）

- **禁止新增 `.unwrap()` 在 Mutex 锁路径上**。锁操作统一走 `with_vendor!` 宏或 `.map_err()`。
- **禁止硬编码魔数 buffer**：`4096`/`8192`/`16384` 必须以命名常量定义，并注释来源（协议字段上限、实测最大值）。
- **超时/Delay 常量必须以命名常量定义**，并注释理由。参考 `serial.rs:12-19` 的命名规范。
- AT 命令拼接统一使用 `format!()`，禁止手动 `+` / `push_str` 拼接含引号的 AT 参数。
- `validate_at_string` / `validate_raw_at_command` 对所有用户可控的 AT 参数字符串强制执行。

### 5. 代码生命周期

- **替换旧实现时必须同步删除旧代码**。不保留注释掉的旧函数、`#[allow(dead_code)]` 标记的备份实现。
- **新增 IPC 命令后**：确认在前端有调用方，在 `invoke_handler!` 中注册，在 `CODE_MAP.md` 中记录。
- **删除功能后**：同步清理前端调用点、IPC handler、trait 方法、文档引用。不留孤儿代码。
- **文件拆分 / 模块重构后**：确认旧文件/旧模块已删除，`mod` 声明已清理。

### 6. 文档维护

- `AGENTS.md` 是所有 Agent 的根契约。
- `CLAUDE.md` 是兼容薄层，只做委托，不重复维护内容。
- 架构/API/流程变更必须在同一次改动中更新对应 owner 文档。
- 文档中的行数/文件大小描述不得手动维护；标注"行号随 commit 漂移"或直接删除具体数字。

### 7. 构建与测试

- `cargo test --workspace` 必须在每次改动后通过。
- `cargo build -p modem-hal` 至少通过（完整 `cargo build --workspace` 在目标平台执行）。
- 前端无构建工具。JS 模块化使用 ES Modules（`<script type="module">`），不引入 Webpack/Vite。

---

## Ownership Rules

- `docs/ARCHITECTURE.md` — 架构 owner，重大重构必须同步更新
- `docs/AT_COMMANDS.md` — AT 命令 owner，新增/修改 AT 命令必须同步更新
- `docs/CODE_MAP.md` — IPC 映射 owner，增删 IPC 命令必须同步更新
- `docs/REVIEW.md` — 技术债追踪，修复后更新状态
- 环境、部署、测试命令等操作性信息，更新对应 owner 文档，不在本文件重复维护。
