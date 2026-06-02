# 项目架构设计

> 最近更新：2026-06-02（commands.rs 死代码删除 · heartbeat 改 try_lock · 前端拆分落实 · 蓝 light 主题）
> 适用于：modem-cat v0.2.x

## 1. 整体架构

```
┌──────────────────────────────────────────────────────┐
│ 前端 (src/desktop/)                                  │
│   index.html  ← UI 结构（8 个 page，~1500 行）        │
│   styles.css  ← 主题样式（3 主题：dark/light/blue-light）│
│   app.js      ← 全部交互逻辑（~3600 行）              │
│                                                      │
│  ⚠ Tauri 只加载 index.html 本身；styles.css 和 app.js │
│  必须通过 <link>/<script src=> 显式引用，否则 0 字节  │
│  对运行生效（早期 e26df93 split 之后曾长期处于这种   │
│  孤儿状态，commit 4f2fb83 才真正接上）。              │
└──────────────────────────────────────────────────────┘
                       │ window.__TAURI__.core.invoke()
                       │ window.__TAURI__.event.listen()
                       ▼
┌──────────────────────────────────────────────────────┐
│ Tauri 后端 (src-tauri/src/)                          │
│   main.rs        ← 入口（设置 NO_PROXY）              │
│   lib.rs         ← Tauri Builder 装配 + AppState      │
│                     + LoggingTransport + 52 IPC cmd   │
│   ports.rs       ← 串口列表探测 (Windows 注册表)       │
│   monitor.rs     ← start_port_monitor 后台线程         │
│                                                      │
│  (历史) commands.rs (504 行) 2026-06-02 已删除        │
│   —— 旧 IPC 层，0 caller，.unwrap() 64 处             │
└──────────────────────────────────────────────────────┘
                       │ Box<dyn ModemVendor>
                       ▼
┌──────────────────────────────────────────────────────┐
│ 共享 HAL (modem-hal/src/)                            │
│   modem_vendor.rs   ← ModemVendor trait (**62 个方法**)    │
│   modem_factory.rs  ← ModemFactory::create()          │
│                      (CGMM 型号 → ChipsetVendor)      │
│   types.rs          ← 共享数据结构（无 spec_bands 表，运行时靠 AT+QNWPREFCFG="rf_band" 拿硬件支持频段）    │
│   transport/                                           │
│     mod.rs    ← AtTransport trait + redact_at_command │
│     serial.rs ← SerialTransport (serialport v4)       │
│     tcp.rs    ← TcpTransport (BufReader<TcpStream>)   │
│   vendors/                                             │
│     quectel/                                          │
│       mod.rs       ← QuectelModem (chip 二态字段)     │
│       parser.rs    ← 80+ pure 解析函数               │
│       qualcomm.rs  ← 高通数据连接/IP/5GLAN            │
│       unisoc.rs    ← 展锐数据连接/IP                  │
│       band_db.rs   ← 硬件频段表                       │
│     tdtech/                                           │
│       mod.rs       ← TdTechModem (AT^ 前缀)           │
│       parser.rs    ← parse_monsc/hcsq/syscfgex        │
│       dial.rs      ← AT^NDISDUP/AT^DHCP               │
└──────────────────────────────────────────────────────┘
                       │ 串口 / TCP
                       ▼
                 5G Modem (USB / Ethernet / TTL)
```

## 2. 关键数据流：一次 IPC 调用

以 `invoke('connect_data', { cid: 1 })` 为例：

```
[前端] `toggleDataConnection` 函数（app.js `connect_data` 调用点）  invoke('connect_data', { cid: 1 })
  │
  ▼
[Tauri runtime] 路由到 #[tauri::command] 函数
  │
  ▼
[lib.rs:604]  async fn connect_data(state, cid) -> Result<(), String>
  │
  ├─ with_vendor_cid! 宏展开：
  │    tokio::task::spawn_blocking(move || {
  │        let mut tguard = state.transport.lock()?;     ← std::sync::Mutex
  │        let mut vguard = state.vendor.lock()?;
  │        let t = tguard.as_deref_mut().ok_or("Not connected")?;
  │        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
  │        v.connect_data(t, cid)                        ← ModemVendor trait
  │    })
  │
  ▼
[QuectelModem]  match self.chip {
       QuectelChip::Qualcomm => qualcomm::connect_data(t, cid),
       QuectelChip::UniSoc   => unisoc::connect_data(t, cid),
   }
  │
  ├─ Qualcomm  → vendors/quectel/qualcomm.rs:5
  │             AT+QMAP="connect",<cid>,1
  │
  └─ UniSoc    → vendors/quectel/unisoc.rs:4
                AT+CGACT=1,<cid>
  │
  ▼
[LoggingTransport::send_at]   lib.rs:33
  │
  ├─ log.push(redact_at_command(cmd))    ← 进 1000 条环形日志
  │
  └─ inner.send_at(cmd)                  ← 真 SerialTransport
       │
       ├─ write_all(cmd + "\r\n")
       ├─ flush()
       └─ read_response()  循环读直到 OK / ERROR / +CME ERROR (8s 超时)
            │
            ▼
       返回 Result<String, String>  ← 沿原路径回前端
```

**典型耗时**：3-8s（受 `RESPONSE_OVERALL=8s` 和读超时影响）

## 3. 模块职责

### 3.1 前端 (`src/desktop/`) — 8 个 page 容器

> ✅ **当前状态（2026-06-02 之后）**：`index.html` 通过 `<link rel="stylesheet" href="styles.css">` 和 `<script src="app.js"></script>` 真正加载了两个外部文件（commit `4f2fb83`）。早期版本的"看起来是单文件 + 两个孤儿副本"是 e26df93 split 残留，已修复。
>
> ⚠ **编辑约定**：修改前端时优先改外部 `app.js` / `styles.css`，而不是把它们再粘回 `index.html` 内联块。若新增第四个主题或新的 IPC，需在本文档 §3.4 增加条目。

| 行号 | Page | 中文名 | 备注 |
|---|---|---|---|
| 1674 | `#page-status` | 模组状态 | 默认 active，进 `doInit` 拉全部数据 |
| 2061 | `#page-cellular` | 蜂窝网络 | 含 APN / 网络配置 / 小区锁 / 邻区 / 5GLAN 五个子 tab |
| 2327 | `#page-at` | AT 调试 | AT 终端主体 |
| 2359 | `#page-hardware` | 系统信息 | 含 unisoc / qualcomm 两个子 tab |
| 2705 | `#page-settings` | 系统设置 | 语言 / 主题 |
| 2754 | `#page-ip` | IP 信息 | MTU / DMZ / LAN 配置 |
| 2849 | `#page-scene` | 情景模式 | UniSoc / Qualcomm 场景切换 |
| 2879 | `#page-atmanual` | AT 手册速查 | 静态 AT_DB 数据，无 IPC |

全局状态：单一 `state` 对象（app.js 顶部 `let state = { ... }`，行号随 commit 漂移；**勿引用具体行号**），无框架；`$.dom` 在 `cacheDom()` 函数中（app.js 启动段）。

### 3.2 Tauri 后端 (`src-tauri/src/`)

| 文件 | 行数 | 职责 |
|---|---|---|
| `lib.rs` | ~1140 | Tauri Builder 装配；`AppState`；`LoggingTransport` 装饰器；**52 个 IPC 命令**（`invoke_handler!` 块注册）；`start_port_monitor`（**与 monitor.rs 重复**） |
| `ports.rs` | 11.9K | 串口列表探测；Windows 注册表读取友好名；`is_at_port()` 关键字判断 |
| `monitor.rs` | 2.6K | `start_port_monitor` 独立线程（2s 轮询） |
| `main.rs` | 0.3K | 入口；`NO_PROXY=tauri.localhost,localhost,127.0.0.1` 兜底 |

> 历史：`commands.rs` (504 行死代码) 2026-06-02 已删除（见 [REVIEW.md#1](REVIEW.md) 状态 `✅ FIXED`）。

**AppState 字段**（lib.rs:15-25）：

```rust
pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,   // 真实通道（含 LoggingTransport 装饰）
    pub vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,      // 厂商驱动
    pub data_cid: Arc<Mutex<i32>>,                              // 当前 PDP CID
    pub connected_port: Arc<Mutex<Option<String>>>,             // 已连接串口名（用于 USB 拔插判断）
    pub at_command_log: Arc<Mutex<Vec<String>>>,                // 1000 条环形日志（待改 VecDeque）
}
```

### 3.3 HAL (`modem-hal/src/`)

| 文件 | 行数 | 职责 |
|---|---|---|
| `lib.rs` | 3.7K | `validate_at_string` / `validate_raw_at_command` / `validate_cid` 校验；napi-rs 暴露（feature gate） |
| `modem_vendor.rs` | 481 | `ModemVendor` trait 定义（**62 个方法**） |
| `modem_factory.rs` | 4.3K | `ModemFactory::create()` —— AT+CGMM → ChipsetVendor → ModemVendor |
| `types.rs` | 6.5K / 252 行 | 所有共享结构体（`spec_bands_for_model()` 已在重构中删除，见 [MODEM_BAND_SPECS.md](MODEM_BAND_SPECS.md)） |
| `transport/mod.rs` | 2.6K | `AtTransport` trait + `redact_at_command` + `MockTransport` |
| `transport/serial.rs` | 6.3K | `SerialTransport`（serialport v4） |
| `transport/tcp.rs` | 3K | `TcpTransport`（BufReader） |
| `vendors/quectel/mod.rs` | 1063 | `QuectelModem`（含 `QuectelChip` 二态字段） |
| `vendors/quectel/parser.rs` | 1260 | 80+ pure 解析函数（基础 / 网络 / QENG / 流量 / QNWPREFCFG / QMAP） |
| `vendors/quectel/qualcomm.rs` | 12.6K | 高通数据连接 / IP / 5GLAN |
| `vendors/quectel/unisoc.rs` | 4.2K | 展锐数据连接 / IP |
| `vendors/quectel/band_db.rs` | - | 硬件频段表 |
| `vendors/tdtech/mod.rs` | 429 | `TdTechModem`（AT^ 前缀命令） |
| `vendors/tdtech/parser.rs` | 7.9K | `parse_monsc` / `parse_hcsq` / `parse_syscfgex` |
| `vendors/tdtech/dial.rs` | 2.4K | `AT^NDISDUP` / `AT^DHCP` |

### 3.4 前端主题系统（`src/desktop/styles.css`）

3 个主题由 `documentElement[data-theme="…"]` 切换，token 均为 CSS 自定义属性。`app.js::setTheme(theme)` 是唯一写入方（写 `data-theme` 属性 + `localStorage.theme`），`updateThemeToggle(theme)` 接收参数同步设置面板的 `.active` 状态。

| 主题 | data-theme | accent | 字体 | 何时为默认 |
|---|---|---|---|---|
| 深色（暖深） | `dark` | `#F97316` 焦糖橙 | `'Inter', ...` | `localStorage.theme` 缺失或不在白名单 |
| 浅色（暖米白） | `light` | `#EA580C` 焦糖橙 | 同上 | 用户在设置页选"浅色" |
| 科技蓝 | `blue-light` | `#0f62fe` IBM 蓝 | 同上 | 用户在设置页选"科技蓝"（commit `d182cc7`） |

**实现要点**：

- **三块选择器特异性均为 (0,1,0)**（一个属性选择器）。`:root` 兜底是 (0,0,1)。同一主题内，蓝 light 块不需要 `!important` 即可在 cascade 中胜出（`blue-light` 块已在 2026-06-02 去掉 20 个 `!important`）。`light` / `dark` 块仍保留 `!important` 是因为它们和 `:root` 共享选择器 `(:root, [data-theme="light"])`，去 `!important` 需要拆开选择器，超出本批范围。
- **持久化**：localStorage key `theme`，值 `dark` / `light` / `blue-light`，三选一。设置页 `app.js::setTheme(theme)` 是唯一写入点。
- **emoji 字体栈**：`body` 已加入 `'Segoe UI Emoji', 'Apple Color Emoji', 'Noto Color Emoji'`（commit `4f2fb83` 后续补）以避免"🌐"按钮在 Win10 / Linux WebView 上 tofu。
- **新增主题的步骤**：
  1. `styles.css` 加 `[data-theme="<name>"]` 块（参考蓝 light 模板）
  2. `app.js::setTheme` 加 emoji / 图标分支（如有）
  3. `app.js::updateThemeToggle` 加按钮 ID + `classList.toggle('active', theme === '<name>')`
  4. `index.html` 设置页加 `<button id="theme<Name>" onclick="setTheme('<name>')">` 按钮
  5. `app.js` LANG 字典加 `theme_<name>` i18n key（中英）
  6. 更新本节表格 + [REVIEW.md](REVIEW.md) 状态

## 4. 厂商检测流程

```
[lib.rs::auto_connect_at]  串行扫描所有 AT 候选端口
  │
  ▼
[SerialTransport::probe_at]  发 AT\r\n, 800ms 等 OK
  │
  ▼
[ModemFactory::create(transport)]
  │
  ├─ query_model(transport)  → AT+CGMM → parse_cgmm → model: String
  │
  ├─ detect_vendor_from_model(model)  ← 大写子串匹配，**优先级**：
  │    1. "MT5700"        → TdTech
  │    2. RG500Q/RM500Q/RG520N/RM520N/RG525F/RG530F/
  │       RM530F/RM530N/RM551E/RM501Q/RG540F/RM540N → Qualcomm
  │    3. RG200U/RM500U/RG500U/RG501U/RM501U        → UniSoc
  │    4. Unknown → **返回 Err**，无默认兜底
  │
  └─ create_from_vendor(model, vendor)
       ├─ TdTech    → TdTechModem { model }
       ├─ Qualcomm  → QuectelModem::qualcomm(model)
       └─ UniSoc    → QuectelModem::unisoc(model)
```

> **风险点**：当前关键字冲突风险低（`RM500Q` vs `RM500U` 靠末尾字母），但若未来加 `RG500UA` 之类型号需补单元测试。

## 5. 多平台命令差异（核心业务）

### 5.1 数据连接

| 平台 | 连接 | 断开 | IP 查询 | 流量 | 重置流量 |
|---|---|---|---|---|---|
| **Qualcomm** | `AT+QMAP="connect",<cid>,1` | `AT+QMAP="connect",<cid>,0` | `AT+QMAP="MPPDN_status"` + `CGPADDR` | `AT+QGDNRCNT?` | `AT+QGDNRCNT=0` |
| **UniSoc** | `AT+CGACT=1,<cid>` | `AT+CGACT=0,<cid>` | `AT+QNETDEVSTATUS=<cid>` | `AT+QGDCNT?` | `AT+QGDCNT=0` |
| **TdTech** | `AT^NDISDUP=1,<cid>` | `AT^NDISDUP=<cid>,0` | `AT^DHCP=<cid>` (IP 是 hex) | （未实现） | （未实现） |

### 5.2 频段锁

| 平台 | 命令 | 参数 |
|---|---|---|
| **Qualcomm** | `AT+QNWLOCK="common/5g",<pci>,<arfcn>,<scs>,<band>` | **5 参数**，pci 在前 |
| **UniSoc** | `AT+QNWLOCK="common/5g",1,<arfcn>,<pci>` | **4 参数**，arfcn 在前 |
| | `AT+QNWLOCKFREQ="common/5g",1,<arfcn>` | UniSoc 额外支持纯频点锁 |
| **TdTech** | `AT^SYSCFGEX?` + `parse_syscfgex` | 频段编码差异大 |

### 5.3 流量命令字符级差异

`QGDCNT`（UniSoc，注意 G 和 D 之间没 R）vs `QGDNRCNT`（Qualcomm，**多个 R**）。
一字之差就会失败，每次提交要二次确认。

## 6. 后台线程

| 线程 | 文件 | 间隔 | 锁策略 | 作用 |
|---|---|---|---|---|
| `usb-monitor` | monitor.rs:13 / lib.rs:869（**重复**） | 2s | 无锁（只读 `available_ports`） | `serialport::available_ports()` 差集 → emit `port-changed` |
| `connection-heartbeat` | lib.rs:929 | 4s | **try_lock**（拿不到就 skip）| `transport.is_alive()` → 拔插 emit `port-changed` |

> 2026-06-02 修复：heartbeat 原用 `.lock()` 阻塞等待，与 IPC handler 抢同一把 std Mutex，单条 AT 命令（3-8s）期间会阻塞 4-12s，USB 拔插感知延迟。改为 `.try_lock()` 拿不到直接 `continue`（下一 tick 再试），不再阻塞 IPC。

## 7. 关键 Trait 一览

### `AtTransport`（最小化，3 方法）

```rust
pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
    fn is_alive(&self) -> bool { true }   // 默认 true
}
```

### `ModemVendor`（业务级，**62 个方法**）

按分类列出（详见 [CODE_MAP.md](CODE_MAP.md)）：

- **基础信息**：`query_sim_status` / `query_imei` / `query_iccid` / `query_hardware_info` / `query_temperature`
- **网络信息**：`query_serving_cell` / `query_neighbor_cells` / `query_signal_strength` / `query_operator` / `query_registration_status` / `query_connection_status`
- **APN & 数据连接**（必须实现）：`query_apn_list` / `set_apn` / `delete_apn` / `set_apn_active` / `connect_data` / `disconnect_data` / `query_ip_info`
- **5GLAN / VLAN**（default `Err`）：`query_vlan` / `set_vlan` / `query_5glan` / `set_5glan` / `configure_qualcomm_5glan` / `enable_eth_pdu` / `connect_qualcomm_5glan` / `query_qualcomm_5glan_status`
- **频段**：`query_band_config` / `set_lte_bands` / `set_nr5g_bands` / `set_nsa_nr5g_bands` (default `Err`) / `set_network_mode`
- **流量/特性/电源**：`query_traffic` / `reset_traffic` / `query_feature_toggles` / `set_feature_toggle` / `reboot`

## 8. 运行模式

- **仅 Desktop 模式**。原 CLI 模式已移除。
- Tauri v2 + `withGlobalTauri: true`：前端直接用 `window.__TAURI__.core.invoke`
- 系统托盘：关闭窗口隐藏到托盘（`on_window_event` 拦截）；右键菜单"控制面板 / 退出"
- 启动时自动扫描 AT 端口并连接（`doInit → toggleConnection → auto_connect_at`）
- 端口变化：USB 拔插 4s 内通过 `port-changed` 事件通知前端（Windows 可靠，macOS 可能更慢，见 REVIEW.md#7）

## 9. 添加新 vendor 的步骤

1. `modem-hal/src/vendors/<name>/mod.rs` 实现 `ModemVendor` trait
2. `modem-hal/src/modem_factory.rs::detect_vendor_from_model` 加关键字 + `create_from_vendor` 加分支
3. 复制 `tdtech/dial.rs` 和 `tdtech/parser.rs` 模板，填好 AT 命令
4. 在前端 `app.js` 厂商判断处加新分支（用 `state.chipVendor`）
5. 至少加 3 个 unit test（model detection / connect / parse）
