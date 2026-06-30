# REVIEW.md

> 最近更新：2026-06-30（worktree `fix/review-high-priority`）
> 只记录当前主线仍然存在的 live 技术债与文档/代码偏差；历史已修问题不再在本文件重复维护。

## High Priority

### 1. transport 超时后可能把截断响应当成功

- `serial.rs` / `tcp.rs` / `websocket.rs` 现在都可能在”收到部分字节后超时”时返回 `Ok`
- 风险：上层 parser 会把链路抖动误判成真实状态
- 处理原则：实时状态读取应把”不完整响应”视为错误，而不是伪成功

### 2. 非 Windows 网卡列表仍返回 mock 数据

- 前端会把返回的 gateway 当成真实 WebSocket 目标
- 风险：UI 展示为可连接，实际上目标地址是伪造值
- 处理原则：要么返回真实枚举，要么明确不支持，不要伪造 live 输入

### 5. 前端 30+ 个 IPC catch 错误未使用 `e.message` 模式

- `CODE_MAP.md §6` 规定 `invoke` 失败时 catch 对象是 `{ message: string }`，统一用 `e.message || String(e)` 兜底
- 仅 2/32 个 catch 块遵循；其余用原始 `'...' + e` / `` `${e}` ``
- 失败路径：`core.js:6` 的 invoke 包装在 IPC 不可用时 reject `new Error('Tauri IPC not available')`；原始 `+ e` 处理器会产出无意义的 `[object Object]`
- 修复建议：在 `core.js` invoke 助手中一次性归一化，返回 `e.message ?? String(e)`

## Medium Priority

### 3. 自动连接缺少 in-flight 保护

- 热插拔自动重连与人工连接触发之间缺少统一”连接中”状态
- 风险：旧断开收尾打掉新连接，或重复发起并发连接任务

### 4. MQTT 启停状态不是单一真相源

- 前端当前会把 MQTT 开关写入 `localStorage`
- 后端真实状态 owner 实际是 `AppState.mqtt_task`
- 风险：UI 和记忆状态覆盖 live 状态

### 6. `atdb.js` 中 `AT+QCFG=”nat”` 数值与 `scene.js` 直接矛盾

| 来源 | nat=0 | nat=1 | nat=2 |
|---|---|---|---|
| `atdb.js` (UniSoc) | 网卡模式 | 路由(NAT) | 网桥 |
| `scene.js` | 桥接 | （未使用） | 路由 |

- `scene.js` 是实时 IPC 调用源，手册与实际效果直接矛盾

### 7. `AT+QCFG=”ims”` 实时发送但不在 AT 合同 / 手册中

- `app.js:1113,1148,1149` 通过 `send_raw_at` 下发 IMS/VoLTE 开关
- 既无 `AT_COMMANDS.md` 条目也无 `data/atdb.js` 条目，违反”AT 命令来源受限”
- **已修复**：已添加到 `AT_COMMANDS.md §6` 和 `data/atdb.js`（UniSoc + Qualcomm 两节）

### 9. 工厂模式 HTTP 客户端静默回退

| 位置 | 问题 |
|---|---|
| `factory.rs:228` | `danger_accept_invalid_certs(true)` |
| `factory.rs:231` | `Client::new()` 静默回退丢掉 5s 超时 |
| `factory.rs:662,226` | 设备 IP 格式直接格式化为 `http://<ip>/api`，无校验 |
| `factory.rs:384-396` | `factory_select.json` 损坏时静默回退到第一个品牌 |
| `factory.rs:519` | SN 序列解析失败时静默重置为 `”00001”`（危害 SN 唯一性） |

### 10. 工厂硬编码敏感默认值

| 位置 | 问题 |
|---|---|
| `index.html:961` | Qualcomm 5GLAN 表单 APN 字段预填充 `value=”5glan2”`（AGENTS.md 中 APN 属敏感字段） |
| `index.html:965` | SNSSAI 字段预填充 `value=”03.010102”` |
| `app.js:2697,2775` | 工厂设备 IP 字段为空时静默使用 `192.168.42.1` |

### 11. `mqtt.rs:210,214` 锁争用与离线混淆

- `try_query_modem_status` 锁争用返回 `Ok(None)`，出站报告仍会发布 `modemConnected: false`
- 将”太忙无法读取”与”调制解调器离线”混为一谈

### 12. 文档漂移（`CALL_FLOW.md` / `CODE_MAP.md`）

- `CALL_FLOW.md §5`：`invoke('connect_data', { cid: 1 })` — 代码无参调用 `app.js:42`
- `CALL_FLOW.md §6`：`invoke('get_ip_info', { cid: 1 })` — 代码无参调用 `app.js:1556`
- `CODE_MAP.md §2.6`：`set_feature_toggle × 3` — `scene.js` 最多 ×2

## Cleanup Rules

- 触碰 live AT 队列、实时状态读取、vendor 检测、连接路径时，必须同步清理 fallback 和死代码
- 不要把技术债写成正式合同；合同进 `ARCHITECTURE.md` / `AT_COMMANDS.md`，现状偏差留在本文件
- 清理完的债务应直接从本文件移除，不保留”已修历史年表”
