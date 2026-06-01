# Code Review 报告

> Review 时点：2026-06-01 · 基于 `main` 分支当前代码
> 严重度：🔴 HIGH 　🟡 MEDIUM 　🟢 LOW

本报告聚焦**可立即动手的修复**，每条给出 file:line + 影响 + 建议。
不写空泛建议（"加注释" / "写测试"），不给口水话。

---

## 0. 与文档的事实偏差（修文档前先看）

| 偏差 | 文档说法 | 代码实情 |
|---|---|---|
| 前端结构 | `index.html` 单文件 ~98KB | 已拆为 `index.html` (319KB) + `app.js` (62KB) + `styles.css` (19KB)，`e26df93` "split index.html" 已合入 |
| `at_adapter.rs` / `at_parser.rs` | 仍在 `src-tauri/src/` | **已删除**（commit `35d7053` "unify AT layer through ModemVendor trait"），AT 业务下沉到 `modem-hal/src/vendors/{quectel,tdtech}/` |
| `lib.rs` 行数 | ~1000 行 | **1139 行**（2026-06-01 F2/F4 安全修复后，拆分后仍是单文件最大者） |
| `commands.rs` 状态 | （文档未提） | **504 行死代码**：30 个 `#[tauri::command]` 0 caller，`.unwrap()` **64 处**，会编译进二进制 |
| `start_port_monitor` | 在 `lib.rs` | **重复**：lib.rs:869 与 monitor.rs:13 都有；lib.rs 版本带 `panic::catch_unwind` + 5s 后 sleep 重启（line 919），monitor.rs 版本带 `catch_unwind` 但**无重试 sleep**（catch_unwind 后线程直接退出）|
| 厂商检测关键字 | 文档未列 | TdTech→Qualcomm→UniSoc→Unknown（无默认兜底，未识别直接 `Err`） |
| `serial.rs::is_alive` | 跨平台一致 | **不一致**：Windows 可靠（ClearCommError 4s 内），macOS/Linux 驱动层可能 20s+ 才感知 |

---

## 1. 🔴 P0 清单

### [HIGH] #1 `commands.rs` 是死代码但仍参与编译
- **文件**：`src-tauri/src/commands.rs:1-504`（实际行数 504；旧文档写 517 是早期估算）
- **问题**：30 个 `#[tauri::command]` 函数（`get_modem_status` / `connect_data` / ...）0 个 caller（`grep -rn "mod commands" src-tauri/` 无结果）；**64 处** `transport.lock().unwrap()`（`grep -c "\.unwrap()" src-tauri/src/commands.rs`），Mutex 中毒即 panic
- **影响**：编译进 binary 增体积 ~18KB；未来若有人 `mod commands;` 立即崩溃；`.unwrap()` 64 处意味着 1 次锁污染即 64 处 panic
- **建议**：删除文件；或在 `lib.rs:1066` 注释 + `#[allow(dead_code)]` + `#[doc(hidden)]` 标记废弃

### [HIGH] #2 `send_raw_at` 完全绕过输入校验
- **文件**：`src-tauri/src/lib.rs:724-734`（修复后 729 行加 `validate_at_string`）
- **问题**：不调用 `validate_at_string`；前端 `app.js` 9 处用 `send_raw_at` 发 `AT+QSIMLOCK` / `AT+QNWLOCK` / `AT+QDMZ` 等带敏感字段命令
- **影响**：2c991a4 的注入防护对此路径**完全失效**；XSS 攻击者可注入 `\r\n` 执行额外命令
- **状态**：✅ **2026-06-01 FIXED** —— `send_raw_at` 首行加 `validate_at_string(&command)?;`

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
- **状态**：✅ **2026-06-01 FIXED** —— 改为 `password.ok_or_else(|| "PLMN lock requires a password ...")?;`，强制必传

### [HIGH] #5 heartbeat 与 IPC 争同一把 std Mutex
- **文件**：`src-tauri/src/lib.rs:929-970` (heartbeat) vs `lib.rs:64-97` (with_vendor 宏)
- **问题**：`start_connection_heartbeat` 每 4s `transport.lock()`，与所有 IPC handler 的 `spawn_blocking(transport.lock())` 抢同一把 std Mutex
- **影响**：单条 AT 命令耗 3-8s 期间，heartbeat 阻塞；USB 拔出后 4-12s 才被前端感知
- **建议**：
  - heartbeat 用 `try_lock()`，拿不到就 skip；或
  - 让 `LoggingTransport` 在 `send_at` 出错时把 `Arc<AtomicBool>` 置 false，heartbeat 只读 atomic

---

## 2. 🟡 P1 清单

### [MEDIUM] #6 `LoggingTransport` 1000 条静默截断
- **文件**：`src-tauri/src/lib.rs:36-38`
- **问题**：`if log.len() < 1000 { log.push(...) }` 超过即丢
- **影响**：排查问题时误判"没发命令"；`pop_at_commands` 取到的不是真实完整历史
- **建议**：改用 `VecDeque` 真正环形 buffer（push_back + pop_front）

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

### [MEDIUM] #9 `validate_at_string` 未覆盖多命令串联
- **文件**：`modem-hal/src/lib.rs:18-40`
- **问题**：检查 `\r\n` / `"` / 控制字符；但允许 `;`（多命令串联符）、`S0=0`（寄存器修改）
- **影响**：用户输入 `cmnet;AT+CFUN=1,1` 会执行两条命令
- **建议**：拒绝 `;` `&` `S` 开头；或强制 `format!("AT+CMD=\"{}\"", escape(arg))`

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

### [LOW] #16 `serial.rs` drain 阶段无 max-loop 保护
- **文件**：`modem-hal/src/transport/serial.rs:138-145`
- **建议**：加 `for _ in 0..64 { ... }` 限制循环次数

### [LOW] #17 `Result<T, String>` 迁移到 `thiserror`
- **文件**：`modem-hal/` (166 处) + `src-tauri/` (78 处) = **244 处**签名（`grep -rE "Result<[^>]+, *String>" modem-hal/src/ src-tauri/src/ | wc -l`）
- **建议**：增量迁移，先在 modem-hal 引入 `enum HalError { Io, Parse, AtCme{i32}, AtCms, Vendor(String), LockPoisoned }`
- **工时估算**：~4-6h（含测试）

### [LOW] #18 `LoggingTransport` 锁覆盖 send_at 失败
- **文件**：`src-tauri/src/lib.rs:33-48`
- **建议**：把 `log.push` 移到 `inner.send_at` 返回 `Ok` 之后

### [LOW] #19 napi-feature 是死代码
- **文件**：`modem-hal/src/lib.rs:54-121`
- **问题**：`#[napi]` 暴露的 `ModemHandle` 在 `feature = "napi-feature"` 下，但 `Cargo.toml` 未声明此 feature
- **建议**：删除，或在 `Cargo.toml` 启用并补 README

### [LOW] #20 `commands.rs` vs `lib.rs` 锁顺序不一致
- **文件**：`src-tauri/src/commands.rs:47` vs `lib.rs:91`
- **影响**：删除 commands.rs 后此问题消失
- **建议**：同 #1

---

## 4. 不在清单但值得知道

- `with_vendor!` 宏：消除 34 处样板，价值高；错误消息建议统一为 i18n key
- `tauri.conf.json` `withGlobalTauri: true` 是有意保留，app.js 直接挂 `window.__TAURI__` 即可
- 旧的 `start_port_monitor` 在 lib.rs 与 monitor.rs **同时存在**，是上次 refactor（commit 4253981 "perf: 低风险高收益优化"）未清理的残留

---

## 5. 修复优先级建议

| 周期 | 项目 | 工时估算 |
|---|---|---|
| **本周** | #1（删 commands.rs）+ #2（send_raw_at 加校验）+ #4（删默认密码） | 2-3h |
| **下周** | #3（redact 完善）+ #5（heartbeat 不阻塞）+ #6（VecDeque 环形 buffer） | 3-4h |
| **本月** | #7（is_alive 跨平台）+ #8（合并锁）+ #9（白名单 `;`）+ #10（BaselineModem 重构） | 1-2 天 |
| **后续** | #11 ~ #20 | 持续 |
