# Code Map

> 最近更新：2026-07-07
> 覆盖：前端 UI 元素 → IPC 命令 → 后端 live handler 三列映射

## 1. 后端 live IPC 命令清单

> 当前 live IPC 统一注册在 `src-tauri/src/lib.rs::generate_handler!`，但实际执行面分散在 `handlers.rs`、`connection.rs`、`debug_terminal.rs`、`dloader.rs`。
> `list_ports` 对 USB 串口会附带 `usbVid` / `usbPid` / `detectedModel` / `detectedChipset`，供前端和自动连接链路复用同一份识别结果。
> `get_hardware_info` 只展示 `AppState.connected_usb_ids` 或 vendor 已返回的 `usbVid` / `usbPid`；若系统枚举阶段没有拿到 USB ID，则保持空值，不补发 `AT+QCFG="usbcfg"`。

- 连接类：`list_ports` / `auto_connect_at` / `connect_serial` / `connect_tcp` / `list_network_adapters` / `connect_websocket` / `disconnect`
- 状态类：`get_app_version` / `get_modem_status` / `get_hardware_info` / `get_ip_info` / `get_apn_list` / `get_neighbor_cells` / `get_qos_info` / `get_network_mode` / `get_ims_enabled` / `get_cfun_mode` / `get_lan_config` / `get_bands` / `get_feature_toggles` / `get_usbnet_mode` / `get_traffic` / `get_5glan` / `get_sim_slot` / `get_nat_mode` / `get_vlan` / `get_qualcomm_config`
- 写操作：`set_apn_config` / `delete_apn_config` / `set_apn_active` / `set_5glan` / `set_network_mode_cmd` / `set_ims_enabled` / `set_mtu` / `set_lan_config` / `set_dmz` / `clear_dmz` / `set_nr5g_band_cmd` / `set_bands` / `reset_all_bands` / `set_feature_toggle` / `set_usbnet_mode` / `set_sim_slot` / `set_nat_mode` / `set_vlan` / `set_qualcomm_config`
- 数据 / 射频 / 系统：`connect_data` / `disconnect_data` / `set_cfun` / `reboot_modem` / `factory_reset`
- 锁网 / 小区锁：`query_cell_lock` / `set_cell_lock` / `clear_cell_lock` / `set_plmn_lock` / `clear_plmn_lock`
- Qualcomm 5GLAN：`query_qualcomm_5glan_status` / `configure_qualcomm_5glan` / `enable_eth_pdu` / `connect_qualcomm_5glan`
- AT / MQTT：`send_raw_at` / `pop_at_commands` / `export_at_log` / `set_mqtt_enabled` / `get_mqtt_enabled`
- Debug Terminal：`get_debug_terminal_capabilities` / `list_debug_network_adapters` / `get_debug_terminal_prefs` / `save_debug_terminal_prefs` / `start_adb_session` / `start_ssh_session` / `write_debug_terminal_input` / `close_debug_terminal_session`
- Firmware：`pick_pac_file` / `pac_info` / `start_firmware_download` / `stop_firmware_download`

## 2. 前端 UI → IPC 触发点

### 2.1 状态页（`#page-status`）

| UI 元素 | HTML ID | 触发函数 | IPC 命令 |
|---|---|---|---|
| SIM 状态 | `#simStatus` | refreshModemStatus | `get_modem_status` |
| 注册状态 | `#regStatus` | 同上 | `get_modem_status` |
| 连接状态 | `#connStatus` | 同上 | `get_modem_status` |
| IMEI / ICCID / 运营商 / 网络类型 | 各自 ID | 同上 | `get_modem_status` |
| 流量 | `#ulTraffic` / `#dlTraffic` | refreshTraffic | `get_traffic` |
| 射频开关 | `#rfToggle` | onRfToggle | `set_cfun` + `get_modem_status` |
| 连接后射频状态回读 | `#rfToggle` | syncRfState | `get_cfun_mode` |
| SIM 卡槽切换 | `.sim-slot-option` | setSimSlot | `set_sim_slot` + `get_sim_slot` |
| 数据连接按钮 | `#dataConnectBtn` | toggleDataConnection | `connect_data` / `disconnect_data` |

### 2.2 蜂窝网络页（`#page-cellular`，5 个子 tab）

#### APN tab（`#ctab-apn`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| APN 列表 | refreshApnList | `get_apn_list` |
| 新增/编辑 | saveApn | `set_apn_config` |
| 删除 | deleteApn | `delete_apn_config` |
| 激活/取消 | toggleApnActive | `set_apn_active` |

#### 网络配置 tab（`#ctab-netlock`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 首选网络选择 | applyPreferredNetwork | `set_network_mode_cmd` |
| 网络模式（路由/网卡） | loadNetlockData / applyNatMode | `get_nat_mode` / `set_nat_mode` |
| 频段网格 | refreshBands | `get_bands` |
| 应用频段 | applyBandLock | `set_bands` |
| 重置频段 | resetBandLock | `reset_all_bands` |
| PLMN 锁 | applyOperatorLock / clearOperatorLock | `set_plmn_lock` / `clear_plmn_lock` |
| IMS 状态 | loadNetlockData / setIms | `get_ims_enabled` / `set_ims_enabled` |

#### 小区/频点锁定 tab（`#ctab-celllock`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 进入时拉取 | queryCellLock | `query_cell_lock` |
| 提交 | saveCellLock | `set_cell_lock` |
| 清除 | clearCellLock | `clear_cell_lock` |

#### 邻区 tab（`#ctab-neighbor`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 邻区表格 | loadNeighborCells / refreshNeighbors | `get_neighbor_cells` |

#### 5GLAN tab（`#ctab-5glan`，3 个子 tab）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| UniSoc 子 tab | refresh5Glan / saveUnisoc5Glan | `get_5glan` / `set_5glan` |
| Qualcomm 子 tab | refresh5GlanQualcommStatus | `query_qualcomm_5glan_status` |
| Qualcomm 提交配置 | configureQualcomm5Glan | `configure_qualcomm_5glan` |
| Qualcomm ETH PDU | enableEthPdu | `enable_eth_pdu` |
| Qualcomm 拨号 | connectQualcomm5Glan | `connect_qualcomm_5glan` |

### 2.3 AT 调试页（`#page-at`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 终端输出 | addTerminalLine / flushAtLog | `pop_at_commands`（轮询） |
| 命令输入 | sendAtCommand | `send_raw_at` |
| 快捷 AT 按钮 | quickAt | `send_raw_at` |
| 导出按钮 | exportAtLog | `export_at_log` |

### 2.4 ADB 调试页（`#page-adbdebug`）

| UI 元素 | HTML ID | 触发函数 | IPC 命令 |
|---|---|---|---|
| 页面初始化 | `#adbDebugNav` | `initDebugTerminal` | `get_debug_terminal_capabilities` / `get_debug_terminal_prefs` |
| 连接按钮 | `#adbDebugConnectBtn` | `connectAdbDebug` | `start_adb_session` |
| 终端键盘输入 | `#adbDebugTerminal` | `handleDebugTerminalKey` / `handleDebugTerminalPaste` | `write_debug_terminal_input` |
| 断开按钮 | `#adbDebugDisconnectBtn` | `disconnectDebugTerminal` | `close_debug_terminal_session` |
| 终端输出 | `#adbDebugTerminal` | `handleDebugOutputEvent` | `debug-terminal-output` 事件 |

> ADB 调试页不读取 modem AT 状态，不调用 `get_feature_toggles` / `AT+QCFG="usbcfg"`；是否启用 ADB 由 `adb.exe shell` 的实际启动结果返回。

### 2.5 SSH 调试页（`#page-sshdebug`）

| UI 元素 | HTML ID | 触发函数 | IPC 命令 |
|---|---|---|---|
| 网卡下拉框 | `#sshAdapterSelect` | `refreshSshDebugAdapters` / `handleSshAdapterChange` | `list_debug_network_adapters` |
| 用户名/最近 IP 回填 | `#sshUsername` / `#sshHost` | `loadDebugPrefs` | `get_debug_terminal_prefs` |
| 保存偏好 | — | `saveDebugPrefs` | `save_debug_terminal_prefs` |
| 连接按钮 | `#sshDebugConnectBtn` | `connectSshDebug` | `start_ssh_session` |
| 终端键盘输入 | `#sshDebugTerminal` | `handleDebugTerminalKey` / `handleDebugTerminalPaste` | `write_debug_terminal_input` |
| 断开按钮 | `#sshDebugDisconnectBtn` | `disconnectDebugTerminal` | `close_debug_terminal_session` |
| 终端输出 | `#sshDebugTerminal` | `handleDebugOutputEvent` | `debug-terminal-output` 事件 |

> `list_debug_network_adapters` 只返回有线网卡；默认把设备 IP 填成所选网卡网关，用户仍可手改。当前仅保存用户名、上次网卡和上次 IP，不保存密码。

### 2.6 系统信息页（`#page-hardware`，2 个子 tab）

#### UniSoc 子 tab（`#hwtab-unisoc`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 功能开关（PCIe/以太网/ProxyARP/UART/ETH/ADB/NAPT/...） | applyToggle | `set_feature_toggle` |
| USB 网卡模式 | changeUsbNetMode | `set_usbnet_mode` |
| QoS | refreshQos | `get_qos_info` |
| 流量 | refreshTraffic | `get_traffic` |

#### Qualcomm 子 tab（`#hwtab-qualcomm`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 模组型号/USB VID-PID/固件/温度/SN | refreshHardwareInfo | `get_hardware_info` |
| Qualcomm 配置 | refreshQualcommConfig | `get_qualcomm_config` |
| USB net / Data interface / PCIe / USB speed / IPPT | setQcXxxToggle | `set_qualcomm_config` |
| ETH driver | saveQcEthDriver | `set_qualcomm_config` |

### 2.7 IP 信息页（`#page-ip`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| IP/掩码/网关/DNS | refreshIpInfo | `get_ip_info` |
| MTU 设置 | applyMtu | `set_mtu` |
| DMZ 设置 | applyDmz / clearDmz | `set_dmz` / `clear_dmz` |
| LAN IP 查询 | refreshLanConfig | `get_lan_config` |
| LAN IP 配置 | applyLanConfig | `set_lan_config` |

> WebSocket 网关连接当前由状态页连接区域触发：`toggleConnection()` 在 `connectionType=ethernet` 时调用 `connect_websocket`，并可选传递显式用户名/密码；未提供时保持匿名模式，不再自动填入默认凭据。

### 2.8 情景模式页（`#page-scene`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 进入时并发预读 | loadScenePage | `get_feature_toggles` + `get_nat_mode` + `get_usbnet_mode` |
| 激活场景 | activateScene | `set_nat_mode` + `set_feature_toggle` × 2 + `set_usbnet_mode` |

### 2.9 系统设置页（`#page-settings`）

纯前端（语言 / 主题），无 IPC。

### 2.10 AT 手册页（`#page-atmanual`）

纯前端（静态 AT_DB），无 IPC。

### 2.11 固件下载页（`#page-firmware`）

| UI 元素 | HTML ID | 触发函数 | IPC 命令 |
|---|---|---|---|
| 选择 PAC | `#fwSelectPacBtn` | click handler | `pick_pac_file` → `pac_info` |
| 开始下载 | `#fwStartBtn` | click handler | `start_firmware_download` |
| 停止下载 | `#fwStopBtn` | click handler | `stop_firmware_download` |
| 进度条 | `#fwProgressFill` | — | 由 `firmware-event` 事件驱动 |
| 日志控制台 | `#fwLog` | — | 由 `firmware-event` 事件驱动 |

## 3. Tauri 事件订阅

| 事件名 | 触发源 | 前端回调位置 | 作用 |
|---|---|---|---|
| `port-changed` | usb-monitor 线程 + heartbeat 线程 | app.js `setupUsbMonitor` 函数 | `{ added, removed: PortChangeEntry[] }`；每项含 `portName` / `timestamp` / `usbVid` / `usbPid` / `detectedModel` / `detectedChipset`，用于日志上报、自动重连和强制断开 |
| `show-about` | Tauri 菜单 / 快捷键 | app.js `setupAboutListener` 函数 | `showAbout()` 显示关于对话框 |
| `debug-terminal-output` | debug_terminal.rs | debug-terminal.js `handleDebugOutputEvent` | ADB / SSH 终端输出与系统状态回显 |
| `firmware-event` | dloader.rs sidecar 事件转发 | app.js `fwListen` 回调 | Log/Progress/StateChange/Completed/Error/Terminated 等固件下载事件 |

## 4. UI 元素 → AT 命令 → 解析函数（重点页面）

### 4.1 状态页（`get_modem_status`）

| 显示 | AT 命令 | 解析函数 | 位置 |
|---|---|---|---|
| SIM 状态 | `AT+CPIN?` | `parse_cpin` | `quectel/parser.rs` |
| IMEI | `AT+CGSN` | `parse_cgsn` | `quectel/parser.rs` |
| ICCID (Qualcomm) | `AT+ICCID` | `parse_iccid` | `quectel/parser.rs` |
| ICCID (UniSoc) | `AT+CCID` | `parse_iccid` | `quectel/parser.rs` |
| 服务小区 | `AT+QENG="servingcell"` | `parse_qeng_serving_cell` | `quectel/parser.rs` |
| 运营商 | `AT+COPS?` | `parse_cops_with_act` | `quectel/parser.rs` |
| 注册状态（专用查询） | `AT+CEREG?` | `parse_cereg` | `quectel/parser.rs` |
| 天线 ANT0-3 (Qualcomm) | `AT+QRSRP` | `parse_qrsrp` | `quectel/parser.rs` |
| 天线 ANT0-3 (UniSoc) | `AT+QANTRSSI?` | `parse_qantrssi` | `quectel/parser.rs` |
| 数据激活状态 (Qualcomm) | `AT+QMAP="MPDN_status"` | `parse_mpdn_connect_status` | `quectel/qualcomm.rs` |

> `get_modem_status` 的 `regStatus` 当前来自 `AT+QENG="servingcell"` 解析后的 `mobility_state`；`AT+CEREG?` 仍作为专用查询合同保留，但不是该聚合接口的 live 主路径。
| 数据激活状态 (UniSoc) | `AT+CGACT?` | `parse_cgact` | `quectel/parser.rs` |

## 5. 前端缓存 / DOM

- 全局唯一前端状态源：`state`
- `$.dom` 在 `cacheDom()` 中缓存常用节点
- 5GLAN / APN / 场景 / Debug Terminal 等页面可有局部 UI 数据，但不能替代 live 状态 owner

## 6. 改前端需注意

- **所有 IPC 必须有 `await`**：Tauri v2 Promise 化
- **错误处理**：`invoke` 失败时 `catch` 拿到的对象是 `{ message: string }`，统一用 `e.message || String(e)` 兜底
- **懒加载**：重页面（如 hardware / ip / scene / atmanual / adbdebug / sshdebug / firmware）优先按需拉取，不要在启动时堆满同步请求
- **Tauri 事件**：`listen` 必须在初始化阶段注册
- **MQTT 状态回读后端**：UI 不得把 MQTT 开关写入 `localStorage` 充当 live 状态
- **WebSocket 凭据显式输入**：允许匿名网关；若网关要求认证，必须由用户显式提供用户名/密码，禁止补默认值
