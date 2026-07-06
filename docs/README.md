# Docs 总目录

> 2026-06-18 更新
> Agent 入口在仓库根：[AGENTS.md](../AGENTS.md) 是根契约，[CLAUDE.md](../CLAUDE.md) 只做委托。

仓库根的 [README.md](../README.md) 面向“第一次打开仓库的人”，提供产品说明、架构概览、环境准备、构建命令和发布产物选择；本文件继续只做 `docs/` 目录索引。

## 文档列表

| 文档 | 用途 | 受众 |
|---|---|---|
| [CONTEXT_PACK.md](CONTEXT_PACK.md) | 任务路由、owner 文档索引、按需加载规则 | Agent / 新协作者 |
| [ARCHITECTURE.md](ARCHITECTURE.md) | 整体架构、模块职责、状态边界、后台模块、连接模式 | 所有读者 |
| [CODE_MAP.md](CODE_MAP.md) | 前端 UI 元素 → IPC 命令 → 后端 handler 三列映射；Tauri 事件订阅 | 改前端 / 加新 IPC 时查 |
| [CALL_FLOW.md](CALL_FLOW.md) | 启动、连接、查询、配置、数据连接、5GLAN、频段、AT 调试、后台监控流程图 | 调试 / 改后端时查 |
| [AT_COMMANDS.md](AT_COMMANDS.md) | 正式主合同的 AT 指令映射（与 `modem_factory.rs` 厂商检测对齐） | 改 HAL / 写新 vendor 时查 |
| [Quectel_AT_Commands.md](Quectel_AT_Commands.md) | Quectel RGx00U & RM500U 系列 AT 命令手册（参考自 PDF） | 翻老型号手册 |
| [MODEM_BAND_SPECS.md](MODEM_BAND_SPECS.md) | 型号硬件频段参考（当前未在代码中直接使用） | 翻规格 |
| [BUILD.md](BUILD.md) | macOS / Windows 构建、版本管理、发版流程、常见问题 | 发布时查 |
| [TECH_STACK.md](TECH_STACK.md) | 当前技术栈、依赖、运行约束、辅助模块 | 新人入门 |
| [CODING.md](CODING.md) | AT 输入校验、敏感信息、前端入口、文档同步规范 | 改代码前查 |
| [REVIEW.md](REVIEW.md) | 当前已知技术债、风险点、与正式设计合同的偏差 | 排期修 bug 时查 |

## 需求与方案文档

| 路径 | 用途 |
|---|---|
| [specs/001-modem-debug-tool](specs/001-modem-debug-tool/) | 桌面工具初始需求包：`spec` / `plan` / `tasks` / `contracts` / `research` |
| [superpowers/specs](superpowers/specs/) | 历史设计草案与专项设计说明 |
| [superpowers/plans](superpowers/plans/) | 历史实施计划与拆解记录 |

## 发版产物速查

| 产物 | 路径模式 | 使用场景 |
|---|---|---|
| 轻量免安装包 | `dist/ModemCat_vX.Y.Z_portable-lite.zip` | 目标 Windows 机器已安装系统 WebView2，想优先减小下载体积 |
| 完整免安装包 | `dist/ModemCat_vX.Y.Z_portable.zip` | 目标机器可能没有 WebView2，需要离线直接运行 |
| NSIS 安装包 | `dist/Modem Cat_X.Y.Z_x64-setup.exe` | 需要常规安装流程、桌面/开始菜单入口、适合普通终端用户 |
| MSI 安装包 | `dist/Modem Cat_X.Y.Z_x64_zh-CN.msi` | 需要企业分发、统一部署或更标准的 Windows 安装介质 |

补充说明：
轻量免安装包和完整免安装包都包含 `modem-cat.exe`、ADB 运行组件、`r26-cli` 固件 sidecar。
两者唯一差异是完整免安装包额外包含与 `modem-cat.exe` 同层的 `webview2-runtime/`。
详细构建方式和注意事项见 [BUILD.md](BUILD.md)。

## 文档维护原则

1. **先看根入口，再按需加载**。从 [AGENTS.md](../AGENTS.md) 开始，经由 [CONTEXT_PACK.md](CONTEXT_PACK.md) 路由到对应 owner 文档，不要把所有细节塞回根文件。
2. **代码与文档不一致时，以代码为准**。修文档时先确认 live path，再决定是更新 owner 文档还是把偏差登记到 [REVIEW.md](REVIEW.md)。
3. **正式合同与技术债分开写**。主流程、主合同写进 `ARCHITECTURE.md` / `AT_COMMANDS.md` / `CODE_MAP.md`；fallback、兼容分支、历史残留写进 `REVIEW.md`，不要混写。
4. **改代码必同步改 owner 文档**。新增/删除 IPC、调整 AT 合同、改变调用路径时，同一次改动内完成文档更新。
5. **需求文档统一留在 `docs/` 下**。仓库级需求 / 计划 / 任务文档放在 `docs/specs/` 或 `docs/superpowers/`，不要再在仓库根平铺新的 `specs/` 目录。
6. **本地参考导出物不放正式索引**。HTML 导出、TXT 转写、临时手册整理稿、PDF 本地副本都不算 owner 文档；若只用于个人检索，保持 gitignored，不写进本目录索引。

## 历史归档

旧说明保留在 git 历史中；当前文档已经按现有技术栈重整，后续以当前 owner 文档为准：

- 旧 `ARCHITECTURE.md` 曾混入已下线结构、旧入口与历史估算
- 旧 `CODE_MAP.md` 曾遗漏 License / Factory / Firmware IPC，并把旧连接路径写成当前入口
- 旧 `TECH_STACK.md` 曾带入其他项目的前端假设，不适用于当前 plain HTML/CSS/JS + Tauri 结构
