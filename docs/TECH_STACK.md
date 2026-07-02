# 技术栈

> 最近更新：2026-07-02

## 1. 核心技术

### 1.1 桌面框架

- **Tauri 2**
  - 前端通过 `window.__TAURI__.core.invoke` / `event.listen` 与后端通信
  - `src-tauri/src/lib.rs` 负责装配；实际执行面分散在 `handlers.rs`、`connection.rs`、`monitor.rs`、`license.rs`、`factory.rs`、`dloader.rs`

### 1.2 前端

- **HTML + CSS + plain JavaScript**
- **无框架、无 bundler、无 ES Modules**
- 当前采用顺序 `<script src>` 分层加载：
  - `src/desktop/js/core.js`
  - `src/desktop/js/i18n.js`
  - `src/desktop/js/theme.js`
  - `src/desktop/js/scene.js`
  - `src/desktop/data/atdb.js`
  - `src/desktop/js/debug-terminal.js`
  - `src/desktop/app.js`
- 全局状态 owner：`state`

### 1.3 后端

- **Rust 2021**
- **Tokio**
  - IPC handler 以 async 暴露，阻塞 I/O 通过 `tokio::task::spawn_blocking` 下沉
- **serialport**
  - 串口 transport
- **rumqttc**
  - 可选 MQTT 后台上报
  - broker / port / 认证信息必须显式提供，不能硬编码默认生产值
- **reqwest**
  - 工厂模式设备 HTTP 通信
- **ssh2**
  - SSH 调试终端会话
- **chrono**
  - SN / 记录 / 时间处理
- **winreg**
  - Windows 端口友好名解析

### 1.4 HAL

- `modem-hal`
  - `AtTransport` trait
  - `SerialTransport` / `TcpTransport` / `WebSocketTransport`
  - `ModemFactory`
  - `ModemVendor`
  - `vendors/quectel/*`

### 1.5 辅助模块

- `modem-license`
  - License 校验与状态建模（当前最终用户桌面构建不暴露其 IPC）
- `r26-cli` sidecar
  - PAC 解析与固件下载执行
  - 由 `src-tauri/src/dloader.rs` 管理
  - 真正刷机能力当前只随 Windows 二进制交付；非 Windows 目标保留占位 sidecar 以保证本地构建 / 测试可通过
- `adb.exe` + `AdbWin*.dll`
  - Windows ADB 调试 sidecar 资源
  - 发布脚本从 `Sdk/` 自动同步到 `src-tauri/resources/adb/`
  - 由 `src-tauri/src/debug_terminal.rs` 从 `src-tauri/resources/adb/` 打包目录解析

## 2. Live 依赖关系

```text
src/desktop/*
  → invoke / listen
src-tauri/src/lib.rs
  → AppState.transport / vendor / license
src-tauri/src/debug_terminal.rs
  → adb.exe sidecar / ssh2 shell
modem-hal/src/*
  → AtTransport / ModemVendor
serial / tcp / websocket transport
  → 5G modem / gateway / tcp peer

src-tauri/src/factory.rs
  → reqwest / HTTP device APIs

src-tauri/src/dloader.rs
  → r26-cli sidecar / DLFrame.dll
```

## 3. 运行方式

- 当前主产品形态是 **Tauri 桌面应用**
- backend 支持的 transport 形态包括：
  - serial
  - tcp
  - websocket
- 最终用户调试能力额外包含：
  - Windows ADB shell
  - SSH shell
- WebSocket 网关允许匿名接入；若目标网关要求认证，用户名/密码必须由用户显式提供，禁止补默认值
- `mqtt.rs` 是可选后台上报模块，不是第二条业务主路径
- `debug_terminal.rs` 和 `dloader.rs` 是辅助业务模块，不得绕过 HAL 新建 AT 队列
- MQTT 仅在显式设置 `MODEM_CAT_MQTT_HOST`、`MODEM_CAT_MQTT_PORT`，以及可选的 `MODEM_CAT_MQTT_USERNAME` / `MODEM_CAT_MQTT_PASSWORD` 后才允许启用

## 4. 设计约束与技术栈关系

- 由于前端没有 bundler，文档和代码都必须按“顺序脚本加载”设计，不要照搬 Vue / Vite / ES Module 项目的约定
- 由于 modem I/O 通过 Rust HAL 串行化，任何并发设计都必须围绕**单一 AT 队列**展开，而不是增加 fallback 路径
- 由于 transport 抽象已经支持 serial / tcp / websocket，新功能应复用 `AtTransport` 边界，不要旁路直接发命令
- 由于 Factory / Firmware 属于辅助路径，它们可以使用 HTTP / sidecar，但不能反向污染 live modem 状态 owner

## 5. 构建与测试入口

- `cargo test --workspace`
- `cargo build -p modem-hal`
- 完整桌面构建与平台说明见 `docs/BUILD.md`
