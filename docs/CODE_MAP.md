# Code Map

## 前端 HTML 元素到后端代码映射

### 1. 页面结构

| UI 区域 | HTML ID | 对应函数/逻辑 |
|---------|---------|--------------|
| 左侧导航 | `.nav-item` | `switchCellularTab()`, `loadHardwarePage()` [行 1439] |
| 模组状态页面 | `#page-status` | `refreshModemStatus()` [行 1574] |
| 蜂窝网络页面 | `#page-cellular` | `switchCellularTab()` [行 1721] |
| AT调试页面 | `#page-at` | `sendAtCommand()` [行 1659] |
| 硬件信息页面 | `#page-hardware` | `loadHardwarePage()` [行 1445] |
| IP信息页面 | `#page-ip` | `refreshIpInfo()` [行 2082] |

### 2. 模组状态页面

| 显示元素 | HTML ID | 数据来源 AT 指令 |
|---------|---------|-----------------|
| SIM状态 | `#simStatus` | `AT+CPIN?` → `query_modem_status()` |
| SIM槽 | `#simSlot` | 前端固定显示 "SIM 1" |
| 注册状态 | `#regStatus` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| 连接状态 | `#connStatus` | `AT+CGACT?` → `parse_cgact()` |
| IMEI | `#imei` | `AT+CGSN` → `parse_cgsn()` |
| ICCID | `#iccid` | `AT+CCID` / `AT+QCCID` → `parse_ccid()` |
| 运营商 | `#operator` | `AT+COPS?` → `parse_cops()` |
| 网络类型 | `#networkType` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| PCI | `#pci` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| Cell ID | `#cellid` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| ARFCN | `#arfcn` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| 频宽 | `#bandwidth` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| RSRP | `#rsrp` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| RSRQ | `#rsrq` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| SINR | `#sinr` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| TX Power | `#txPower` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| SCS | `#scs` | `AT+QENG="servingcell"` → `parse_qeng_servingcell()` |
| 天线ANT0-3 | `#ant0`~`#ant3` | `AT+QANTRSSI?` → `parse_qantrssi()` |
| 上行带宽 | `#ulBandwidth` | `AT+C5GQOSRDP=<cid>` → `query_qos()` |
| 下行带宽 | `#dlBandwidth` | `AT+C5GQOSRDP=<cid>` → `query_qos()` |
| 上行流量 | `#ulTraffic` | `AT+QGDCNT?` → `query_traffic()` |
| 下行流量 | `#dlTraffic` | `AT+QGDCNT?` → `query_traffic()` |
| 刷新按钮 | `.panel-header button` | `refreshModemStatus()` [行 1574] |

### 3. 蜂窝网络页面

| 显示元素 | HTML ID | 数据来源 |
|---------|---------|---------|
| APN列表 | `#apnList` | `refreshApnList()` → `invoke('get_apn_list')` |
| 网络模式选择 | `#preferredNetwork` | `loadNetlockData()` → `invoke('get_network_mode')` |
| LTE频段网格 | `#bandGridLte` | `refreshBands()` → `invoke('get_bands')` |
| NR频段网格 | `#bandGridNr` | `refreshBands()` → `invoke('get_bands')` |
| 运营商锁定 | `#lockMcc`, `#lockMnc` | 前端状态管理 |
| 小区锁定列表 | `#lockList` | 前端 `lockEntries` 数组 |
| LTE邻区表格 | `#lteNeighborBody` | `loadNeighborCells()` → `invoke('get_neighbor_cells')` |
| NR邻区表格 | `#nrNeighborBody` | `loadNeighborCells()` → `invoke('get_neighbor_cells')` |

### 4. AT调试页面

| 显示元素 | HTML ID | 对应函数 |
|---------|---------|---------|
| 终端输出 | `#terminal` | `addTerminalLine()` [行 1697] |
| 命令输入 | `#atCommand` | `sendAtCommand()` [行 1659] |
| 发送按钮 | `button:onclick(sendAtCommand)` | `invoke('send_raw_at')` |
| 清除按钮 | `button:onclick(clearTerminal)` | `clearTerminal()` [行 1715] |

### 5. 硬件信息页面

| 显示元素 | HTML ID | 数据来源 AT 指令 |
|---------|---------|-----------------|
| 模组型号 | `#hwModel` | `ATI` → `query_hardware_info()` |
| 生产厂家 | `#hwManufacturer` | `ATI` → `query_hardware_info()` |
| 固件版本 | `#hwFirmware` | `ATI` → `query_hardware_info()` |
| AP基线版本 | `#hwApBaseline` | `AT+QBASELINE` → `query_hardware_info()` |
| CP基线版本 | `#hwCpBaseline` | `AT+QBASELINE` → `query_hardware_info()` |
| SOC温度 | `#hwSocTemp` | `AT+QTEMP` → `query_hardware_info()` |
| PA温度 | `#hwPaTemp` | `AT+QTEMP` → `query_hardware_info()` |
| PCIe开关 | `#togglePcie` | `get_feature_toggles()` |
| Ethernet开关 | `#toggleEthernet` | `get_feature_toggles()` |
| ETH AT开关 | `#toggleEthAt` | `get_feature_toggles()` |
| UART AT开关 | `#toggleUartAt` | `get_feature_toggles()` |
| ADB开关 | `#toggleAdb` | `get_feature_toggles()` |
| USB网卡模式 | `#usbNetMode` | `get_usbnet_mode()` |
| 刷新按钮 | `onclick(refreshHardwareInfo)` | `invoke('get_hardware_info')` |

### 6. IP信息页面

| 显示元素 | HTML ID | 数据来源 AT 指令 |
|---------|---------|-----------------|
| IPv4地址 | `#ipv4Addr` | `AT+QNETDEVSTATUS=<cid>` → `query_ip_info()` |
| IPv4子网掩码 | `#ipv4Mask` | `AT+QNETDEVSTATUS=<cid>` → `query_ip_info()` |
| IPv4网关 | `#ipv4Gw` | `AT+QNETDEVSTATUS=<cid>` → `query_ip_info()` |
| IPv4 DNS | `#ipv4Dns` | `AT+QNETDEVSTATUS=<cid>` → `query_ip_info()` |
| IPv6地址 | `#ipv6Addr` | `AT+QNETDEVSTATUS=<cid>` → `query_ip_info()` |
| Proxy ARP开关 | `#toggleProxyArp` | `get_feature_toggles()` |
| DMZ主机 | `#dmzHost` | 前端状态管理 |
| MTU值 | `#mtuValue` | 前端状态管理 |

### 7. 系统状态栏

| 显示元素 | HTML ID | 对应数据 |
|---------|---------|---------|
| 模组连接状态 | `#statusLabel` | `state.connected` |
| 数据连接状态 | `#dataStatusLabel` | `state.dataConnected` |
| 模组图标 | `#statusDot` | `state.connected` 状态 |
| 数据图标 | `#dataDot` | `state.dataConnected` 状态 |
| 数据连接按钮 | `#dataConnectBtn` | `toggleDataConnection()` |

## 后端代码位置

### lib.rs 关键函数

| 函数 | 行号 | 功能 |
|------|------|------|
| `get_windows_all_port_info()` | 51-105 | Windows注册表读取串口友好名称 |
| `list_ports()` | 113-157 | 列出所有串口 |
| `is_at_port()` | 224-267 | 判断是否为AT端口 |
| `auto_connect_at()` | 272-348 | 自动连接AT端口 |
| `get_modem_status()` | 391-406 | 获取模组状态 |
| `get_hardware_info()` | 408-421 | 获取硬件信息 |
| `get_ip_info()` | 423-438 | 获取IP信息 |
| `get_neighbor_cells()` | 458-472 | 获取邻区信息 |
| `get_bands()` | 626-639 | 获取频段配置 |
| `set_bands()` | 641-653 | 设置频段 |
| `get_feature_toggles()` | 665-678 | 获取功能开关 |
| `set_feature_toggle()` | 680-701 | 设置功能开关 |
| `send_raw_at()` | 614-624 | 发送原始AT命令 |

### at_adapter.rs 关键函数

| 函数 | 行号 | AT指令 |
|------|------|--------|
| `query_modem_status()` | 18-169 | CPIN, CGSN, CCID, QENG, COPS, QANTRSSI, CGACT |
| `query_hardware_info()` | 172-215 | ATI, QBASELINE, QTEMP |
| `query_ip_info()` | 218-235 | QNETDEVSTATUS |
| `query_apn_list()` | 239-257 | CGACT, QICSGP? |
| `query_neighbor_cells()` | 261-269 | QENG="neighbourcell" |
| `query_qos()` | 272-287 | C5GQOSRDP |
| `query_bands()` | 375-410 | QNWPREFCFG=?/lte_band/nr5g_band |
| `query_feature_toggles()` | 471-518 | QCFG |
| `query_traffic()` | 574-583 | QGDCNT? |
| `set_apn()` | 290-309 | QICSGP |
| `connect_data()` | 324-332 | QNETDEVCTL |
| `set_network_mode()` | 346-355 | QNWPREFCFG="mode_pref" |
| `set_bands()` | 424-443 | QNWPREFCFG="lte_band"/"nr5g_band" |
| `reboot_modem()` | 446-453 | CRESET |
| `factory_reset()` | 456-463 | QFACT=0 |

### at_parser.rs 关键函数

| 函数 | 行号 | 解析响应 |
|------|------|--------|
| `is_ok()` | 4-7 | 检查OK结尾 |
| `extract_data_lines()` | 10-24 | 过滤回显/OK/ERROR |
| `parse_cpin()` | 28-35 | +CPIN: |
| `parse_cgsn()` | 37-45 | IMEI |
| `parse_ccid()` | 47-57 | +CCID: / +ICCID: |
| `parse_ati()` | 64-81 | ATI响应 |
| `parse_qeng_servingcell()` | 153-224 | +QENG: "servingcell" |
| `parse_cgact()` | 322-335 | +CGACT: |
| `parse_qicsgp()` | 360-391 | +QICSGP: |
| `parse_qeng_neighbourcell()` | 407-540 | +QENG: "neighbourcell" |
| `parse_cops()` | 544-566 | +COPS: |
| `parse_qnetdevstatus()` | 294-319 | +QNETDEVSTATUS: |
| `parse_qtemp()` | 678-698 | +QTEMP: |
| `parse_qnwprefcfg_supported()` | 736-776 | +QNWPREFCFG: |

### transport.rs 关键函数

| 函数 | 行号 | 功能 |
|------|------|------|
| `SerialTransport::new()` | 37-43 | 打开串口 |
| `SerialTransport::probe_at()` | 47-89 | AT端口探测 |
| `SerialTransport::send_at()` | 实现AtTransport | 发送AT命令 |
| `SerialTransport::read_response()` | 91-150 | 读取响应 |
| `TcpTransport::new()` | - | TCP连接 (如需) |

### 类型定义 (types.rs)

| 结构体 | 行号 | 用途 |
|--------|------|------|
| `ModemStatus` | 23-41 | 模组状态 |
| `HardwareInfo` | 53-61 | 硬件信息 |
| `IpInfo` | 65-73 | IP信息 |
| `ApnEntry` | 77-84 | APN配置 |
| `NeighborCell` | 88-97 | 邻区信息 |
| `NeighborCells` | 101-104 | 邻区列表 |
| `BandConfig` | 108-113 | 频段配置 |
| `FeatureToggles` | 117-124 | 功能开关 |
| `QosInfo` | 45-49 | QoS信息 |
| `TrafficInfo` | 128-131 | 流量统计 |
| `PortInfo` | 135-141 | 串口信息 |
| `AtTimingEntry` | 5-10 | AT耗时记录 |
| `AtTimingStats` | 14-19 | AT耗时统计 |