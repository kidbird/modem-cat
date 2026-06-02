# Code Review 报告

> Review 时点：2026-06-01 · 基于 `main` 分支当前代码
> 严重度：🔴 HIGH 　🟡 MEDIUM 　🟢 LOW

本报告聚焦**可立即动手的修复**，每条给出 file:line + 影响 + 建议。
不写空泛建议（"加注释" / "写测试"），不给口水话。

---

## 0. 与文档的事实偏差（修文档前先看）

> 2026-06-02 更新：以下条目已随 commit 4f2fb83 (frontend split) 和 claude/fix-review-batch2 (commands.rs / heartbeat) 落地，剩余条目仍待办。

| 偏差 | 文档说法 | 代码实情 | 状态 |
|---|---|---|---|
| 前端结构 | `index.html` 单文件 ~98KB | **2026-06-02**: `index.html` (1494 行) + `app.js` (3621 行) + `styles.css` (1436 行)，拆分已通过 `<link>`/`<script src=>` 真正生效（commit `4f2fb83`） | ✅ FIXED |
| `at_adapter.rs` / `at_parser.rs` | 仍在 `src-tauri/src/` | **已删除**（commit `35d7053` "unify AT layer through ModemVendor trait"），AT 业务下沉到 `modem-hal/src/vendors/{quectel,tdtech}/` | ✅ FIXED |
| `lib.rs` 行数 | ~1000 行 | **~1140 行**（2026-06-02 batch 后仍是单文件最大者） | 维持 |
| `commands.rs` 状态 | （文档未提） | **2026-06-02**: 文件已删除（claude/fix-review-batch2 commit） | ✅ FIXED |
| `start_port_monitor` | 在 `lib.rs` | **重复**：lib.rs:869 与 monitor.rs:13 都有；lib.rs 版本带 `panic::catch_unwind` + 5s 后 sleep 重启（line 919），monitor.rs 版本带 `catch_unwind` 但**无重试 sleep**（catch_unwind 后线程直接退出）| 待办 |
| 厂商检测关键字 | 文档未列 | TdTech→Qualcomm→UniSoc→Unknown（无默认兜底，未识别直接 `Err`） | 待办 |
| `serial.rs::is_alive` | 跨平台一致 | **不一致**：Windows 可靠（ClearCommError 4s 内），macOS/Linux 驱动层可能 20s+ 才感知 | 待办 |
| blue-light 主题 | 文档未列 | 2026-06-02 已加 §3.4 章节说明 3 主题 | ✅ FIXED |

---

## 1. 🔴 P0 清单

### [HIGH] #1 `commands.rs` 是死代码但仍参与编译
- **文件**：`src-tauri/src/commands.rs:1-504`（实际行数 504；旧文档写 517 是早期估算）
- **问题**：30 个 `#[tauri::command]` 函数（`get_modem_status` / `connect_data` / ...）0 个 caller（`grep -rn "mod commands" src-tauri/` 无结果）；**64 处** `transport.lock().unwrap()`（`grep -c "\.unwrap()" src-tauri/src/commands.rs`），Mutex 中毒即 panic
- **影响**：编译进 binary 增体积 ~18KB；未来若有人 `mod commands;` 立即崩溃；`.unwrap()` 64 处意味着 1 次锁污染即 64 处 panic
- **建议**：删除文件；或在 `lib.rs:1066` 注释 + `#[allow(dead_code)]` + `#[doc(hidden)]` 标记废弃
- **状态**：✅ **2026-06-02 FIXED** —— 文件已删除（commit `claude/fix-review-batch2`）。`grep -rn "mod commands" src-tauri/src/` 0 命中。lib.rs 顶部 "52 个 IPC" 与 diagram 中的 `commands.rs` 行同步从 [ARCHITECTURE.md](ARCHITECTURE.md) 移除。

### [HIGH] #2 `send_raw_at` 完全绕过输入校验
- **文件**：`src-tauri/src/lib.rs:724-734`
- **问题**：原先不校验 raw AT；第一次修复误用了参数校验器 `validate_at_string`，会拒绝 `AT+QCFG="ims"` 这类合法完整 AT 命令
- **影响**：不校验会让 XSS 攻击者注入 `\r\n` 执行额外命令；误用参数校验器会导致带引号的合法 raw AT 全部失败
- **状态**：✅ **2026-06-01 FIXED** —— `send_raw_at` 改用 `validate_raw_at_command(&command)?;`，允许合法引号并拒绝换行/控制字符/`;` 串联

### [HIGH] #3 `redact_at_command` 覆盖不全
- **文件**：`modem-hal/src/transport/mod.rs:71-220`（修复后 3 层防御）
- **问题**：只 redact `QSIMLOCK`。遗漏：
  - `AT+CGAUTH=<cid>,<auth>,"<user>","<password>"` (APN 密码)
  - `AT+CGDCONT` 的 PCO 字段（含 PAP/CHAP 凭据）
  - `AT+QHTTPURL` / `AT+QSSLCFG` (token/URL)
  - `AT+QNTP` (NTP server 凭据)
- **影响**：AT 命令日志会原样写出 APN 凭据到磁盘 / 前端
- **状态**：✅ **2026-06-01 FIXED** —— 三层防御：
  1. QSIMLOCK `"PN","<password>"` 专项
  2. 位置参数特例 `CGAUTH` / `CGDCONT`（redact 最后引号值）
  3. 通用 `,key="value"` 扫描（SENSITIVE_KEYS = password/passwd/pwd/token/auth/secret）
- **测试**：`test_redact_at_command_qsimlock` / `_cgauth_password` / `_cgdcnt_password` / `_keyed_password` / `_does_not_match_substring`

### [HIGH] #4 `set_plmn_lock` 默认硬编码密码 "12345678"
- **文件**：`src-tauri/src/lib.rs:761-766, 768-772`
- **问题**：`password.unwrap_or_else(|| "12345678".to_string())` —— 用户没传就套用默认密码
- **影响**：用户误以为"未设置"实则模组已被锁定；安全审计等同于后门
- **状态**：✅ **2026-06-01 FIXED** —— 强制必传 password；后端对 password 调 `validate_at_string`；前端增加供应商密码输入，不再硬编码默认密码

### [HIGH] #5 heartbeat 与 IPC 争同一把 std Mutex
- **文件**：`src-tauri/src/lib.rs:929-970` (heartbeat) vs `lib.rs:64-97` (with_vendor 宏)
- **问题**：`start_connection_heartbeat` 每 4s `transport.lock()`，与所有 IPC handler 的 `spawn_blocking(transport.lock())` 抢同一把 std Mutex
- **影响**：单条 AT 命令耗 3-8s 期间，heartbeat 阻塞；USB 拔出后 4-12s 才被前端感知
- **建议**：
  - heartbeat 用 `try_lock()`，拿不到就 skip；或
  - 让 `LoggingTransport` 在 `send_at` 出错时把 `Arc<AtomicBool>` 置 false，heartbeat 只读 atomic
- **状态**：✅ **2026-06-02 FIXED** —— 3 处 `.lock()` 全改为 `.try_lock()`，拿不到直接 `continue`（下一 tick 再试），不再阻塞 IPC。heartbeat 注释也补了"绝不阻塞 IPC"的红线说明。

---

## 2. 🟡 P1 清单

### [MEDIUM] #6 `LoggingTransport` 1000 条静默截断
- **文件**：`src-tauri/src/lib.rs:36-38`
- **问题**：`if log.len() < 1000 { log.push(...) }` 超过即丢
- **影响**：排查问题时误判"没发命令"；`pop_at_commands` 取到的不是真实完整历史
- **建议**：改用 `VecDeque` 真正环形 buffer（push_back + pop_front）
- **状态**：✅ **2026-06-02 FIXED** —— `Vec<String>` 改 `VecDeque<String>`，`AT_LOG_CAPACITY = 1000` 上限 + `pop_front` 真环形。`pop_at_commands` 改 `drain(..).collect()`。

### [MEDIUM] #7 `is_alive()` 跨平台不一致
- **文件**：`modem-hal/src/transport/serial.rs:168-172`
- **问题**：`bytes_to_read()` 在 Windows 走 `ClearCommError` 4s 内可靠；macOS/Linux USB-serial 驱动在设备移除后 `ioctl` 仍可能返回 `Ok(0)` 数秒
- **影响**：Windows 用户 4s 内断连，macOS 可能 20s+
- **建议**：对 macOS/Linux 在 `is_alive` 内做一次 0 字节 `try_read`（失败即 dead）

### [MEDIUM] #8 `connect_data` 双重锁
- **文件**：`src-tauri/src/lib.rs:604-621`
- **问题**：`with_vendor_cid!` 宏外额外 `data_cid.lock()`，与其它命令共享同一把 std Mutex
- **影响**：若某命令在持锁期间再调 `connect_data`（嵌套 IPC），std Mutex 死锁
- **建议**：合并 `data_cid` 进 `AppState` 复合结构，或改用 `parking_lot::Mutex`（非可中毒）
- **状态**：✅ **2026-06-02 FIXED** —— `data_cid: Arc<Mutex<i32>>` 改 `Arc<AtomicI32>`。`with_vendor_cid!` 宏 / `connect_data` / `disconnect` 三处改 `load(Relaxed)` / `store(Relaxed)`。嵌套 IPC 不再可能死锁。

### [MEDIUM] #9 `validate_at_string` 未覆盖多命令串联
- **文件**：`modem-hal/src/lib.rs:18-40`
- **问题**：检查 `\r\n` / `"` / 控制字符；但允许 `;`（多命令串联符）、`S0=0`（寄存器修改）
- **影响**：用户输入 `cmnet;AT+CFUN=1,1` 会执行两条命令
- **建议**：拒绝 `;` `&` `S` 开头；或强制 `format!("AT+CMD=\"{}\"", escape(arg))`
- **状态**：✅ **2026-06-02 FIXED**（`validate_raw_at_command` 路径）—— `;` 早被拒；`&`（Hayes `AT&F` / `AT&W` 工厂重置）新拒；`ATSn=value` 多位 S-register 写也新拒。`S-register` 读（`ATS0?` / `ATS5`）仍允许。新增测试 `raw_at_command_rejects_hayes_amp_and_s_register`。
- **遗留**：`validate_at_string`（参数校验）仍允许 `;` —— 因为 `;` 走的是 `format!()` 进引号路径，不构成命令串联。`U+2028 LINE SEPARATOR` / `U+2029 PARAGRAPH SEPARATOR` (Zl/Zp) 仍未被 `is_control()` 捕获（仅 Cc 类），后续 wt 处理。

### [MEDIUM] #10 `tdtech/mod.rs` 与 `quectel/mod.rs` 90% 重复
- **文件**：`modem-hal/src/vendors/tdtech/mod.rs:40-419`（429 行总计）
- **问题**：`ModemVendor` trait 共 62 个方法（实际），TdTech 实现了其中约 50 个签名/逻辑一致（`query_sim_status` / `query_imei` / `query_iccid` / `set_apn` / `reboot` ...），每个 TdTech 方法基本只改 AT 命令名
- **影响**：再加 vendor 时维护地狱；bugfix 需改 3 处
- **建议**：抽 `BaselineModem` 结构体 + `at_cmd(&mut t, &str)` 模板方法，新 vendor 只覆盖 5-6 个特殊方法

### [MEDIUM] #11 `tdtech::query_imei` 反向匹配
- **文件**：`modem-hal/src/vendors/tdtech/mod.rs:55-64`
- **问题**：`ln.chars().all(|c| c.is_ascii_digit()) && ln.len() >= 14` —— 若响应里任何一行恰好 14+ 位纯数字（CRC、版本号），误判
- **影响**：偶发返回错误 IMEI
- **建议**：优先匹配 `+CGSN:` / `^IMEI:` 前缀，再 fallback

---

## 3. 🟢 P2 清单

### [LOW] #12 `QuectelChip` 未来加变体会引发连锁 panic
- **文件**：`modem-hal/src/vendors/quectel/mod.rs` 17 处 `match self.chip`
- **建议**：加 `#[non_exhaustive]`，每个 match 末尾加 `_ => return Err(...)`
- **状态**：✅ **2026-06-02 FIXED** —— `pub enum QuectelChip` 加 `#[non_exhaustive]`。下游 crate 不再能 exhaustive-match；同 crate 内的 match 加新 variant 时编译器会强制提示。

### [LOW] #13 `parser.rs` 80+ 解析函数无单元测试
- **文件**：`modem-hal/src/vendors/quectel/parser.rs` (1260 行)
- **建议**：抽 3-5 个代表函数加 `#[cfg(test)]` 测试，每个 1 positive + 1 negative

### [LOW] #14 `parser.rs` 重复模式
- **文件**：同 #13
- **问题**：`for line in resp.lines() { if let Some(rest) = line.trim().strip_prefix("...") { ... } }` 出现 30+ 次
- **建议**：抽 `fn find_line<'a>(resp: &'a str, prefix: &str) -> Option<&'a str>` helper

### [LOW] #15 `auto_connect_at` 串行阻塞
- **文件**：`src-tauri/src/lib.rs:353-429`（函数定义 353-；循环体 ~381-426）
- **问题**：循环里 `spawn_blocking` 同步等待，N 端口 × 3-8s = 3N~8s，期间 UI 卡住
- **建议**：`futures::future::select_all` 并行 probe；或加 `tokio::time::timeout(2s)`
- **状态**：✅ **2026-06-02 FIXED**（commits 63efedd + fcdd5c6）—— N 个端口并行 `tokio::spawn` probe + `tokio::time::timeout(2s)` 兜底。启动 3 端口从 9-24s 降到 ~2s。`fcdd5c6` 把 vendor detection 移出 timeout 防止误切。
- **遗留**：vendor detection 自身无上限（`ModemFactory::create` 内部），若某型号 SKU 检测 hang 会阻塞拿 first-success 路径。

### [LOW] #16 `serial.rs` drain 阶段无 max-loop 保护
- **文件**：`modem-hal/src/transport/serial.rs:138-145`
- **建议**：加 `for _ in 0..64 { ... }` 限制循环次数
- **状态**：✅ **2026-06-02 FIXED** —— `for _ in 0..MAX_DRAIN_READS`（MAX_DRAIN_READS = 64，64 × 4096 B = 256 KB 远超合理 stale buffer）。

### [LOW] #17 `Result<T, String>` 迁移到 `thiserror`
- **文件**：`modem-hal/` (166 处) + `src-tauri/` (78 处) = **244 处**签名（`grep -rE "Result<[^>]+, *String>" modem-hal/src/ src-tauri/src/ | wc -l`）
- **建议**：增量迁移，先在 modem-hal 引入 `enum HalError { Io, Parse, AtCme{i32}, AtCms, Vendor(String), LockPoisoned }`
- **工时估算**：~4-6h（含测试）

### [LOW] #18 `LoggingTransport` 锁覆盖 send_at 失败
- **文件**：`src-tauri/src/lib.rs:33-48`
- **建议**：把 `log.push` 移到 `inner.send_at` 返回 `Ok` 之后
- **状态**：✅ **2026-06-02 FIXED** —— `send_at` 先调用 `inner.send_at`，结果拿到后再 push 日志。失败时条目附 `⟵ error` 信息。锁改 `try_lock` 永不阻塞 AT 路径。

### [LOW] #19 napi-feature 是死代码
- **文件**：`modem-hal/src/lib.rs:54-121`
- **问题**：`#[napi]` 暴露的 `ModemHandle` 在 `feature = "napi-feature"` 下，但 `Cargo.toml` 未声明此 feature
- **建议**：删除，或在 `Cargo.toml` 启用并补 README
- **状态**：✅ **2026-06-02 FIXED** —— `napi_exports` 模块（70 行）删除；`Cargo.toml` 移除 `napi` / `napi-derive` 依赖 + `napi-feature` feature + `napi-build` build-dep；`build.rs` 简化为 `fn main() {}`；`types.rs` 2 个 `#[cfg_attr(feature = "napi-feature", ...)]` 删除。`Cargo.lock` -83 行。

### [LOW] #20 `commands.rs` vs `lib.rs` 锁顺序不一致
- **文件**：`src-tauri/src/commands.rs:47` vs `lib.rs:91`
- **影响**：删除 commands.rs 后此问题消失
- **建议**：同 #1
- **状态**：✅ **2026-06-02 FIXED** —— 随 #1 `commands.rs` 删除而消失。`src-tauri/src/commands.rs` 已不存在。

---

## 4. 不在清单但值得知道

- `with_vendor!` 宏：消除 34 处样板，价值高；错误消息建议统一为 i18n key
- `tauri.conf.json` `withGlobalTauri: true` 是有意保留，app.js 直接挂 `window.__TAURI__` 即可
- 旧的 `start_port_monitor` 在 lib.rs 与 monitor.rs **同时存在**，是上次 refactor（commit 4253981 "perf: 低风险高收益优化"）未清理的残留

---

## 5. 修复优先级建议

> 2026-06-02 更新：P0 5 条 + 9 条 P1/P2 已修（#6/#8/#9 + #12/#15/#16/#18/#19/#20）。剩余 #7（macOS 暂缓）、#10/#11（TdTech 暂缓）、#13/#14（parser 测试/抽 helper）、#17（thiserror 迁移）。

| 周期 | 项目 | 工时估算 |
|---|---|---|
| ~~本周~~ | ~~#1（删 commands.rs）~~ ✅ + ~~#2/#4（安全校验）~~ ✅ | 0h（已完成） |
| ~~下周~~ | ~~#3/#5（redact 完善 + heartbeat 不阻塞）~~ ✅ + ~~#6（VecDeque）~~ ✅ | 0h |
| ~~本月~~ | ~~#8（合并锁 → AtomicI32）~~ ✅ + ~~#9（拒绝 `; & S`）~~ ✅ + ~~#12/#15/#16/#18/#19/#20~~ ✅ | 0h |
| **下一批** | #13（parser 单测）+ #14（parser 抽 `find_line` helper）+ #17（thiserror 迁移 244 处）| 半天-2 天 |
| **后续** | #7（is_alive macOS — 暂缓）+ #10/#11（TdTech — 暂缓） | 持续 |

> 2026-06-02 更新：P0 5 条已修 5 条（#1/#2/#3/#4/#5 全部 `✅ FIXED`），剩余优先级 #6 (VecDeque) 是 P1 中最低门槛的；建议下一批从这里开始。
