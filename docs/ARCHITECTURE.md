# 项目架构设计

## 1. 整体架构（Desktop Only）

```
┌───────────────────────────────────────────────┐
│ 前端: src/desktop/index.html                 │
│ (单文件 HTML/CSS/JS)                          │
└───────────────────────────────────────────────┘
                     │
                  Tauri IPC
                     │
┌───────────────────────────────────────────────┐
│ 后端: src-tauri/src/                         │
│ - lib.rs (命令入口/状态/托盘)                 │
│ - at_adapter.rs (AT 命令读写业务)             │
│ - at_parser.rs (AT 响应解析)                  │
└───────────────────────────────────────────────┘
                     │
┌───────────────────────────────────────────────┐
│ 共享 HAL: modem-hal/                          │
│ - transport/ (串口/TCP 传输抽象)              │
│ - modem_factory.rs (厂商/型号识别)            │
│ - vendors/ (Quectel/TdTech 适配)              │
└───────────────────────────────────────────────┘
                     │
                 5G Modem
```

## 2. 模块划分

### 2.1 前端模块 (`src/desktop/`)
- `index.html`: 桌面应用唯一前端文件

### 2.2 Tauri 后端模块 (`src-tauri/src/`)
- `lib.rs`: Tauri command 定义、连接状态、托盘与窗口行为
- `at_adapter.rs`: 查询/配置类 AT 业务能力（状态、IP、APN、频段、功能开关等）
- `at_parser.rs`: 响应解析函数集合

### 2.3 HAL 模块 (`modem-hal/src/`)
- `transport/`: `AtTransport` trait 与 Serial/TCP 实现
- `modem_factory.rs`: 按型号识别芯片/厂商并构造对应 modem adapter
- `vendors/`: 厂商实现与解析逻辑
- `types.rs`: 共享数据结构

## 3. 状态管理（Tauri）

`AppState` 位于 `src-tauri/src/lib.rs`，核心状态包括：
- `transport`: 当前 AT 传输对象
- `data_cid`: 当前数据拨号 CID
- `connected_port`: 当前已连接串口名（用于断连/热插拔判断）

## 4. IPC 命令边界

所有前端 `invoke()` 命令在 `src-tauri/src/lib.rs` 暴露。
命令分为四类：
- 连接管理：`list_ports` / `auto_connect_at` / `connect_serial` / `connect_tcp` / `disconnect`
- 状态查询：`get_modem_status` / `get_hardware_info` / `get_ip_info` / `get_traffic` / ...
- 配置写入：`set_apn_config` / `set_network_mode_cmd` / `set_bands` / `set_feature_toggle` / ...
- 诊断调试：`send_raw_at`

## 5. 运行模式说明

- 当前项目仅保留 **Desktop 模式**。
- CLI 相关实现与构建入口已移除。
