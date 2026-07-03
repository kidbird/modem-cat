# 函数调用流程

> 最近更新：2026-06-22（对齐 `main` 分支当前代码）

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
              │    ├─ invoke_handler!  ← live IPC 命令注册（分发到 handlers / connection / license / factory / dloader）
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
                              └─→ connection.rs::auto_connect_at()
                                      │
                                      ├─ serialport::available_ports()
                                      ├─ get_windows_all_port_info() [winreg, cfg(windows)]
                                      ├─ is_at_port() → 判断 AT 候选端口
                                      │
                                      ├─ spawn_blocking + `AT` probe
                                      │
                                      └─ ModemFactory::create()  ← AT+CGMM 型号检测
                                              └─→ 成功：建 LoggingTransport → AppState.transport
                                              └─→ 失败：尝试下一个端口

                          // 连接成功后
                          └─ refreshAll()  [app.js 启动段 doInit 内]
                                  ├─ sequential try/catch（非并行）
                                  │   ├─ refreshModemStatus()    → get_modem_status
                                  │   ├─ refreshIpInfo()         → get_ip_info
                                  │   ├─ refreshHardwareInfo()   → get_hardware_info
                                  │   ├─ refreshApnList()        → get_apn_list
                                  │   ├─ refreshQos()            → get_qos_info
                                  │   └─ refreshTraffic()        → get_traffic
                                  └─ 各调用独立 catch，失败不阻断后续
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
       ├─ hardware → loadHardwarePage()  → refreshHardwareInfo
       ├─ ip       → refreshLanConfig()  → send_raw_at（按芯片走 `AT+QMAP="LANIP"` / `AT+QCFG="lanip"` / `AT+QCFG="lanip_ex"`）
       ├─ scene    → loadScenePage()     → 并发拉 feature_toggles / nat_mode / usbnet_mode
       └─ atmanual → initAtdbPage()      → 纯前端 AT_DB 索引
```

## 3. 通用 IPC 调用链（标准模式）

```
[前端]  invoke('command_name', args)
  │
  ▼
[Tauri runtime]  路由到 #[tauri::command] 函数
  │
  ▼
[handlers.rs / connection.rs 等 handler]  with_vendor! 宏展开
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
[LoggingTransport::send_at]
  │
  ├─ inner.send_at(cmd)  ← 真实 transport 先发送
  │
  └─ 发送完成后写环形日志（成功或错误都会 redact）
       │
       └─ Result<String, String>  ← 沿原路径回前端（JSON 序列化）
```

## 4. 状态查询流程（`get_modem_status`）

```
invoke('get_modem_status')
  │
  └─→ handlers.rs::get_modem_status  [with_vendor! 宏]
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
                  │     └─ reg_status ← serving_cell.mobility_state（非独立 AT+CEREG?）
                  ├─ AT+COPS?           → parse_cops_with_act  → operator
                  ├─ AT+QRSRP (Qual)    → parse_qrsrp          → ant[4]
                  │  AT+QANTRSSI? (UniS)
                  ├─ AT+CGACT? (UniSoc) → parse_cgact          → active_cids, conn_status
                  ├─ AT+QMAP="MPDN_status" (Qualcomm) → parse_mpdn_connect_status → conn_status
                  │
                  └─→ ModemStatus { sim, imei, iccid, operator, ant[], active_cids, ... }
```

## 5. 数据连接流程

```
invoke('connect_data', { cid: 1 })
  │
  └─→ handlers.rs::connect_data  [with_vendor_cid! 宏]
          │
          └─→ vendor.connect_data(&mut transport, cid)
                  │
                  ├─ QuectelModem (Qualcomm) → qualcomm::connect_data
                  │     AT+QMAP="connect",<cid>,1  → is_ok 检查
                  │
                  ├─ QuectelModem (UniSoc) → unisoc::connect_data
                  │     AT+QNETDEVCTL=<cid>,1,1  → is_ok 检查
```

## 6. IP 查询流程

```
invoke('get_ip_info', { cid: 1 })
  │
  └─→ handlers.rs::get_ip_info  [with_vendor_cid! 宏]
          │
          └─→ vendor.query_ip_info(&mut transport, cid)
                  │
                  ├─ Qualcomm  → qualcomm::query_ip_info
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
  └─→ vendor.query_bands_with_spec
        │
        ├─ AT+QNWPREFCFG=?            → parse_qnwprefcfg_supported (lte_supported, nr_supported)
        ├─ AT+QNWPREFCFG="lte_band"   → parse_qnwprefcfg_bands → lte_locked
        └─ AT+QNWPREFCFG="nr5g_band"  → parse_qnwprefcfg_bands → nr_locked

invoke('set_bands', { lte: "1:3:5", nr: "78:79" })
  │
  └─→ vendor.set_lte_bands / set_nr5g_bands
        ├─ AT+QNWPREFCFG="lte_band",1:3:5     ← 频段号纯数字、冒号分隔、无额外引号
        └─ AT+QNWPREFCFG="nr5g_band",78:79

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
        ├─ AT+QCFG="eth_at"      → parse_qcfg_int → eth_at  （仅 Qualcomm；UniSoc 硬编码 false）
        ├─ AT+QCFG="usbcfg"      → parse_qcfg_usbcfg_adb → adb
        ├─ AT+QCFG="napt"        → parse_qcfg_int → napt
        └─ AT+QCFG="netmask"     → parse_qcfg_int → netmask

  任一 live 读取或解析失败 → 直接返回错误；不再把失败伪装成 false / 0

invoke('set_feature_toggle', { feature: "adb", enabled: true })
  │
  └─→ handlers.rs::set_feature_toggle
        │
        ├─ feature="adb" → 读当前 usbcfg，修改最后一位，写回
        └─ 其它          → AT+QCFG="<feature>",<0|1>
```

## 10. AT 调试页（`send_raw_at`）

> 本路径发送完整 AT 命令，使用 `validate_raw_at_command` 校验；不要复用只面向参数的 `validate_at_string`。

```
[app.js]  invoke('send_raw_at', { command: "AT+XXX" })
  │
  └─→ handlers.rs::send_raw_at
        │
        └─→ tokio::task::spawn_blocking
              │
              └─→ state.transport.lock()?.send_at(cmd)
                    │
                    ├─ inner.send_at(cmd)  ← transport 先发送
                    └─ [LoggingTransport] 发送完成后写 redact 日志
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

## 13. ADB 调试流程

### 13.1 页面进入与 ADB 开关检查

```
[前端]  用户进入 ADB 调试页
  │
  └─→ initDebugTerminal()  [debug-terminal.js]
        │
        ├─ invoke('get_debug_terminal_capabilities')
        │     └─→ debug_terminal.rs::get_debug_terminal_capabilities
        │           └─ Windows 才返回 adb_supported=true
        │
        ├─ invoke('get_debug_terminal_prefs')
        └─ handleDebugTerminalPageChange(...)
              └─ ensureAdbEnabled()
                    └─ invoke('get_feature_toggles')
                          └─ adb=false → 仅提示“请先开启 ADB 并重启设备后重新连接”
```

### 13.2 建立 ADB shell 会话

```
[前端]  connectAdbDebug()
  │
  └─ invoke('start_adb_session')
        └─→ debug_terminal.rs::start_adb_session
              ├─ 检查当前无活动调试会话
              ├─ 解析 Tauri `resourceDir()/adb/adb.exe`
              ├─ Windows 下启动 `adb.exe shell`
              └─ 后台线程循环读取 stdout/stderr
                    └─ emit('debug-terminal-output', { kind: 'adb', stream, text })
```

### 13.3 ADB 终端输入 / 断开 / 离开页面

```
[前端]  用户点击终端后直接按键输入
  └─→ handleDebugTerminalKey(event, "adb")
        └─ invoke('write_debug_terminal_input', { input })
              └─→ debug_terminal.rs::write_debug_terminal_input
                    └─ 写入 adb child stdin

[前端]  点击“断开”
  └─→ disconnectDebugTerminal("adb")
        └─ invoke('close_debug_terminal_session')
              └─→ 关闭 adb shell 子进程

[后端]  adb shell 退出
  └─ emit('debug-terminal-output', { kind: 'adb', stream: 'system', text: 'ADB shell 已退出 ...' })

[前端]  handleDebugOutputEvent()
  └─→ 按 system 文本把按钮状态恢复成“连接 / 断开禁用”

[前端]  离开 ADB/SSH 页
  └─→ handleDebugTerminalPageChange(prev, next)
        └─ invoke('close_debug_terminal_session')
              └─→ 关闭活动调试子进程 / SSH channel
```

## 14. SSH 调试流程

### 14.1 网卡枚举与默认 IP

```
[前端]  用户进入 SSH 调试页
  │
  └─→ refreshSshDebugAdapters()  [debug-terminal.js]
        │
        ├─ invoke('list_debug_network_adapters')
        │     └─→ debug_terminal.rs::list_debug_network_adapters
        │           └─ 复用 connection.rs::list_network_adapters()，再过滤掉 Wi-Fi / WLAN
        │
        ├─ 用 adapter.gateway 填充下拉框 value
        └─ handleSshAdapterChange()
              └─ 默认把 `#sshHost` 设为所选网卡网关
```

### 14.2 建立 SSH shell 会话

```
[前端]  connectSshDebug()
  │
  ├─ invoke('save_debug_terminal_prefs')
  │     └─ 仅保存用户名 / 上次网卡 / 上次 IP，不保存密码
  │
  └─ invoke('start_ssh_session', { host, username, password })
        └─→ debug_terminal.rs::start_ssh_session
              ├─ TcpStream::connect(host:22)
              ├─ ssh2::Session::handshake()
              ├─ userauth_password()
              ├─ channel_session().request_pty("xterm")
              ├─ shell()
              └─ 后台线程读取 channel 输出
                    └─ emit('debug-terminal-output', { kind: 'ssh', stream, text })
```

### 14.3 SSH 终端输入 / 断开

```
[前端]  用户点击终端后直接按键输入
  └─→ handleDebugTerminalKey(event, "ssh")
        └─ invoke('write_debug_terminal_input', { input })
              └─→ debug_terminal.rs::write_debug_terminal_input
                    └─ 通过 channel stdin 写入远端 shell

[前端]  点击“断开”
  └─→ disconnectDebugTerminal("ssh")
        └─ invoke('close_debug_terminal_session')
              └─→ 关闭 SSH shell channel
```

## 15. 固件下载流程

### 15.1 PAC 选择与分析

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

### 15.2 下载执行

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

### 15.3 事件处理（前端）

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

### 15.4 三层架构

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

