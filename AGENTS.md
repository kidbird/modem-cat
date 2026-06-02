# AGENTS.md

This file provides guidance to **CodeBuddy Code** when working with code in this repository.

> 最近更新：2026-06-01
> 详细文档见 [docs/](docs/) 目录。优先看 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) 和 [docs/REVIEW.md](docs/REVIEW.md)。

## Project Overview

modem-cat 是 5G 模组调试桌面工具。Tauri 2.x 桌面 app，Rust 后端 + 拆分的 HTML/CSS/JS 前端。
核心模组逻辑在 `modem-hal/`，独立 Rust crate，被 Tauri 后端直接消费。

## Build Commands

```bash
# Desktop app (Tauri)
cd src-tauri && cargo build --release

# Full workspace (both crates)
cargo build --workspace

# Run all Rust tests
cargo test --workspace
```

## Critical: Exe Lock on Rebuild

**Tauri app 必须完全退出再 rebuild。** Windows 上 `.exe` 持有文件锁阻塞 `cargo build`。
App 关闭时最小化到系统托盘 — 右键托盘图标选"退出"才是真正退出。

## Architecture

```
src/desktop/{index.html, app.js, styles.css}    ← 前端（已拆 3 文件）
        │
     Tauri IPC (invoke + listen)
        │
src-tauri/src/
   lib.rs       ← Tauri Builder 装配 + AppState + LoggingTransport + 52 IPC（invoke_handler 注册）+ with_vendor! 宏 + start_port_monitor
   ports.rs     ← 串口列表 + Windows 注册表
   main.rs      ← 入口

> 历史：`commands.rs`（504 行死代码）与 `monitor.rs`（孤儿重复版 start_port_monitor）均已删除。
        │
modem-hal/src/                                  ← 共享 HAL crate
        │
   serialport / TCP → 5G Modem
```

### Frontend（`src/desktop/`）

- **已拆 3 文件**：`index.html` (319KB) + `app.js` (62KB) + `styles.css` (19KB)
- 无前端框架；8 个 page 容器
- 单一全局 `state` 对象
- 通过 `window.__TAURI__.core.invoke()` 调用后端

### Tauri Backend（`src-tauri/src/`）

- `lib.rs` — 全部 52 个 `#[tauri::command]` handlers（invoke_handler! 注册）+ `AppState`（transport, vendor, data_cid, connected_port, at_command_log）+ `LoggingTransport` 装饰器
- `ports.rs` — 串口列表探测
- `commands.rs` / `monitor.rs` — 历史孤儿文件，均已删除

### modem-hal（`modem-hal/src/`）

- `modem_vendor.rs` — `ModemVendor` trait（**62 个方法**，vendor 无关接口；`grep -cE "^    fn " modem-hal/src/modem_vendor.rs` 区间内）
- `modem_factory.rs` — `ModemFactory::create()` 检测厂商
- `types.rs` — 共享数据 + `ChipsetVendor` 枚举
- `transport/mod.rs` — `AtTransport` trait + `MockTransport`
- `transport/serial.rs` — `SerialTransport` (serialport v4)
- `transport/tcp.rs` — `TcpTransport`
- `vendors/quectel/` — Quectel: `mod.rs` (chip 二态) + `parser.rs` + `qualcomm.rs` + `unisoc.rs` + `band_db.rs`
- `vendors/tdtech/` — TdTech MT5700M-CN: `mod.rs` (AT^) + `parser.rs` + `dial.rs`
- Feature flags: `serial` (default), `napi-feature` (napi-rs ModemHandle，目前未启用)

## Vendor Detection

`ModemFactory::create()` 按型号检测（优先级从高到低）：

| 优先级 | 厂商 | 关键字 |
|---|---|---|
| 1 | TdTech | `MT5700` |
| 2 | Qualcomm | `RG500Q`, `RM500Q`, `RG520N`, `RM520N`, `RG525F`, `RG530F`, `RM530F`, `RM530N`, `RM551E`, `RM501Q`, `RG540F`, `RM540N` |
| 3 | UniSoc | `RG200U`, `RM500U`, `RG500U`, `RG501U`, `RM501U` |
| 4 | **Unknown** | 兜底，无默认 adapter |

## Desktop App Behavior

- **Close button**: 隐藏到系统托盘（不退出）。右键托盘图标出菜单。
- **Tray menu**: "控制面板" (显示窗口) / "退出" (退出 app)
- **Auto-connect**: 启动时自动扫描端口并连接 AT 端口

## Tauri IPC Commands（lib.rs 注册 52 个，commands.rs 死代码 30 个，index.html 实际 invoke 44 个，app.js 同步副本 invoke 32 个）

> 完整清单见 [docs/CODE_MAP.md](docs/CODE_MAP.md) §1。

按类别分组：
- **连接**：`list_ports`, `auto_connect_at`, `connect_serial`, `connect_tcp`, `disconnect`
- **状态查询**：`get_modem_status`, `get_hardware_info`, `get_ip_info`, `get_traffic`, `get_apn_list`, `get_neighbor_cells`, `get_bands`, `get_feature_toggles`, `get_network_mode`, `get_usbnet_mode`, `get_5glan`, `get_vlan`, `get_sim_slot`, `get_qos_info`, `get_qualcomm_config`, `get_nat_mode`
- **配置写入**：`set_apn_config`, `delete_apn_config`, `set_apn_active`, `set_network_mode_cmd`, `set_bands`, `reset_all_bands`, `set_feature_toggle`, `set_usbnet_mode`, `set_5glan`, `set_sim_slot`, `set_vlan`, `set_qualcomm_config`, `set_nat_mode`, `set_cfun`, `set_plmn_lock`, `clear_plmn_lock`, `set_cell_lock`, `clear_cell_lock`, `query_cell_lock`
- **数据连接**：`connect_data`, `disconnect_data`
- **5GLAN (Qualcomm)**：`configure_qualcomm_5glan`, `enable_eth_pdu`, `connect_qualcomm_5glan`, `query_qualcomm_5glan_status`
- **诊断**：`send_raw_at`, `pop_at_commands`, `reboot_modem`, `factory_reset`, `get_app_version`

## Platform Notes

- Windows 构建脚本 (`build-win.bat`) 用 `vswhere.exe` 自动定位 MSVC
- `winreg` crate 仅 Windows（`cfg(target_os = "windows")`）
- `serialport` crate v4
- Rust workspace: members 是 `modem-hal` 和 `src-tauri`（根 `Cargo.toml`）

## Tauri v2 Specifics

- 使用 `tauri::Manager` trait
- 系统托盘在 `tauri.conf.json` 的 `app.trayIcon`
- 托盘菜单用 `tauri::menu::MenuBuilder` 程序化构建
- 窗口关闭通过 `.on_window_event()` 拦截

## Key Conventions

- AT parser 函数 pure：取 `&str` 入参，返回 struct，无 IO
- `ModemVendor` trait 是业务级抽象
- 加新 vendor：实现 trait + 加 `modem_factory.rs` 检测分支
- 加新 IPC：在 `lib.rs` 加 `#[tauri::command]` + 用 `with_vendor!` 宏
- 前端无构建工具 — 直接编辑 `index.html` / `app.js` / `styles.css`
- 所有 Rust 测试是 pure 单测（无 IO / 无硬件）
- 错误处理：`Result<T, String>` 全程

## 已知 P0 问题

详见 [docs/REVIEW.md](docs/REVIEW.md)。本周应修 5 条 HIGH：

1. 删除 `commands.rs` 死代码
2. `send_raw_at` 使用 `validate_raw_at_command` 校验完整 AT 命令
3. `redact_at_command` 补全敏感字段
4. `set_plmn_lock` / `clear_plmn_lock` 移除硬编码默认密码并校验用户输入密码
5. heartbeat 改 atomic / try_lock 避免阻塞
