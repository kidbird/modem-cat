# 项目架构设计

> 最近更新：2026-06-18
> 适用于：modem-cat 当前主线

## 1. 总体结构

```text
src/desktop/
  index.html
  styles.css
  js/{core,i18n,theme,scene}.js
  data/atdb.js
  app.js
        │  window.__TAURI__.core.invoke / event.listen
        ▼
src-tauri/src/
  main.rs      ← 入口（NO_PROXY）
  lib.rs       ← live IPC / AppState / queue / monitor / tray / handler 注册
  mqtt.rs      ← 可选 MQTT 后台状态上报
  license.rs   ← License 加载 / 校验 / IPC
  factory.rs   ← 工厂模式 HTTP 设备通信 / 本地持久化 / IPC
  dloader.rs   ← 固件下载 sidecar / 事件转发 / IPC
        │
        ├─ modem-hal/src/*
        │    transport/{serial,tcp,websocket}.rs
        │    modem_factory.rs / modem_vendor.rs
        │    vendors/quectel/{mod,parser,qualcomm,unisoc,band_db}.rs
        │
        ├─ HTTP device APIs（factory 模式）
        └─ r26-cli sidecar / DLFrame.dll（固件下载）
```

## 2. 唯一 AT 队列

### 2.1 设计目标

- 实际发送 AT 的地方只能有一条 live 队列。
- 并发问题通过**单一串行化入口**收敛，而不是通过多条 fallback 路径绕开。
- 状态读取必须实时、失败即报错，不能用第二条 AT 或默认值伪装成功。

### 2.2 当前 live 串行化路径

```text
前端 invoke(...)
  → src-tauri/src/lib.rs 中的 #[tauri::command]
  → with_vendor! / with_vendor_cid! / send_raw_at
  → AppState.transport (Mutex<Option<Box<dyn AtTransport>>>)
  → ModemVendor 实现
  → AtTransport::send_at(...)
```

- `AppState.transport` 是当前唯一 live AT 队列 owner。
- `send_raw_at` 也必须走同一把 `transport` 锁，不能成为第二条直连路径。
- `mqtt.rs` 已改为复用同一把 `transport` 锁，并按 `transport -> vendor` 顺序取锁。
- `factory.rs` 的 HTTP 请求和 `dloader.rs` 的 sidecar 不是 AT 路径，不得反向引入第二条 modem AT 队列。

## 3. 单一真相源

### 3.1 前端

- 当前前端是 **plain script 顺序加载**，不是 bundler，也不是 ES Modules。
- 前端唯一状态 owner 是 `state`。
- `localStorage` 只应用于 UI 偏好和少量非 live 记忆项；任何 live modem 状态镜像都视为技术债。

### 3.2 后端

`AppState` 是唯一后端 modem / queue 状态 owner，当前 live 字段包括：

- `transport`
- `vendor`
- `data_cid`
- `connected_port`
- `at_command_log`
- `mqtt_task`
- `license`

此外还有两个独立受管状态：

- `FactoryState`
  工厂模式配置、当前产品、设备地址、流水号等
- `DloaderState`
  固件下载 sidecar 运行句柄

禁止在这些 owner 之外再维护第二份 live 连接状态、CID、日志、License 或下载进程状态。

## 4. Live 后端模块

### 4.1 `src-tauri/src/lib.rs`

- Tauri Builder 装配
- live IPC handler 注册
- `AppState`
- `LoggingTransport`
- 自动连接 / 连接管理
- USB 监控与心跳线程
- 菜单 / 窗口行为

### 4.2 `src-tauri/src/mqtt.rs`

- 可选 MQTT 状态上报循环
- 只能复用 live AT 队列和固定锁顺序
- 不能自建第二条 modem I/O 路径

### 4.3 `src-tauri/src/license.rs`

- 启动时加载 `license.dat`
- 校验 License 并更新 `AppState.license`
- 暴露 `get_license_status` / `load_license_file`
- 通过 `license-changed` 事件通知前端

### 4.4 `src-tauri/src/factory.rs`

- 工厂模式本地配置读写
- 设备 HTTP 通信
- SN / 记录 / 产品配置 IPC
- 不直接参与 AT 队列，但会与前端 License 可见性联动

### 4.5 `src-tauri/src/dloader.rs`

- PAC 选择 / 信息解析
- `r26-cli` sidecar 管理
- `firmware-event` 事件转发
- 不直接参与 AT 队列

## 5. HAL 结构

### 5.1 传输层

- `serial.rs`
- `tcp.rs`
- `websocket.rs`

三者都实现 `AtTransport`，所以 live AT 队列最终只允许收敛到同一 trait 边界：

```rust
pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
    fn is_alive(&self) -> bool { true }
}
```

### 5.2 厂商层

- `modem_factory.rs`
  `AT+CGMM` → 型号字符串 → 芯片平台识别
- `modem_vendor.rs`
  统一业务 trait
- `vendors/quectel/mod.rs`
  共享 Quectel 主流程
- `qualcomm.rs` / `unisoc.rs`
  平台差异实现
- `parser.rs`
  解析函数集合

## 6. 平台差异与合同边界

### 6.1 正式平台差异

- Qualcomm：数据连接 / 状态查询主路径围绕 `AT+QMAP`
- UniSoc：数据连接 / IP 查询主路径围绕 `AT+QNETDEVCTL` / `AT+QNETDEVSTATUS`
- `Unknown`：正式合同是直接报错，不允许猜 adapter

### 6.2 不属于正式合同的内容

以下内容不应继续被当成“设计的一部分”：

- 读取实时状态时改发第二条 AT
- 用空串 / `0` / `false` 伪造查询结果
- 猜测 `Unknown` 型号对应的平台 adapter
- 保留旧连接文件或旧 handler 作为第二条路径

这些都按技术债处理，统一记到 `docs/REVIEW.md`。

## 7. 连接模式与辅助流程

当前后端支持的 live transport 形态：

- 串口自动探测 / 串口直连
- TCP 直连
- WebSocket 网关连接
- 可选 MQTT 上报

当前后端还包含两类辅助路径：

- Factory：HTTP 设备管理
- Firmware：sidecar 驱动刷机

无论 transport 类型如何变化，live modem I/O 仍必须复用同一条 `AppState.transport` 串行化路径。

## 8. 后台线程与事件

- `usb-monitor`
  负责枚举串口变化并发 `port-changed`
- `connection-heartbeat`
  负责检测已连接 transport 的存活状态
- `mqtt loop`
  负责远端状态上报
- `firmware-event`
  负责 sidecar 下载事件转发到前端
- `license-changed`
  负责 License 状态更新广播

后台流程的核心约束只有两条：

1. 不能新增第二条 AT 发送队列
2. 不能通过 fallback 掩盖实时状态失败

## 9. 文档分工

- 需要看**AT 合同**：读 `docs/AT_COMMANDS.md`
- 需要看**前端触发点 / IPC 名**：读 `docs/CODE_MAP.md`
- 需要看**完整调用链**：读 `docs/CALL_FLOW.md`
- 需要看**当前 live 技术债**：读 `docs/REVIEW.md`
- 需要看**构建 / 依赖 / 运行方式**：读 `docs/TECH_STACK.md` 与 `docs/BUILD.md`
