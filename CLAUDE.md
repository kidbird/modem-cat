# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> 最近更新：2026-06-01
> 详细文档见 [docs/](docs/)，尤其是 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)、[docs/CODE_MAP.md](docs/CODE_MAP.md)、[docs/CALL_FLOW.md](docs/CALL_FLOW.md)、[docs/REVIEW.md](docs/REVIEW.md)。

## Project Overview

modem-cat 是 5G 模组调试桌面工具。Tauri 2.x 桌面 app，Rust 后端 + 拆分的 HTML/CSS/JS 前端。
核心模组逻辑在 `modem-hal/`，是独立 Rust crate，被 Tauri 后端直接消费。

## Commands

```bash
# Desktop app (Tauri)
cd src-tauri && cargo build --release

# Full workspace (both crates)
cargo build --workspace
cargo test --workspace

# Run tests for a single crate
cargo test -p modem-hal
cargo test -p modem-cat

# Run a specific test
cargo test -p modem-hal parse_qeng_serving_cell
```

## Windows Build Note

**Tauri app 必须完全退出再 rebuild。** 运行的 `.exe` 持有文件锁，阻塞 `cargo build`。
App 关闭时最小化到系统托盘 — 右键托盘图标选"退出"才是真正退出。

## Architecture（速览）

```
src/desktop/{index.html, app.js, styles.css}    ← 前端（已拆 3 文件，不是单文件）
        │
     Tauri IPC (invoke + listen)
        │
src-tauri/src/
   lib.rs       ← 1142 行: Tauri Builder 装配 + AppState + LoggingTransport + **52 个 IPC**（invoke_handler 注册）+ with_vendor! 宏
   commands.rs  ← 504 行死代码 (REVIEW.md#1)：30 个 #[tauri::command] + 64 处 .unwrap()，编译进 binary 但 0 caller
   ports.rs     ← 串口列表 + Windows 注册表友好名
   monitor.rs   ← start_port_monitor 后台线程
   main.rs      ← 入口
        │
modem-hal/src/                                  ← 共享 HAL crate
   modem_vendor.rs    ← ModemVendor trait (**62 个方法**)
   modem_factory.rs   ← ModemFactory::create() — AT+CGMM 型号检测
   types.rs           ← 共享数据结构
   transport/{mod,serial,tcp}.rs  ← AtTransport trait + 实现
   vendors/quectel/   ← QuectelModem (chip: Qualcomm/UniSoc 二态)
                       + qualcomm.rs / unisoc.rs / parser.rs / band_db.rs
   vendors/tdtech/    ← TdTechModem (AT^ 前缀) + parser.rs / dial.rs
        │
   serialport / TCP → 5G Modem
```

### 关键运行模式

- **with_vendor! 宏**（lib.rs:64）：所有 IPC handler 都用它消除 lock/spawn_blocking 样板
- **LoggingTransport 装饰器**（lib.rs:33）：包装真实 transport，旁路记录 1000 条 AT 日志
- **start_port_monitor**（lib.rs:869 **与** monitor.rs:13 重复，REVIEW 待清理）
- **start_connection_heartbeat**（lib.rs:929）：4s 间隔，硬件拔插通过 `port-changed` 事件通知前端

### Frontend（`src/desktop/`）

- **已拆 3 文件**：`index.html` (319KB, 6479 行) + `app.js` (62KB, 1556 行) + `styles.css` (19KB)
- 无前端框架；8 个 page 容器（`#page-status` / `#page-cellular` / `#page-ip` / `#page-at` / `#page-hardware` / `#page-scene` / `#page-atmanual` / `#page-settings`）
- 单一全局 `state` 对象（位于 `app.js` 顶部全局 `let state = { ... }`，具体行随 commit 漂移；**勿引用具体行号**，以"全局 state 对象"为锚点）
- `$.dom` 在 `cacheDom()` 函数中（`app.js` 启动段；同上行号漂移）
- **前端 invoke 调用**：实际入口 `index.html` 44 个唯一名字；同步副本 `app.js` 32 个唯一名字。
- 通过 `window.__TAURI__.core.invoke()` 调用后端，监听 `port-changed` + `show-about` 事件

## Vendor Detection（型号 → 厂商）

`ModemFactory::detect_vendor_from_model()`（`modem-hal/src/modem_factory.rs:41`）按优先级匹配：

| 优先级 | 厂商 | 关键字 |
|---|---|---|
| 1 | TdTech | `MT5700` |
| 2 | Qualcomm | `RG500Q`, `RM500Q`, `RG520N`, `RM520N`, `RG525F`, `RG530F`, `RM530F`, `RM530N`, `RM551E`, `RM501Q`, `RG540F`, `RM540N` |
| 3 | UniSoc | `RG200U`, `RM500U`, `RG500U`, `RG501U`, `RM501U` |
| 4 | **Unknown** | （兜底，**无默认 adapter**，直接 `Err`） |

详见 [docs/AT_COMMANDS.md](docs/AT_COMMANDS.md) 顶部表格。

## 已知问题（P0 修，本周内）

详见 [docs/REVIEW.md](docs/REVIEW.md) 的 P0 清单（5 条 HIGH）：

1. **`commands.rs` 是死代码**（504 行，30 个 IPC，64 处 `.unwrap()`，编译进 binary，应删除）
2. **`send_raw_at` 必须使用 `validate_raw_at_command`**（完整 AT 校验，不能误用参数校验）
3. **`redact_at_command` 覆盖不全**（APN 密码 / PCO 凭据未 redact）
4. **`set_plmn_lock` / `clear_plmn_lock` 必须由用户传密码并校验**（禁止硬编码默认密码）
5. **heartbeat 与 IPC 争同一 std Mutex**（USB 拔插感知延迟 4-12s）

## Key Conventions

- **AT parser 函数是 pure 的**：取 `&str` 入参，返回解析后 struct，无 IO —— 容易单测
- **`ModemVendor` trait 是业务级抽象**：命令构造完全是 vendor 内部实现细节
- **加新 vendor**：实现 `ModemVendor` trait + 加 `modem_factory.rs` 检测分支 + 在 `vendors/<name>/` 新建模块
- **加新 IPC 命令**：在 `lib.rs` 加 `#[tauri::command]` + 用 `with_vendor!` 宏锁 transport/vendor
- **所有 Rust 测试都是 pure 单测**（无 IO / 无硬件）
- **错误处理**：`Result<T, String>` 全程，**未引入 thiserror**（REVIEW.md#17）

## Tauri v2 Specifics

- 使用 `tauri::Manager` trait 取 `webview_windows()` / `tray_by_id()`
- 系统托盘在 `tauri.conf.json` 的 `app.trayIcon`；菜单用 `tauri::menu::MenuBuilder` 程序化构建
- 窗口关闭通过 `.on_window_event()` 拦截 → `hide()` 到托盘
- 自定义 `modemcat://` URI scheme 协议服务嵌入式 `index.html`
- `winreg` crate 仅 Windows target（`cfg(target_os = "windows")`）拿串口友好名
- `withGlobalTauri: true` —— 前端直接用 `window.__TAURI__.core.invoke`
