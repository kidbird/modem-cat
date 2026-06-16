# Code Map

> 最近更新：2026-06-01
> 覆盖：前端 UI 元素 → IPC 命令 → 后端实现 三列映射

## 1. 后端 IPC 命令清单（52 个，由 `lib.rs` 暴露）

> 旧 `commands.rs` 还有 30 个同名/类似命令，但 0 caller 全部死代码（见 [REVIEW.md](REVIEW.md)#1）。

| # | IPC 命令 | 后端位置 | 类别 | 异步模型 |
|---|---|---|---|---|
| 1 | `get_app_version` | lib.rs | 系统 | 同步 |
| 2 | `list_ports` | lib.rs (→ ports.rs) | 连接 | 同步 |
| 3 | `auto_connect_at` | lib.rs | 连接 | spawn_blocking |
| 4 | `connect_serial` | lib.rs (→ ports.rs) | 连接 | spawn_blocking |
| 5 | `connect_tcp` | lib.rs (→ ports.rs) | 连接 | spawn_blocking |
| 6 | `disconnect` | lib.rs | 连接 | 同步 |
| 7 | `get_modem_status` | lib.rs | 状态 | spawn_blocking |
| 8 | `get_hardware_info` | lib.rs | 状态 | spawn_blocking |
| 9 | `get_ip_info` | lib.rs | 状态 | spawn_blocking |
| 10 | `get_apn_list` | lib.rs | APN | spawn_blocking |
| 11 | `get_neighbor_cells` | lib.rs | 网络 | spawn_blocking |
| 12 | `get_qos_info` | lib.rs | 网络 | spawn_blocking |
| 13 | `get_network_mode` | lib.rs | 网络 | spawn_blocking |
| 14 | `get_bands` | lib.rs | 频段 | spawn_blocking |
| 15 | `get_feature_toggles` | lib.rs | 开关 | spawn_blocking |
| 16 | `get_usbnet_mode` | lib.rs | 开关 | spawn_blocking |
| 17 | `get_traffic` | lib.rs | 流量 | spawn_blocking |
| 18 | `get_5glan` | lib.rs | 5GLAN | spawn_blocking |
| 19 | `get_sim_slot` | lib.rs | SIM | spawn_blocking |
| 20 | `get_nat_mode` | lib.rs | 情景 | spawn_blocking |
| 21 | `get_vlan` | lib.rs | VLAN | spawn_blocking |
| 22 | `get_qualcomm_config` | lib.rs | 高通 | spawn_blocking |
| 23 | `set_apn_config` | lib.rs | APN | spawn_blocking |
| 24 | `delete_apn_config` | lib.rs | APN | spawn_blocking |
| 25 | `set_apn_active` | lib.rs | APN | spawn_blocking |
| 26 | `set_5glan` | lib.rs | 5GLAN | spawn_blocking |
| 27 | `set_network_mode_cmd` | lib.rs | 网络 | spawn_blocking |
| 28 | `set_nr5g_band_cmd` | lib.rs | 频段 | spawn_blocking |
| 29 | `set_bands` | lib.rs | 频段 | spawn_blocking |
| 30 | `reset_all_bands` | lib.rs | 频段 | spawn_blocking |
| 31 | `set_feature_toggle` | lib.rs | 开关 | spawn_blocking |
| 32 | `set_usbnet_mode` | lib.rs | 开关 | spawn_blocking |
| 33 | `set_sim_slot` | lib.rs | SIM | spawn_blocking |
| 34 | `set_nat_mode` | lib.rs | 情景 | spawn_blocking |
| 35 | `set_vlan` | lib.rs | VLAN | spawn_blocking |
| 36 | `set_qualcomm_config` | lib.rs | 高通 | spawn_blocking |
| 37 | `connect_data` | lib.rs | 数据 | spawn_blocking |
| 38 | `disconnect_data` | lib.rs | 数据 | spawn_blocking |
| 39 | `set_cfun` | lib.rs | 射频 | spawn_blocking |
| 40 | `reboot_modem` | lib.rs | 系统 | spawn_blocking |
| 41 | `factory_reset` | lib.rs | 系统 | spawn_blocking |
| 42 | `set_plmn_lock` | lib.rs | PLMN | spawn_blocking |
| 43 | `clear_plmn_lock` | lib.rs | PLMN | spawn_blocking |
| 44 | `query_cell_lock` | lib.rs | 小区锁 | spawn_blocking |
| 45 | `set_cell_lock` | lib.rs | 小区锁 | spawn_blocking |
| 46 | `clear_cell_lock` | lib.rs | 小区锁 | spawn_blocking |
| 47 | `query_qualcomm_5glan_status` | lib.rs | 5GLAN | spawn_blocking |
| 48 | `configure_qualcomm_5glan` | lib.rs | 5GLAN | spawn_blocking |
| 49 | `enable_eth_pdu` | lib.rs | 5GLAN | spawn_blocking |
| 50 | `connect_qualcomm_5glan` | lib.rs | 5GLAN | spawn_blocking |
| 51 | `send_raw_at` | lib.rs | AT | spawn_blocking；完整 AT 命令走 `validate_raw_at_command` |
| 52 | `pop_at_commands` | lib.rs | AT 日志 | 同步 |

> 注：三套独立数字：
> - 前端同步副本 `app.js` 有 **32 个**唯一 `invoke('...')` 调用（`grep -oE "invoke\('[a-z_]+'" src/desktop/app.js | sort -u | wc -l`）
> - 实际入口 `index.html` 有 **44 个**唯一 `invoke('...')` 调用（同上命令替换文件名）
> - 后端 `lib.rs` 实际暴露 **52 个** `#[tauri::command]`（`invoke_handler!` 块中注册）
> - `commands.rs` 另有 **30 个**同名/类似死代码（0 caller，REVIEW.md#1）
> 三套数加起来不一致是正常的；要求是前端调用名必须都能在后端实际暴露命令中找到，不算死代码。

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
| 频段网格 | refreshBands | `get_bands` |
| 应用频段 | applyBandLock | `set_bands` |
| 重置频段 | resetBandLock | `reset_all_bands` |
| PLMN 锁 | applyOperatorLock / clearOperatorLock | `set_plmn_lock` / `clear_plmn_lock` |
| IMS 状态 | loadNetlockData / setIms | `send_raw_at`（IMS 走兜底） |

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

### 2.4 系统信息页（`#page-hardware`，2 个子 tab）

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
| 模组型号/固件/温度 | refreshHardwareInfo | `get_hardware_info` |
| Qualcomm 配置 | refreshQualcommConfig | `get_qualcomm_config` |
| USB net / Data interface / PCIe / USB speed / IPPT | setQcXxxToggle | `set_qualcomm_config` |
| ETH driver | saveQcEthDriver | `set_qualcomm_config` |

### 2.5 IP 信息页（`#page-ip`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| IP/掩码/网关/DNS | refreshIpInfo | `get_ip_info` |
| MTU 设置 | applyMtu | `send_raw_at`（兜底） |
| DMZ 设置 | applyDmz / clearDmz | `send_raw_at`（兜底） |
| LAN IP 查询 | refreshLanConfig | `send_raw_at`（兜底） |
| LAN IP 配置 | applyLanConfig | `send_raw_at`（兜底） |

### 2.6 情景模式页（`#page-scene`）

| UI 元素 | 触发函数 | IPC 命令 |
|---|---|---|
| 进入时并发预读 | loadScenePage | `get_feature_toggles` + `get_nat_mode` + `get_usbnet_mode` |
| 激活场景 | activateScene | `set_nat_mode` + `set_feature_toggle` × 3 + `set_usbnet_mode` |

### 2.7 系统设置页（`#page-settings`）

纯前端（语言 / 主题），无 IPC。

### 2.8 AT 手册页（`#page-atmanual`）

纯前端（静态 AT_DB），无 IPC。

## 3. Tauri 事件订阅

| 事件名 | 触发源 | 前端回调位置 | 作用 |
|---|---|---|---|
| `port-changed` | usb-monitor 线程 + heartbeat 线程 | app.js `setupUsbMonitor` 函数 | `{ added, removed: string[] }`，触发自动重连 / 强制断开 |
| `show-about` | Tauri 菜单 / 快捷键 | app.js `setupAboutListener` 函数 | `showAbout()` 显示关于对话框 |

## 4. UI 元素 → AT 命令 → 解析函数（重点页面）

### 4.1 状态页（`get_modem_status`）

| 显示 | AT 命令 | 解析函数 | parser.rs 位置 |
|---|---|---|---|
| SIM 状态 | `AT+CPIN?` | parse_cpin | quectel/parser.rs |
| IMEI | `AT+CGSN` | parse_cgsn | quectel/parser.rs |
| ICCID (Qualcomm) | `AT+ICCID` | parse_iccid | quectel/parser.rs |
| ICCID (UniSoc) | `AT+CCID` | parse_iccid | quectel/parser.rs |
| 服务小区 | `AT+QENG="servingcell"` | parse_qeng_serving_cell | quectel/parser.rs |
| 运营商 | `AT+COPS?` | parse_cops_with_act | quectel/parser.rs |
| 注册状态 | `AT+CEREG?` | parse_cereg | quectel/parser.rs |
| 天线 ANT0-3 (Qualcomm) | `AT+QRSRP` | parse_qrsrp | quectel/parser.rs |
| 天线 ANT0-3 (UniSoc) | `AT+QANTRSSI?` | parse_qantrssi | quectel/parser.rs |
| 数据激活状态 | `AT+CGACT?` | parse_cgact | quectel/parser.rs |

## 5. 前端缓存 / DOM

- 单一 `state` 对象（app.js 顶部 `let state = { ... }`，行号随 commit 漂移）— 全局唯一状态源
- `$.dom` 缓存（在 `cacheDom()` 函数中，app.js 启动段；行号漂移）— 一次预查常用 ID
- 5GLAN / APN / 场景等页面有局部变量（`apnData`, `glanData`, `sceneCurrentState`）

## 6. 改前端需注意

- **所有 IPC 必须有 `await`**：Tauri v2 Promise 化
- **错误处理**：`invoke` 失败时 `catch` 拿到的对象是 `{ message: string }`，统一用 `e.message || String(e)` 兜底
- **懒加载**：4 个页面 (`hardware` / `ip` / `scene` / `atmanual`) 在 nav-click 时按需加载数据，其它 4 个靠 `doInit` 一次性拉
- **Tauri 事件**：`listen` 必须在 `doInit` 内调用（`DOMContentLoaded` + 3s timeout 兜底）
