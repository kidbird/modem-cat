# 技术栈

## 1. 核心技术

### 1.1 桌面应用框架
- **Tauri v2.10.3**: Rust 编写的桌面应用框架
  - 特性: `custom-protocol`, `tray-icon`
  - 窗口管理、系统托盘、IPC 通信

### 1.2 前端技术
- **HTML5 + CSS3 + Vanilla JavaScript**: 单文件前端，无框架依赖
  - CSS 变量实现主题切换（亮/暗）
  - Flexbox/Grid 布局
  - 内联 SVG 图标

### 1.3 后端技术
- **Rust 2021 Edition**: 系统级编程
- **Tokio**: 异步运行时
- **Serialport v4**: 串口通信
- **Winreg**: Windows 注册表访问（获取串口友好名称）

## 2. 依赖关系

```
┌─────────────────────────────────────────────────────────────┐
│                         前端 (HTML/JS)                      │
│                     @tauri-apps/api v2                      │
└──────────────────────────┬──────────────────────────────────┘
                           │ IPC invoke()
┌──────────────────────────▼──────────────────────────────────┐
│                    Tauri Commands (Rust)                    │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ lib.rs: 命令入口，状态管理，系统托盘                    │   │
│  └─────────────────────────┬────────────────────────────┘   │
│                            │                                 │
│  ┌─────────────────────────▼────────────────────────────┐   │
│  │ at_adapter.rs: AT命令执行层                           │   │
│  │ - query_modem_status()    → AT+CPIN?, AT+CGSN, AT+QENG│   │
│  │ - query_hardware_info()   → ATI, AT+QBASELINE, AT+QTEMP│  │
│  │ - query_ip_info()         → AT+QNETDEVSTATUS          │   │
│  │ - query_apn_list()        → AT+QICSGP?                │   │
│  │ - query_bands()           → AT+QNWPREFCFG=?           │   │
│  └─────────────────────────┬────────────────────────────┘   │
│                            │                                 │
│  ┌─────────────────────────▼────────────────────────────┐   │
│  │ at_parser.rs: 响应解析层                              │   │
│  │ - parse_cpin(), parse_cgsn(), parse_qeng_servingcell()│   │
│  │ - parse_qicsgp(), parse_qeng_neighbourcell()         │   │
│  └─────────────────────────┬────────────────────────────┘   │
│                            │                                 │
│  ┌─────────────────────────▼────────────────────────────┐   │
│  │ transport.rs: 通信传输层                              │   │
│  │ - SerialTransport (USB/TTL 串口)                     │   │
│  │ - TcpTransport (网络透传模式)                         │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │  5G Modem   │
                    └─────────────┘
```

## 3. 数据结构

详见 `types.rs`:

| 结构体 | 用途 |
|--------|------|
| `ModemStatus` | 模组状态 (SIM, 注册, 信号, 小区) |
| `HardwareInfo` | 硬件信息 (型号, 固件, 温度) |
| `IpInfo` | IP 配置信息 |
| `ApnEntry` | APN 配置条目 |
| `NeighborCells` | LTE/NR 邻区列表 |
| `BandConfig` | 频段配置 (支持/锁定) |
| `FeatureToggles` | 功能开关状态 |
| `QosInfo` | QoS 信息 |
| `TrafficInfo` | 流量统计 |
| `PortInfo` | 串口信息 |
| `AtTimingStats` | AT 指令耗时统计 |

## 4. 构建环境

### 4.1 编译工具
- **Rust 1.94.1+**: 通过 `rustup` 安装
- **MSVC Build Tools 2022**: Windows 平台编译
- **Bun 1.3.12+**: TypeScript 运行时（仅 CLI 开发）

### 4.2 构建命令

```bash
# Tauri 桌面应用
cd src-tauri && cargo build --release

# 或使用构建脚本
build-tauri.bat  # Windows 专用，设置 MSVC 环境变量

# TypeScript 类型检查
bun run typecheck
```