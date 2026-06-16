# 项目架构设计

> 适用于：modem-cat v0.2.x

> 最近更新：2026-06-16

## 1. 整体架构

```
┌──────────────────────────────────────────────────────┐
│ 前端 (src/desktop/)                                  │
│   index.html  ← UI 结构（8 个 page）                 │
│   styles.css  ← 主题样式（3 主题：dark/light/blue-light）│
│   app.js      ← 全部交互逻辑                         │
│                                                      │
│  ⚠ Tauri 只加载 index.html 本身；styles.css 和 app.js │
│  必须通过 <link>/<script src=> 显式引用               │
└──────────────────────────────────────────────────────┘
                       │ window.__TAURI__.core.invoke()
                       │ window.__TAURI__.event.listen()
                       ▼
┌──────────────────────────────────────────────────────┐
│ Tauri 后端 (src-tauri/src/)                          │
│   main.rs        ← 入口（设置 NO_PROXY）              │
│   lib.rs         ← Tauri Builder 装配 + AppState      │
│                     + LoggingTransport + IPC commands  │
│                     + start_port_monitor              │
└──────────────────────────────────────────────────────┘
                       │ Box<dyn ModemVendor>
                       ▼
┌──────────────────────────────────────────────────────┐
│ 共享 HAL (modem-hal/src/)                            │
│   modem_vendor.rs   ← ModemVendor trait              │
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

### 3.1 前端 (`src/desktop/`) — 10 个 page 容器

> ⚠ **编辑约定**：修改前端时优先改外部 `app.js` / `styles.css`，而不是把它们再粘回 `index.html` 内联块。若新增第四个主题或新的 IPC，需在本文档 §3.4 增加条目。

| Page | 中文名 | 备注 |
|---|---|---|
| `#page-status` | 模组状态 | 默认 active，进 `doInit` 拉全部数据 |
| `#page-cellular` | 蜂窝网络 | 含 APN / 网络配置 / 小区锁 / 邻区 / 5GLAN 五个子 tab |
| `#page-at` | AT 调试 | AT 终端主体 |
| `#page-hardware` | 系统信息 | 含 unisoc / qualcomm 两个子 tab |
| `#page-settings` | 系统设置 | 语言 / 主题 |
| `#page-ip` | IP 信息 | MTU / DMZ / LAN 配置 |
| `#page-scene` | 情景模式 | UniSoc / Qualcomm 场景切换 |
| `#page-atmanual` | AT 手册速查 | 静态 AT_DB 数据，无 IPC |
| `#page-factory` | 工厂模式 | License 控制显示；含生产操作 / 产品配置 / 生产记录三个子 tab |
| `#page-firmware` | 固件下载 | License 控制显示；PAC 选择 / 安全策略 / 下载控制 / 日志 |

前端状态：→ `AGENTS.md §2`。`$.dom` 在 `cacheDom()` 函数中（app.js 启动段）。

### 3.2 Tauri 后端 (`src-tauri/src/`)

| 文件 | 职责 |
|---|---|
| `lib.rs` | Tauri Builder 装配；`AppState`；`LoggingTransport` 装饰器；IPC commands（`invoke_handler!` 块注册）；`start_port_monitor` |
| `main.rs` | 入口；`NO_PROXY=tauri.localhost,localhost,127.0.0.1` 兜底 |
| `license.rs` | License 状态管理、文件加载、IPC 命令（`get_license_status` / `load_license_file`） |
| `factory.rs` | 工厂模式：SN 生成、HTTP 设备客户端、CSV 持久化、21 个 IPC 命令 |
| `dloader.rs` | 固件下载：PAC 安全分析、r26-cli sidecar 管理、事件转发、4 个 IPC 命令 |

**AppState 字段**：

```rust
pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,   // 真实通道（含 LoggingTransport 装饰）
    pub vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,      // 厂商驱动
    pub data_cid: Arc<Mutex<i32>>,                              // 当前 PDP CID
    pub connected_port: Arc<Mutex<Option<String>>>,             // 已连接串口名（用于 USB 拔插判断）
    pub at_command_log: Arc<Mutex<VecDeque<String>>>,           // 1000 条环形日志
    pub license: Arc<Mutex<Option<LicenseStatus>>>,             // License 状态
}
```

**其他 Tauri 管理状态**：

| 状态类型 | 模块 | 用途 |
|---|---|---|
| `FactoryState` | factory.rs | 工厂模式配置、当前产品、SN 序号、设备 IP |
| `DloaderState` | dloader.rs | 固件下载 sidecar 进程句柄（用于停止下载） |

### 3.3 HAL (`modem-hal/src/`)

| 文件 | 职责 |
|---|---|
| `lib.rs` | `validate_at_string` / `validate_raw_at_command` / `validate_cid` 校验 |
| `modem_vendor.rs` | `ModemVendor` trait 定义 |
| `modem_factory.rs` | `ModemFactory::create()` —— AT+CGMM → ChipsetVendor → ModemVendor |
| `types.rs` | 所有共享结构体 |
| `transport/mod.rs` | `AtTransport` trait + `redact_at_command` + `MockTransport` |
| `transport/serial.rs` | `SerialTransport`（serialport v4） |
| `transport/tcp.rs` | `TcpTransport`（BufReader） |
| `vendors/quectel/mod.rs` | `QuectelModem`（含 `QuectelChip` 二态字段） |
| `vendors/quectel/parser.rs` | 80+ pure 解析函数 |
| `vendors/quectel/qualcomm.rs` | 高通数据连接 / IP / 5GLAN |
| `vendors/quectel/unisoc.rs` | 展锐数据连接 / IP |
| `vendors/quectel/band_db.rs` | UniSoc 硬件频段静态表 |

### 3.4 前端主题系统（`src/desktop/styles.css`）

3 个主题由 `documentElement[data-theme="…"]` 切换，token 均为 CSS 自定义属性。`app.js::setTheme(theme)` 是唯一写入方（写 `data-theme` 属性 + `localStorage.theme`），`updateThemeToggle(theme)` 接收参数同步设置面板的 `.active` 状态。

| 主题 | data-theme | accent | 字体 | 何时为默认 |
|---|---|---|---|---|
| 深色（暖深） | `dark` | `#F97316` 焦糖橙 | `'Inter', ...` | `localStorage.theme` 缺失或不在白名单 |
| 浅色（暖米白） | `light` | `#EA580C` 焦糖橙 | 同上 | 用户在设置页选"浅色" |
| 科技蓝 | `blue-light` | `#0f62fe` IBM 蓝 | 同上 | 用户在设置页选"科技蓝" |

**实现要点**：

- **持久化**：localStorage key `theme`，值 `dark` / `light` / `blue-light`，三选一。设置页 `app.js::setTheme(theme)` 是唯一写入点。
- **emoji 字体栈**：`body` 已加入 `'Segoe UI Emoji', 'Apple Color Emoji', 'Noto Color Emoji'` 以避免"🌐"按钮在 Win10 / Linux WebView 上 tofu。
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
  │    1. RG500Q/RM500Q/RG520N/RM520N/RG525F/RG530F/
  │       RM530F/RM530N/RM551E/RM501Q/RG540F/RM540N → Qualcomm
  │    2. RG200U/RM500U/RG500U/RG501U/RM501U        → UniSoc
  │    3. Unknown → **返回 Err**，无默认兜底
  │
  └─ create_from_vendor(model, vendor)
       ├─ Qualcomm  → QuectelModem::qualcomm(model)
       └─ UniSoc    → QuectelModem::unisoc(model)
```

## 5. 多平台命令差异（核心业务）

### 5.1 数据连接

| 平台 | 连接 | 断开 | IP 查询 | 流量 | 重置流量 |
|---|---|---|---|---|---|
| **Qualcomm** | `AT+QMAP="connect",<cid>,1` | `AT+QMAP="connect",<cid>,0` | `AT+QMAP="MPPDN_status"` + `CGPADDR` | `AT+QGDNRCNT?` | `AT+QGDNRCNT=0` |
| **UniSoc** | `AT+CGACT=1,<cid>` | `AT+CGACT=0,<cid>` | `AT+QNETDEVSTATUS=<cid>` | `AT+QGDCNT?` | `AT+QGDCNT=0` |

### 5.2 频段锁

| 平台 | 命令 | 参数 |
|---|---|---|
| **Qualcomm** | `AT+QNWLOCK="common/5g",<pci>,<arfcn>,<scs>,<band>` | **5 参数**，pci 在前 |
| **UniSoc** | `AT+QNWLOCK="common/5g",1,<arfcn>,<pci>` | **4 参数**，arfcn 在前 |
| | `AT+QNWLOCKFREQ="common/5g",1,<arfcn>` | UniSoc 额外支持纯频点锁 |

### 5.3 流量命令字符级差异

`QGDCNT`（UniSoc，注意 G 和 D 之间没 R）vs `QGDNRCNT`（Qualcomm，**多个 R**）。
一字之差就会失败，每次提交要二次确认。

## 6. 后台线程

| 线程 | 文件 | 间隔 | 锁策略 | 作用 |
|---|---|---|---|---|
| `usb-monitor` | lib.rs | 2s | 无锁（只读 `available_ports`） | `serialport::available_ports()` 差集 → emit `port-changed` |
| `connection-heartbeat` | lib.rs | 4s | **try_lock**（拿不到就 skip）| `transport.is_alive()` → 拔插 emit `port-changed` |

## 7. 关键 Trait 一览

### `AtTransport`（最小化，3 方法）

```rust
pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
    fn is_alive(&self) -> bool { true }   // 默认 true
}
```

### `ModemVendor`（业务级）

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
- 端口变化：USB 拔插通过 `port-changed` 事件通知前端

## 9. 添加新 vendor 的步骤

1. `modem-hal/src/vendors/<name>/mod.rs` 实现 `ModemVendor` trait
2. `modem-hal/src/modem_factory.rs::detect_vendor_from_model` 加关键字 + `create_from_vendor` 加分支
3. 复制 `quectel/` 目录结构，填好 AT 命令和 parser
4. 在前端 `app.js` 厂商判断处加新分支（用 `state.chipVendor`）
5. 至少加 3 个 unit test（model detection / connect / parse）

## Source Of Truth

- `ARCHITECTURE.md` — 架构、数据流、模块职责、线程模型（唯一权威）
- `CODE_MAP.md` — IPC 命令 × 前端触发点 × AT 解析函数 三列映射
- `AT_COMMANDS.md` — AT 命令→解析函数→平台差异（与 `modem_factory.rs` 厂商检测严格对齐）
- `CALL_FLOW.md` — IPC 调用全链路时序图
- `REVIEW.md` — 已知问题清单与修复状态
- `BUILD.md` — 构建命令与平台注意事项
- `TECH_STACK.md` — 技术栈与依赖

## Doc Owner

- `ARCHITECTURE.md` — 架构 owner，重大重构必须同步更新
- `AT_COMMANDS.md` — AT 命令 owner，新增/修改 AT 命令必须同步更新
- `CODE_MAP.md` — IPC 映射 owner，增删 IPC 命令必须同步更新
- `REVIEW.md` — 技术债追踪，修复后更新状态
- 环境、部署、测试命令等操作性信息，更新对应 owner 文档
