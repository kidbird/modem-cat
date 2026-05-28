# Code Map

## 前端 HTML 元素到后端代码映射

### 1. 页面结构

| UI 区域 | HTML ID | 对应函数/逻辑 |
|---------|---------|--------------|
| 左侧导航 | `.nav-item` | `switchCellularTab()`, `loadHardwarePage()` |
| 模组状态页面 | `#page-status` | `refreshModemStatus()` |
| 蜂窝网络页面 | `#page-cellular` | `switchCellularTab()` |
| AT调试页面 | `#page-at` | `sendAtCommand()` |
| 硬件信息页面 | `#page-hardware` | `loadHardwarePage()` |
| IP信息页面 | `#page-ip` | `refreshIpInfo()` |

### 2. 模组状态页面

| 显示元素 | HTML ID | 数据来源 AT 指令 |
|---------|---------|-----------------|
| SIM状态 | `#simStatus` | `AT+CPIN?` → `query_modem_status()` |
| 注册状态 | `#regStatus` | `AT+QENG="servingcell"` → `parse_qeng_serving_cell()` |
| 连接状态 | `#connStatus` | `AT+CGACT?` → `parse_cgact()` |
| IMEI | `#imei` | `AT+CGSN` → `parse_cgsn()` |
| ICCID | `#iccid` | `AT+CCID`/`AT+ICCID`/`AT+QCCID` → `parse_iccid()` |
| 运营商 | `#operator` | `AT+COPS?` → `parse_cops_with_act()` |
| 网络类型/PCI/Cell ID/ARFCN/频宽 | 各自 ID | `AT+QENG="servingcell"` → `parse_qeng_serving_cell()` |
| RSRP/RSRQ/SINR | 各自 ID | 同上，`ServingCellInfo` 字段 |
| 天线ANT0-3 | `#ant0`~`#ant3` | Qualcomm: `AT+QRSRP` → `parse_qrsrp()` / UniSoc: `AT+QANTRSSI?` → `parse_qantrssi()` |
| 上/下行带宽 | `#ulBandwidth`, `#dlBandwidth` | `AT+C5GQOSRDP=<cid>` → `parse_c5gqosrdp()` |
| 上/下行流量 | `#ulTraffic`, `#dlTraffic` | Qualcomm: `AT+QGDNRCNT?` / UniSoc: `AT+QGDCNT?` |
| 射频开关 | `#rfToggle` | `AT+CFUN?` 读取 / `AT+CFUN=0/1` 设置 |

### 3. 蜂窝网络页面

| 显示元素 | HTML ID | 数据来源 |
|---------|---------|---------|
| APN列表 | `#apnList` | `invoke('get_apn_list')` → `AT+QICSGP?` / `AT+CGDCONT?` |
| 网络模式 | `#preferredNetwork` | `invoke('get_network_mode')` → `AT+QNWPREFCFG="mode_pref"` |
| LTE/NR频段网格 | `#bandGridLte`, `#bandGridNr` | `invoke('get_bands')` → `AT+QNWPREFCFG=?/lte_band/nr5g_band` |
| LTE邻区/NR邻区表格 | `#lteNeighborBody`, `#nrNeighborBody` | `invoke('get_neighbor_cells')` → `AT+QENG="neighbourcell"` |
| 小区锁定列表 | `#lockList` | `invoke('query_cell_lock')` → `AT+QNWLOCK/QNWLOCKFREQ` |

### 4. AT调试页面

| 显示元素 | HTML ID | 对应函数 |
|---------|---------|---------|
| 终端输出 | `#terminal` | `addTerminalLine()` |
| 命令输入 | `#atCommand` | `sendAtCommand()` |
| 发送按钮 | `onclick(sendAtCommand)` | `invoke('send_raw_at')` |

### 5. 硬件信息页面

| 显示元素 | HTML ID | 数据来源 AT 指令 |
|---------|---------|-----------------|
| 模组型号/厂家/固件 | `#hwModel` 等 | `AT+CGMM`, `AT+CGMI`, `AT+GMR` |
| AP/CP基线 | `#hwApBaseline`, `#hwCpBaseline` | `AT+QBASELINE` |
| SOC/PA温度 | `#hwSocTemp`, `#hwPaTemp` | `AT+QTEMP` |
| PCIe/Ethernet 等开关 | `#togglePcie` 等 | `invoke('get_feature_toggles')` → `AT+QCFG="..."`  |
| 高通专属配置面板 | `#hwtab-qualcomm` | `invoke('get_qualcomm_config')` (仅 Qualcomm 显示) |

### 6. IP信息页面

| 显示元素 | HTML ID | 数据来源 AT 指令 |
|---------|---------|-----------------|
| IPv4/IPv6 地址/掩码/网关/DNS | 各自 ID | Qualcomm: `AT+QMAP="WWAN"` / UniSoc: `AT+QNETDEVSTATUS=<cid>` |
| LAN IP 配置 | `#lanGateway` 等 | `AT+QCFG="lanip_ex"` |
| Proxy ARP 开关 | `#toggleProxyArp` | `invoke('get_feature_toggles')` |

### 7. 系统状态栏

| 显示元素 | HTML ID | 对应数据 |
|---------|---------|---------|
| 模组连接状态 | `#statusLabel` | `state.connected` |
| 数据连接状态 | `#dataStatusLabel` | `state.dataConnected` |
| 射频状态 | `#rfToggle` | `state.rfEnabled` |
| 数据连接按钮 | `#dataConnectBtn` | `toggleDataConnection()` |

---

## 后端代码位置

### src-tauri/src/lib.rs — Tauri 命令层

| 函数 | 功能 |
|------|------|
| `AppState` | transport + vendor + data_cid + connected_port + at_command_log |
| `LoggingTransport` | 装饰器：记录所有 AT 命令到日志环形缓冲 |
| `get_windows_all_port_info()` | Windows 注册表读取串口友好名称 |
| `list_ports()` | 列出所有串口（含 WMI 友好名称） |
| `is_at_port()` | 判断是否为 AT 端口（描述关键字 + 厂商白名单） |
| `auto_connect_at()` | 自动扫描所有 AT 候选端口并连接 |
| `connect_serial()` / `connect_tcp()` | 手动连接，vendor 检测失败则整体失败 |
| `disconnect()` | 断开连接，清空 transport/vendor/connected_port |
| `start_port_monitor()` | 后台线程每 2s 轮询端口列表，发 `port-changed` 事件 |
| `start_connection_heartbeat()` | 后台线程每 4s 检查 `is_alive()`，硬件拔出时发 `port-changed` |
| `send_raw_at()` | 原始 AT 终端，绕过 vendor 直接发命令 |
| `pop_at_commands()` | 取出并清空内部 AT 日志（前端轮询） |

### modem-hal/src/ — HAL 核心

| 文件 | 职责 |
|------|------|
| `modem_factory.rs` | `ModemFactory::create()` — AT+CGMM 检测型号 → 创建 vendor 实例 |
| `modem_vendor.rs` | `ModemVendor` trait — 28+ 方法，vendor 无关接口 |
| `types.rs` | 所有共享数据结构（ModemStatus, IpInfo, BandConfig 等）+ `spec_bands_for_model()` |
| `transport/mod.rs` | `AtTransport` trait + `MockTransport` |
| `transport/serial.rs` | `SerialTransport` — serialport v4，`is_alive()` 用 `bytes_to_read()` 检测硬件 |
| `transport/tcp.rs` | `TcpTransport` |

### modem-hal/src/vendors/quectel/ — Quectel 实现

| 文件 | 职责 |
|------|------|
| `mod.rs` | `QuectelModem` struct + `QuectelChip` enum（Qualcomm/UniSoc），实现 `ModemVendor` trait |
| `parser.rs` | 所有 AT 响应解析纯函数（`parse_qeng_serving_cell`, `parse_qantrssi`, `is_ok` 等） |
| `qualcomm.rs` | 高通平台数据连接：`AT+QMAP`，IP：`AT+QMAP="WWAN"`，流量：`AT+QGDNRCNT?` |
| `unisoc.rs` | 展锐平台数据连接：`AT+QNETDEVCTL`，IP：`AT+QNETDEVSTATUS`，流量：`AT+QGDCNT?` |

### modem-hal/src/vendors/tdtech/ — 鼎桥 MT5700 实现

| 文件 | 职责 |
|------|------|
| `mod.rs` | `TdTechModem` struct，实现 `ModemVendor` trait，`AT^` 前缀命令 |
| `parser.rs` | `parse_monsc`, `parse_hcsq`, `decode_syscfgex_lteband` 等 |
| `dial.rs` | 数据拨号连接/断开/IP 查询 |
