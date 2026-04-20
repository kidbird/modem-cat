# 项目架构设计

## 1. 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                         用户界面层                               │
│                   src/desktop/index.html                        │
│              (单文件 HTML，含 CSS/JS 内联)                       │
└─────────────────────────────────────────────────────────────────┘
                                │
                          Tauri IPC
                                │
┌─────────────────────────────────────────────────────────────────┐
│                       Rust 后端层                                 │
│                      src-tauri/src/                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────┐ │
│  │   lib.rs    │  │at_adapter.rs│  │at_parser.rs │  │types.rs │ │
│  │  (主入口)   │  │ (AT指令)    │  │  (解析器)    │  │ (数据结构)│ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────┘ │
│  ┌─────────────┐  ┌─────────────┐                               │
│  │transport.rs │  │ network.rs  │                               │
│  │ (通信层)     │  │ (TCP传输)   │                               │
│  └─────────────┘  └─────────────┘                               │
└─────────────────────────────────────────────────────────────────┘
                                │
                    ┌──────────┴──────────┐
                    │    物理通信层         │
                    │ Serial Port / TCP    │
                    │    5G 模组           │
                    └─────────────────────┘
```

## 2. 模块划分

### 2.1 前端模块 (`src/desktop/`)
- **index.html**: 唯一的桌面应用前端文件，包含所有 HTML、CSS、JavaScript

### 2.2 Rust 后端模块 (`src-tauri/src/`)
| 文件 | 职责 |
|------|------|
| `lib.rs` | 主入口，Tauri 命令定义，系统托盘初始化，窗口事件处理 |
| `at_adapter.rs` | AT 命令执行适配器，封装所有对模组的 AT 操作 |
| `at_parser.rs` | AT 响应解析器，将原始响应转换为结构化数据 |
| `transport.rs` | 通信传输层，支持 Serial 和 TCP 两种传输方式 |
| `types.rs` | 数据结构定义，所有通过 IPC 传输的数据结构 |
| `network.rs` | 网络相关（占位，当前 TCP 功能在 transport.rs 中） |
| `serial.rs` | 串口相关（占位，逻辑已合并到 lib.rs） |

## 3. 状态管理

```rust
pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,  // AT传输层实例
    pub data_cid: Arc<Mutex<i32>>,                            // 当前 PDP 上下文 ID
    pub timing: Arc<Mutex<TimingTracker>>,                   // AT 指令耗时追踪
    pub active_cids: Arc<Mutex<Vec<i32>>>,                   // 已激活的 CID 列表
}
```

## 4. IPC 命令映射

所有前端 `invoke()` 调用的命令在 `lib.rs` 中定义：

| 命令 | 函数 | 功能 |
|------|------|------|
| `list_ports` | `list_ports()` | 列出所有可用串口 |
| `auto_connect_at` | `auto_connect_at()` | 自动扫描并连接 AT 端口 |
| `connect_serial` | `connect_serial()` | 通过串口连接 |
| `connect_tcp` | `connect_tcp()` | 通过 TCP 连接 |
| `disconnect` | `disconnect()` | 断开连接 |
| `get_modem_status` | `get_modem_status()` | 获取模组状态 |
| `get_hardware_info` | `get_hardware_info()` | 获取硬件信息 |
| `get_ip_info` | `get_ip_info()` | 获取 IP 信息 |
| `get_apn_list` | `get_apn_list()` | 获取 APN 列表 |
| `get_neighbor_cells` | `get_neighbor_cells()` | 获取邻区信息 |
| `get_qos_info` | `get_qos_info()` | 获取 QoS 信息 |
| `get_network_mode` | `get_network_mode()` | 获取网络模式 |
| `get_bands` | `get_bands()` | 获取频段配置 |
| `get_feature_toggles` | `get_feature_toggles()` | 获取功能开关 |
| `get_usbnet_mode` | `get_usbnet_mode()` | 获取 USB 网卡模式 |
| `get_traffic` | `get_traffic()` | 获取流量统计 |
| `set_apn_config` | `set_apn_config()` | 设置 APN |
| `delete_apn_config` | `delete_apn_config()` | 删除 APN |
| `connect_data` | `connect_data()` | 连接数据 |
| `disconnect_data` | `disconnect_data()` | 断开数据 |
| `set_network_mode_cmd` | `set_network_mode_cmd()` | 设置网络模式 |
| `set_bands` | `set_bands()` | 设置频段 |
| `reset_all_bands` | `reset_all_bands()` | 重置频段 |
| `set_feature_toggle` | `set_feature_toggle()` | 设置功能开关 |
| `set_usbnet_mode` | `set_usbnet_mode()` | 设置 USB 网卡模式 |
| `reboot_modem` | `reboot_modem()` | 重启模组 |
| `factory_reset` | `factory_reset()` | 恢复出厂设置 |
| `send_raw_at` | `send_raw_at()` | 发送原始 AT 命令 |

## 5. 系统托盘架构

```
┌──────────────────────────────────────┐
│          System Tray                 │
│  ┌────────────────────────────────┐ │
│  │ Icon: icon.ico                 │ │
│  └────────────────────────────────┘ │
│              │                       │
│              ▼                       │
│  ┌────────────────────────────────┐ │
│  │ Context Menu                    │ │
│  │ ├── 控制面板 (show_window)      │ │
│  │ └── 退出 (quit)                 │ │
│  └────────────────────────────────┘ │
└──────────────────────────────────────┘

窗口关闭行为:
- 点击关闭按钮 → window.hide() → 窗口隐藏到托盘
- 右键托盘菜单 "退出" → app.exit(0) → 真正退出
- 左键点击托盘图标 → 显示并聚焦窗口
```

## 6. 自动连接流程

```
应用启动
    │
    ▼
doInit()
    │
    ├─ refreshPortList()      // 扫描串口
    │
    └─ toggleConnection()
            │
            ├─ auto_connect_at()
            │     ├─ list_ports()          // 获取所有端口
            │     ├─ 扫描 AT 候选端口
            │     ├─ 逐个探测: AT\r\n
            │     └─ 返回成功连接的端口
            │
            └─ refreshAll()
                  ├─ refreshModemStatus()  // 网络状态
                  ├─ refreshIpInfo()       // IP 信息
                  ├─ refreshApnList()      // APN 列表
                  ├─ refreshQos()           // QoS
                  └─ refreshTraffic()      // 流量
```