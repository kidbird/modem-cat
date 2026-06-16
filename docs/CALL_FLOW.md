# 函数调用流程

> 最近更新：2026-06-16（对齐 `main` 分支当前代码）

## 1. 启动与初始化

```
应用启动
  │
  └─→ main.rs
        └─→ NO_PROXY=tauri.localhost,localhost,127.0.0.1
        └─→ modem_cat_lib::run()
              │
              ▼
        [lib.rs:run()]
              │
              ├─ tauri::Builder::default()
              │    ├─ setup() → 初始化系统托盘菜单
              │    │              ├─ tray_by_id("main")
              │    │              ├─ set_menu(控制面板/退出)
              │    │              └─ on_menu_event / on_tray_icon_event
              │    │
              │    ├─ invoke_handler!  ← 30 个 IPC 命令注册
              │    │
              │    ├─ on_window_event() → 窗口关闭时 hide() 而非退出
              │    │
              │    └─ .setup()  → 启动后台线程
              │         ├─ start_port_monitor(app.handle().clone())  ← 2s 轮询
              │         └─ start_connection_heartbeat(...)           ← 4s 轮询
              │
              ▼
        前端 index.html 加载
              │
              └─→ doInit()  [app.js 启动入口]
                      │
                      ├─ mark_init_start()
                      ├─ refreshPortList()
                      │       └─ invoke('list_ports')
                      │
                      ├─ setupUsbMonitor()  ← listen('port-changed')
                      ├─ setupAboutListener()  ← listen('show-about')
                      │
                      └─ toggleConnection()  ← 若已连接则跳过
                              │
                              └─→ auto_connect_at()  [lib.rs:353-429]
                                      │
                                      ├─ serialport::available_ports()
                                      ├─ get_windows_all_port_info() [winreg, cfg(windows)]
                                      ├─ is_at_port() → 判断 AT 候选端口
                                      │
                                      ├─ SerialTransport::probe_at(port)  ← 800ms AT\r\n
                                      │
                                      └─ ModemFactory::create()  ← AT+CGMM 型号检测
                                              └─→ 成功：建 LoggingTransport → AppState.transport
                                              └─→ 失败：尝试下一个端口

                          // 连接成功后
                          └─ refreshAll()  [app.js 启动段 doInit 内]
                                  ├─ Promise.allSettled
                                  │   ├─ refreshModemStatus()  → get_modem_status
                                  │   ├─ refreshIpInfo()       → get_ip_info
                                  │   ├─ refreshApnList()      → get_apn_list
                                  │   ├─ refreshQos()          → get_qos_info
                                  │   └─ refreshTraffic()      → get_traffic
                                  └─ 各 promise 完成时更新 state.connected / chipVendor
```

## 2. 前端页面导航

```
用户点击左侧导航 .nav-item  [app.js nav click handler]
  │
  ├─ 移除所有 .active
  ├─ 当前项加 .active
  ├─ 对应 #page-xxx 加 .active
  │
  └─ 懒加载检查（仅 4 个页面）
       ├─ hardware → loadHardwarePage()  → refreshHardwareInfo + refreshFeatureToggles
       ├─ ip       → refreshLanConfig()  → send_raw_at (AT+QCFG="lanip_ex")
       ├─ scene    → loadScenePage()     → 并发拉 feature_toggles / nat_mode / usbnet_mode
       └─ atmanual → initAtdbPage()      → 纯前端 AT_DB 索引
```

## 3. 通用 IPC 调用链（标准模式）

```
[前端]  invoke('command_name', args)
  │
  ▼
[Tauri runtime]  路由到 #[tauri::command] 函数  (在 lib.rs)
  │
  ▼
[lib.rs::handler]  with_vendor! 宏展开
  │
  ├─ tokio::task::spawn_blocking(move || { ... })
  │     │
  │     ├─ let mut tguard = state.transport.lock()?;     ← std::sync::Mutex
  │     ├─ let mut vguard = state.vendor.lock()?;
  │     ├─ let t = tguard.as_deref_mut().ok_or("Not connected")?;
  │     ├─ let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
  │     │
  │     └─ v.method_name(t, args)  ← ModemVendor trait
  │              │
  │              └─ QuectelModem::xxx  →  match self.chip {
  │                     Qualcomm => vendors/quectel/qualcomm.rs::xxx,
  │                     UniSoc   => vendors/quectel/unisoc.rs::xxx,
  │                  }
  │
  ▼
[ModemVendor::method]  调 t.send_at("AT+xxx")
  │
  ▼
[LoggingTransport::send_at]  (lib.rs:33)
  │
  ├─ log.push(redact_at_command(cmd))  ← 进 1000 条环形日志
  │
  └─ inner.send_at(cmd)  ← 真实 SerialTransport / TcpTransport
       │
       ├─ write_all(cmd + "\r\n")
       ├─ flush()
       └─ read_response()  循环读直到 OK/ERROR/+CME ERROR (8s 总超时)
            │
            ▼
       Result<String, String>  ← 沿原路径回前端（JSON 序列化）
```

## 4. 状态查询流程（`get_modem_status`）

```
invoke('get_modem_status')
  │
  └─→ lib.rs::get_modem_status  [with_vendor! 宏]
          │
          └─→ vendor.query_modem_status(&mut transport)
                  │
                  ├─ AT+CPIN?           → parse_cpin           → sim_status
                  ├─ AT+CGSN            → parse_cgsn           → imei
                  ├─ AT+ICCID (Qual)    → parse_iccid          → iccid
                  │  AT+CCID   (UniSoc)
                  ├─ AT+QENG="servingcell" → parse_qeng_serving_cell
                  │     ├─ NR5G-SA: 17 字段
                  │     ├─ LTE:     19 字段
                  │     └─ NR5G-NSA: 15 字段
                  ├─ AT+COPS?           → parse_cops_with_act  → operator
                  ├─ AT+QRSRP (Qual)    → parse_qrsrp          → ant[4]
                  │  AT+QANTRSSI? (UniS)
                  ├─ AT+CGACT?          → parse_cgact          → active_cids, conn_status
                  │
                  └─→ ModemStatus { sim, imei, iccid, operator, ant[], active_cids, ... }
```

## 5. 数据连接流程

```
invoke('connect_data', { cid: 1 })
  │
  └─→ lib.rs::connect_data  [with_vendor_cid! 宏]
          │
          └─→ vendor.connect_data(&mut transport, cid)
                  │
                  ├─ QuectelModem (Qualcomm) → qualcomm::connect_data
                  │     AT+QMAP="connect",<cid>,1  → is_ok 检查
                  │
                  ├─ QuectelModem (UniSoc) → unisoc::connect_data
                  │     AT+CGACT=1,<cid>  → is_ok 检查
```

## 6. IP 查询流程

```
invoke('get_ip_info', { cid: 1 })
  │
  └─→ lib.rs::get_ip_info  [with_vendor_cid! 宏]
          │
          └─→ vendor.query_ip_info(&mut transport, cid)
                  │
                  ├─ Qualcomm  → qualcomm::query_ip_info
                  │     ├─ AT+QMAP="MPPDN_status"  → parse_mpdn_connect_status
                  │     ├─ AT+CGPADDR  → parse_cgpaddr
                  │     └─ AT+QMAP="WWAN"  → parse_qmap_wwan
                  │     → 拼装 IpInfo { ip, mask, gw, dns_primary, dns_secondary }
                  │
                  ├─ UniSoc    → unisoc::query_ip_info
                  │     AT+QNETDEVSTATUS=<cid>  → 12 字段解析
                  │       [ipv4, mask, gw, empty, dns1, dns2, ipv6, ×3, v6dns1, v6dns2]
```

## 7. 5GLAN 流程（Qualcomm 三步）

```
[UniSoc 路径]  简化版，1 步
  invoke('set_5glan', { cid, enabled, vlanId })
    → AT+QCFG="5glan",<cid>,<0|1>,<vlan>

[Qualcomm 路径]  复杂 3 步 + ETH PDU
  Step 1: invoke('configure_qualcomm_5glan', { cid, apn, snssai, vlanStart, vlanEnd })
          → AT+QMAP="mPDN_rule",0,1,0,<auto_connect>,1,"FF:FF:FF:FF:FF:FF"
            (IPPT_mode=0 路由)  或  ippt_mode=1 桥接
  Step 2: invoke('enable_eth_pdu')  → AT+QMAP="ETH_PDU","enable"
  Step 3: invoke('connect_qualcomm_5glan', { rule, cid })
          → AT+QMAP="connect",<cid>,1
  状态: invoke('query_qualcomm_5glan_status')  → AT+QMAP="MPDN_status"
```

## 8. 频段配置流程

```
invoke('get_bands')
  │
  └─→ vendor.query_band_config
        │
        ├─ AT+QNWPREFCFG=?            → parse_qnwprefcfg_supported (lte_supported, nr_supported)
        ├─ AT+QNWPREFCFG="lte_band"   → parse_qnwprefcfg_bands → lte_locked
        └─ AT+QNWPREFCFG="nr5g_band"  → parse_qnwprefcfg_bands → nr_locked

invoke('set_bands', { lte: "1:3:5", nr: "78:79" })
  │
  └─→ vendor.set_lte_bands / set_nr5g_bands
        ├─ AT+QNWPREFCFG="lte_band","1:3:5"     ← 频段号纯数字、冒号分隔
        └─ AT+QNWPREFCFG="nr5g_band","78:79"

invoke('reset_all_bands')
  └─→ AT+QNWPREFCFG="all_band_reset"
```

## 9. 功能开关流程

```
invoke('get_feature_toggles')
  │
  └─→ vendor.query_feature_toggles
        ├─ AT+QCFG="pcie/mode"   → parse_qcfg_int → pcie_mode
        ├─ AT+QCFG="ethernet"    → parse_qcfg_int → ethernet
        ├─ AT+QCFG="proxyarp"    → parse_qcfg_int → proxyarp
        ├─ AT+QCFG="uartat"      → parse_qcfg_int → uartat
        ├─ AT+QCFG="eth_at"      → parse_qcfg_int → eth_at
        └─ AT+QCFG="usbcfg"      → parse_qcfg_usbcfg_adb → adb

invoke('set_feature_toggle', { feature: "adb", enabled: true })
  │
  └─→ lib.rs::set_feature_toggle
        │
        ├─ feature="adb" → 读当前 usbcfg，修改最后一位，写回
        └─ 其它          → AT+QCFG="<feature>",<0|1>
```

## 10. AT 调试页（`send_raw_at`）

> 本路径发送完整 AT 命令，使用 `validate_raw_at_command` 校验；不要复用只面向参数的 `validate_at_string`。

```
[app.js]  invoke('send_raw_at', { command: "AT+XXX" })
  │
  └─→ lib.rs::send_raw_at  (line 725)
        │
        └─→ tokio::task::spawn_blocking
              │
              └─→ state.transport.lock()?.send_at(cmd)
                    │
                    ├─ [LoggingTransport] log.push(redact(cmd))
                    └─ inner.send_at(cmd)  ← SerialTransport 8s 总超时
                          │
                          └─→ read_response()  返回完整字符串

  返回 Result<String, String>  →  按 \n 切分到 #terminal
       ├─ "OK"      → class="ok"
       ├─ "ERROR"   → class="err"
       └─ 其它      → class="resp"
```

## 11. 后台监控线程

### 11.1 `usb-monitor`（2s 间隔）

```
[setup 阶段]  start_port_monitor(app_handle)
  │
  └─→ std::thread::Builder::new().name("usb-monitor").spawn(|| {
        loop {
          let ports = serialport::available_ports()?;
          let added   = ports_new - last_set;
          let removed = last_set - ports_new;
          if !added.is_empty() || !removed.is_empty() {
            app_handle.emit("port-changed", { added, removed })?;
          }
          std::thread::sleep(Duration::from_secs(2));
        }
      })

```

### 11.2 `connection-heartbeat`（4s 间隔）

```
[setup 阶段]  start_connection_heartbeat(app_handle, transport, connected_port)
  │
  └─→ std::thread::Builder::new().name("connection-heartbeat").spawn(|| {
        loop {
          // 绝不阻塞 IPC —— 拿不到锁就 skip，下个 tick 再试
          let port = connected_port.try_lock().ok()?.clone();
          if let Some(name) = port {
            let alive = transport.try_lock().ok()?.as_deref().is_alive();
            if !alive {
              connected_port.try_lock().ok()?.take();
              app_handle.emit("port-changed", { added: [], removed: [name] })?;
            }
          }
          std::thread::sleep(Duration::from_secs(4));
        }
      })
```

## 12. USB 拔插 → 前端响应

```
[USB 拔]
  │
  ├─ usb-monitor 2s 内检测到 port 消失
  │     → emit "port-changed" { removed: [port] }
  │
  └─ heartbeat 4s 内 is_alive()=false
        → emit "port-changed" { removed: [port] }  （兜底）
        │
        ▼
[app.js setupUsbMonitor 函数]  listen("port-changed", payload => { ... })
  │
  ├─ if removed.includes(state.connectedPort):
  │     state.connected = false
  │     updateConnectionUI()
  │     disconnect_data()
  └─ if added.length > 0 && !state.connected:
        toggleConnection()  ← 自动重连
```

## 13. 工厂模式流程

### 13.1 初始化

```
[前端]  用户首次点击工厂模式导航
  │
  └─→ factoryInit()  [app.js]
        │
        ├─ invoke('init_factory')
        │     └─→ factory.rs::init_factory
        │           ├─ 加载/创建 factory_basecfg.json（嵌入式回退数据）
        │           ├─ 加载/创建 factory_select.json（当前产品选择）
        │           └─ 加载/创建 factory_execute.json（执行历史）
        │
        ├─ invoke('factory_get_base_data')  → 品牌/类型/工厂配置
        ├─ invoke('factory_get_current_product')  → 当前选中产品
        ├─ invoke('factory_get_current_sn')  → 当前 SN
        │
        └─ factoryRenderDropdowns() + factoryUpdateSnDisplay()
```

### 13.2 写入 SN 完整流程

```
[前端]  factoryWriteSn()
  │
  ├─ 检查连接状态（未连接则提示）
  │
  ├─ invoke('factory_write_sn_to_device', { sn })
  │     └─→ factory.rs::factory_write_sn_to_device
  │           └─→ DeviceClient::set_device_sn(sn)
  │                 ├─ POST /api/device_sn_set  ← 写入 SN
  │                 └─ GET /api/device_sn_get   ← 回读验证
  │
  ├─ invoke('factory_save_execute_data')  ← 递增序号，保存执行记录
  │
  ├─ invoke('factory_get_device_info')
  │     └─→ factory.rs::factory_get_device_info
  │           └─→ DeviceClient::get_device_info()
  │                 ├─ GET /api/device_activate_info  → IMEI/ICCID/固件版本
  │                 ├─ GET /api/device_name_get       → 设备名称
  │                 └─ GET /api/license_get           → 激活状态
  │
  ├─ invoke('factory_save_device_record', { deviceInfo })
  │     └─→ factory.rs::factory_save_device_record
  │           └─→ DataManager::append_csv_record()  ← 写入 factory_YYYY-MM-DD.csv
  │
  ├─ invoke('factory_increment_sequence')  → 新 SN
  │
  └─ factoryUpdateSnDisplay() + factoryUpdateDeviceInfo() + factoryUpdateRecordsTab()
```

### 13.3 SN 生成算法

```
SN = brand_code + type_code + fac_code + year_code + month_code + seq_code

示例：A0116C00001
  A      ← 品牌代码（AL）
  01     ← 类型代码（D100）
  1      ← 工厂代码（北京5层）
  6      ← 年代码（2026 → 6）
  C      ← 月代码（12月 → C，十六进制大写）
  00001  ← 序号（5 位零填充，00001 ~ 99999）
```

## 14. 固件下载流程

### 14.1 PAC 选择与分析

```
[前端]  点击"选择 PAC"按钮
  │
  ├─ invoke('pick_pac_file')
  │     └─→ dloader.rs::pick_pac_file
  │           └─ app.dialog().file().pick_file()  ← 原生文件选择对话框
  │
  └─ invoke('pac_info', { path })
        └─→ dloader.rs::pac_info
              ├─ ensure_sidecar_version()  ← 版本校验（OnceLock 缓存）
              └─ run_pac_info()
                    └─ app.shell().sidecar("r26-cli")
                          .args(["pac-info", path, "--json"])
                          .output()
                          └─→ r26-cli 解析 PAC 文件 → SafetyReportDto
```

### 14.2 下载执行

```
[前端]  点击"开始下载"按钮
  │
  └─ invoke('start_firmware_download', { path })
        └─→ dloader.rs::start_firmware_download
              │
              ├─ ensure_sidecar_version()
              ├─ run_pac_info()  ← TOCTOU 保护：重新分析，不信任前端缓存
              ├─ plan_flash()    ← 安全策略检查
              │     ├─ RF 校准 / PhaseCheck → Blocked（禁止刷写）
              │     └─ Erase / NV → Proceed + allow_flags
              │
              ├─ 构建参数：download <path> --port 0 --json [--allow-erase] [--allow-nv-write]
              │
              ├─ 获取 DloaderState 锁，检查无正在进行的下载
              │
              ├─ app.shell().sidecar("r26-cli").args(args).spawn()
              │     └─→ 启动 32 位 r26-cli sidecar 进程
              │
              └─ tauri::async_runtime::spawn(async move {
                    // 转发 sidecar stdout 事件
                    while let Some(event) = rx.recv().await {
                        match event {
                            CommandEvent::Stdout(bytes) => {
                                // 解析 JSON → FirmwareEvent
                                // emit("firmware-event", fw_event)
                            }
                            CommandEvent::Terminated(payload) => {
                                // 清理 child_slot，emit Terminated 事件
                            }
                            ...
                        }
                    }
                 })
```

### 14.3 事件处理（前端）

```
[前端]  fwListen('firmware-event', callback)
  │
  ├─ Log         → fwLog() 追加到日志控制台
  ├─ Progress    → fwSetProgress(file_id, percent) 更新进度条
  ├─ PacLoadProgress → fwSetProgress('加载 PAC…', percent)
  ├─ StateChange → fwLog() 记录状态转换
  ├─ Completed   → fwSetDownloading(false) + fwSetResult(成功/失败)
  ├─ Error       → fwSetDownloading(false) + fwSetResult(错误)
  └─ Terminated  → fwSetDownloading(false) + fwSetResult(异常退出/已停止)
```

### 14.4 三层架构

```
┌─────────────────────────────────────────────────────────────┐
│ 前端 (app.js)                                                │
│   invoke() → Tauri IPC                                      │
│   listen('firmware-event') ← 事件订阅                       │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Tauri 后端 (dloader.rs, 64-bit)                              │
│   pick_pac_file / pac_info / start_firmware_download         │
│   spawn sidecar → 转发 JSON 事件 → emit("firmware-event")    │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ r26-cli sidecar (32-bit, WOW64)                              │
│   pac-info → 解析 PAC 文件 → SafetyReport                    │
│   download → 驱动 DLFrame.dll → BSL 协议刷写                 │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
                    DLFrame.dll (Unisoc ResearchDownload)
```

## 15. License 控制流程

```
[应用启动]
  │
  └─→ lib.rs::setup()
        └─ license::init_license()
              ├─ 检查 license.dat 是否存在
              ├─ 存在 → modem_license::verify() 验签
              │     ├─ 有效 → LicenseStatus { valid: true, features: {...} }
              │     └─ 无效/过期 → LicenseStatus { valid: false, ... }
              └─ 不存在 → LicenseStatus { valid: false, ... }

[前端导航可见性]
  │
  └─→ updateLicenseNavVisibility()
        ├─ 工厂模式导航：valid && factory_mode → 显示
        └─ 固件下载导航：valid && firmware_download → 显示

[加载 License 文件]
  │
  └─ invoke('load_license_file', { path })
        └─→ license.rs::load_license_file
              ├─ modem_license::verify() 验签 + 解析
              └─ 更新 AppState.license → emit('license-changed')
```
