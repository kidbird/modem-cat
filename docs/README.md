# Docs 总目录

> 2026-06-01 重新整理
> 项目根还有 [CLAUDE.md](../CLAUDE.md) 和 [AGENTS.md](../AGENTS.md) 作为 agent 入口。

## 文档列表

| 文档 | 用途 | 受众 |
|---|---|---|
| [ARCHITECTURE.md](ARCHITECTURE.md) | 整体架构、模块职责、关键数据流、厂商检测流程、添加新 vendor 步骤 | 所有读者 |
| [CODE_MAP.md](CODE_MAP.md) | 前端 UI 元素 → IPC 命令 → 后端 handler 三列映射；Tauri 事件订阅 | 改前端 / 加新 IPC 时查 |
| [CALL_FLOW.md](CALL_FLOW.md) | 启动、连接、查询、配置、数据连接、5GLAN、频段、AT 调试、后台监控 12 个流程图 | 调试 / 改后端时查 |
| [AT_COMMANDS.md](AT_COMMANDS.md) | 全平台 AT 指令功能映射（与 `modem_factory.rs` 严格对齐） | 改 HAL / 写新 vendor 时查 |
| [Quectel_AT_Commands.md](Quectel_AT_Commands.md) | Quectel RGx00U & RM500U 系列 AT 命令手册（参考自 PDF） | 翻老型号手册 |
| [MODEM_BAND_SPECS.md](MODEM_BAND_SPECS.md) | 型号硬件频段参考（**当前未在代码中使用**，仅资料） | 翻规格 |
| [BUILD.md](BUILD.md) | macOS / Windows 构建、版本管理、发版流程、常见问题 | 发布时查 |
| [TECH_STACK.md](TECH_STACK.md) | 技术栈、依赖、模块依赖图 | 新人入门 |
| [CODING.md](CODING.md) | AT 输入校验、敏感信息、前端入口、文档同步规范 | 改代码前查 |
| [REVIEW.md](REVIEW.md) | 代码 review 报告（20 条可操作发现，按 P0/P1/P2 排序） | 排期修 bug 时查 |

## 文档维护原则

1. **代码与文档不一致时，以代码为准。**
2. **改代码必同步改文档。** ARCHITECTURE.md / CODE_MAP.md / CALL_FLOW.md 每次结构变更后必须 review。
3. **REVIEW.md 是活跃任务清单。** P1/P2 条目可直接排期，已修复项归档到附录。
4. **文档禁含 commit hash / 精确行号 / 已删除文件历史。** 详见 AGENTS.md §6。
