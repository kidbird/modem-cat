# 函数调用流程

## 1. 启动与初始化

```
应用启动
  │
  └─→ lib.rs: run()
              │
              ├─ tauri::Builder::default()
              │   ├─ setup() → 初始化系统托盘菜单
              │   │              ├─ tray_by_id("main")
              │   │              ├─ set_menu(控制面板/退出)
              │   │              └─ on_menu_event / on_tray_icon_event
              │   │
              │   ├─ on_window_event() → 窗口关闭时 hide() 而非退出
              │   │
              │   └─ .run(generate_context!())
              │
              └─→ 前端 index.html 加载
                      │
                      └─→ doInit()  [行 2291]
                              │
                              ├─ mark_init_start()
                              ├─ refreshPortList()
                              │       └─ invoke('list_ports')
                              │
                              └─ toggleConnection()
                                      │
                                      └─→ auto_connect_at()
                                              │
                                              ├─ serialport::available_ports()
                                              ├─ get_windows_all_port_info() [winreg]
                                              ├─ is_at_port() → 判断 AT 候选端口
                                              │
                                              ├─ SerialTransport::probe_at(port) [行 47-88]
                                              │       └─ 发送 AT\r\n, 800ms 超时等待 OK
                                              │
                                              └─ SerialTransport::new() → 建立连接
                                                      └─ 存储到 state.transport

                          // 连接成功后
                          └─ refreshAll() [行 1530]
                                  ├─ refreshModemStatus()
                                  ├─ refreshIpInfo()
                                  ├─ refreshApnList()
                                  ├─ refreshQos()
                                  └─ refreshTraffic()
```

## 2. 前端页面导航

```
用户点击左侧导航
  │
  └─→ nav-item click 事件
          │
          ├─ 移除所有 active 样式
          ├─ 添加当前 active 样式
          ├─ 显示对应 page
          │
          └─→ 懒加载检查
                  │
                  ├─ hardware 页面 → loadHardwarePage()
                  │       ├─ refreshHardwareInfo()
                  │       └─ refreshFeatureToggles()
                  │
                  ├─ neighbor tab → loadNeighborCells()
                  │
                  └─ netlock tab → loadNetlockData()
                          ├─ get_network_mode()
                          └─ get_bands()
```

## 3. AT 命令发送流程

```
invoke('send_raw_at', {command: "AT+XXX"})
  │
  └─→ lib.rs: send_raw_at() [行 614]
          │
          └─→ at_adapter.rs: send_raw_at() [行 466]
                  │
                  └─→ transport.rs: SerialTransport::send_at() [实现 AtTransport trait]
                          │
                          ├─ write_all(command + "\r\n")
                          ├─ flush()
                          ├─ read_response() [行 91]
                          │       │
                          │       ├─ 循环读取直到遇到 OK/ERROR 或超时
                          │       ├─ 8 秒整体超时
                          │       └─ 返回完整响应字符串
                          │
                          └─→ at_parser.rs: is_ok() 检查是否成功
```

## 4. 获取模组状态流程

```
invoke('get_modem_status')
  │
  └─→ lib.rs: get_modem_status() [async, 行 391]
          │
          ├─ tokio::spawn_blocking → 在阻塞线程中执行
          │
          └─→ at_adapter.rs: query_modem_status() [行 18]
                  │
                  ├─ AT+CPIN? → parse_cpin() → sim_status
                  │
                  ├─ AT+CGSN → parse_cgsn() → imei
                  │
                  ├─ AT+CCID → parse_ccid() → iccid
                  │   └─ 失败则 AT+QCCID
                  │
                  ├─ AT+QENG="servingcell" → parse_qeng_servingcell() [行 153]
                  │       │
                  │       ├─ NR5G-SA: 提取 17 个字段
                  │       ├─ LTE: 提取 19 个字段
                  │       └─ NR5G-NSA: 提取 15 个字段
                  │
                  ├─ AT+COPS? → parse_cops() → operator
                  │
                  ├─ AT+QANTRSSI? → parse_qantrssi() → ant_values[4]
                  │
                  ├─ AT+CGACT? → parse_cgact() → active_cids, conn_status
                  │
                  └─→ 返回 (ModemStatus, Vec<i32>)
                          └─ 缓存 active_cids 供后续 query_apn_list 使用

                  // 耗时记录
                  └─ log_query_timing("query_modem_status", elapsed, true)
                          └─ record_at_timing() → GLOBAL_TIMING
```

## 5. 获取硬件信息流程

```
invoke('get_hardware_info')
  │
  └─→ lib.rs: get_hardware_info() [async, 行 408]
          │
          └─→ at_adapter.rs: query_hardware_info() [行 172]
                  │
                  ├─ ATI → parse_ati() → (manufacturer, model, firmware)
                  │   └─ 失败则: CGMM + CGMI + GMR
                  │
                  ├─ AT+QBASELINE → parse_qbaseline() → (ap, cp)
                  │
                  ├─ AT+QTEMP → parse_qtemp() → (soc_temp, pa_temp)
                  │
                  └─→ 返回 HardwareInfo
```

## 6. 获取邻区流程

```
invoke('get_neighbor_cells')
  │
  └─→ lib.rs: get_neighbor_cells() [async, 行 458]
          │
          └─→ at_adapter.rs: query_neighbor_cells() [行 261]
                  │
                  └─→ AT+QENG="neighbourcell" → parse_qeng_neighbourcell() [行 407]
                          │
                          ├─ 解析 "NR" 类型邻区 → nr_cells
                          │
                          ├─ 解析 "LTE" 类型邻区 → lte_cells
                          │
                          ├─ 解析 "WCDMA" 类型邻区 → lte_cells (复用)
                          │
                          └─→ 返回 (lte_cells, nr_cells)
```

## 7. 频段配置流程

```
invoke('get_bands')
  │
  └─→ lib.rs: get_bands() [async, 行 626]
          │
          └─→ at_adapter.rs: query_bands() [行 375]
                  │
                  ├─ AT+QNWPREFCFG=? → parse_qnwprefcfg_supported() → (lte_supported, nr_supported)
                  │
                  ├─ AT+QNWPREFCFG="lte_band" → parse_qnwprefcfg_bands("lte_band") → lte_locked
                  │
                  ├─ AT+QNWPREFCFG="nr5g_band" → parse_qnwprefcfg_bands("nr5g_band") → nr_locked
                  │
                  └─→ 返回 BandConfig

invoke('set_bands', {lte: "1:3:5", nr: "1:3:5"})
  │
  └─→ at_adapter.rs: set_bands() [行 424]
          │
          ├─ AT+QNWPREFCFG="lte_band","1:3:5"
          │
          └─ AT+QNWPREFCFG="nr5g_band","1:3:5"
```

## 8. 功能开关流程

```
invoke('get_feature_toggles')
  │
  └─→ lib.rs: get_feature_toggles() [async, 行 665]
          │
          └─→ at_adapter.rs: query_feature_toggles() [行 471]
                  │
                  ├─ AT+QCFG="pcie/mode" → parse_qcfg_int() → pcie_mode
                  ├─ AT+QCFG="ethernet" → parse_qcfg_int() → ethernet
                  ├─ AT+QCFG="proxyarp" → parse_qcfg_int() → proxyarp
                  ├─ AT+QCFG="uartat" → parse_qcfg_int() → uartat
                  ├─ AT+QCFG="eth_at" → parse_qcfg_int() → eth_at
                  │
                  └─ AT+QCFG="usbcfg" → parse_qcfg_usbcfg_adb() → adb

invoke('set_feature_toggle', {feature: "adb", enabled: true})
  │
  └─→ lib.rs: set_feature_toggle() [行 680]
          │
          └─→ at_adapter.rs
                  ├─ feature="adb" → set_adb() [行 532]
                  │       ├─ 读取当前 usbcfg
                  │       ├─ 修改最后一位
                  │       └─ 写入 AT+QCFG="usbcfg",...
                  │
                  └─ 其他 → set_qcfg_toggle(key, value) [行 521]
                          └─ AT+QCFG="<key>",<value>
```

## 9. 数据连接流程

```
invoke('connect_data')
  │
  └─→ lib.rs: connect_data() [async, 行 539]
          │
          └─→ at_adapter.rs: connect_data() [行 324]
                  │
                  └─ AT+QNETDEVCTL=<cid>,3,1 → 连接

invoke('disconnect_data')
  │
  └─→ at_adapter.rs: disconnect_data() [行 336]
          │
          └─ AT+QNETDEVCTL=<cid>,2,0 → 断开
```

## 10. 高通 5GLAN（L2 / Ethernet PDU）配置流程

参考 `docs/AT_COMMANDS.md §9` 与
`Quectel_RG520N&RG525F&RG5x0F&RM5x0N_Series_5G_LAN_User_Guide_V1.0.0`。
实现位置：`modem-hal/src/vendors/quectel/mod.rs`，前端入口 `src/desktop/index.html#glan-qualcomm`。

```
前端「步骤 1 — 配置 PDP profile」
  │
  └─→ invoke('configure_qualcomm_5glan', {cid, apn, snssai, profileId, vlanStart, vlanEnd})
          │
          └─→ lib.rs → vendor.configure_qualcomm_5glan()
                  │   仅 Qualcomm，否则返回 Err
                  │
                  ├─ AT+QNWCFG="eth_cfg",<profile_id>,<eth_mode>
                  │     eth_mode = 1 (vlan_start<65535) | 2 (无 VLAN)
                  ├─ AT+CGDCONT=<cid>,"IPV4V6","<apn>",,,,,,,,,,,,,,1,"<snssai>",
                  └─ AT+QWDSCFG="profile",<cid>,"Ethernet","<apn>",<vlan_start>,<vlan_end>

前端「步骤 2 — 启用 ETH PDU 会话」⚠ 重启后才生效
  │
  └─→ invoke('enable_eth_pdu')
          │
          └─→ vendor.enable_eth_pdu()
                  └─ AT+QMAP="ETH_PDU","enable"
                  └─ ⚠ 用户需手动 AT+CFUN=1,1，重启前查询字段仍为 disable

前端「步骤 3 — 建立 MPDN 规则并拨号」
  │
  └─→ invoke('connect_qualcomm_5glan', {ruleId, cid})
          │
          └─→ vendor.connect_qualcomm_5glan()
                  ├─ AT+QMAP="mpdn_rule",<rule_id>,<cid>,0,0,0
                  └─ AT+QMAP="connect",<rule_id>,1

前端「刷新状态」（每次步骤完成后调用）
  │
  └─→ invoke('query_qualcomm_5glan_status')
          │
          └─→ vendor.query_qualcomm_5glan_status()
                  ├─ AT+QMAP="ETH_PDU"        → parse_eth_pdu_enabled() → bool
                  ├─ AT+QMAP="mpdn_rule"      → parse_mpdn_rule_cid(rule=1) → Option<i32>
                  └─ AT+QMAP="MPDN_status"    → parse_mpdn_connect_status_by_rule(rule=1) → bool
                          │
                          └─→ Qualcomm5GlanStatus { eth_pdu_enabled, mpdn_cid, connected }

VLAN 独立子配置（不属于上述三步主流程）
  │
  ├─→ invoke('query_vlan')        → AT+QMAP="VLAN"             → Vec<i32>（过滤 id=0）
  └─→ invoke('set_vlan', {vlanId, enabled})
          ├─ enable :  AT+QMAP="VLAN",<id>,"enable",1   （末尾 1=ETH 型）
          └─ disable:  AT+QMAP="VLAN",<id>,"disable"
```

### 平台守卫

所有上述 vendor 方法在非 Qualcomm 芯片上直接返回
`Err("... only supported on Qualcomm chip")`，避免误发命令；UniSoc 走的是
`AT+QCFG="5glan"`（`set_5glan`），不进入本流程。