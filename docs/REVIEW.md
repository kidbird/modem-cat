# Code Review 报告

> 基于 `main` 分支 · 严重度：🔴 HIGH 🟡 MEDIUM 🟢 LOW

---

## 1. 🟡 P1 待办

### [MEDIUM] #8 `connect_data` 双重锁
- **问题**：`with_vendor_cid!` 宏外额外 `data_cid.lock()`，与其它命令共享同一把 std Mutex
- **影响**：若某命令在持锁期间再调 `connect_data`（嵌套 IPC），std Mutex 死锁
- **建议**：合并 `data_cid` 进 `AppState` 复合结构，或改用 `parking_lot::Mutex`（非可中毒）

### [MEDIUM] #10 BaselineModem 重构以消除 vendor 重复代码
- **问题**：`ModemVendor` trait 方法中，Qualcomm 与 UniSoc 实现仍有重复
- **建议**：抽 `BaselineModem` 结构体 + `at_cmd(&mut t, &str)` 模板方法，新 vendor 只覆盖差异方法
- **进展**：TdTech 已移除，Qualcomm/UniSoc 共享 QuectelModem，重复度降低但仍可优化

### [MEDIUM] #11 ModemVendor trait 方法过多
- **文件**：`modem-hal/src/modem_vendor.rs`
- **问题**：trait 含大量方法，部分有默认实现返回 `Err`
- **建议**：按功能域拆为多个子 trait（`ModemInfo` / `ModemData` / `ModemConfig` / `ModemLock`）

---

## 2. 🟢 P2 待办

### [LOW] #12 `QuectelChip` 未来加变体会引发连锁 panic
- **文件**：`modem-hal/src/vendors/quectel/mod.rs` 多处 `match self.chip`
- **建议**：加 `#[non_exhaustive]`，每个 match 末尾加 `_ => return Err(...)`

### [LOW] #13 `parser.rs` 解析函数无单元测试
- **文件**：`modem-hal/src/vendors/quectel/parser.rs`
- **建议**：抽 3-5 个代表函数加 `#[cfg(test)]` 测试，每个 1 positive + 1 negative

### [LOW] #14 `parser.rs` 重复模式
- **文件**：同 #13
- **问题**：`for line in resp.lines() { if let Some(rest) = line.trim().strip_prefix("...") { ... } }` 出现 30+ 次
- **建议**：抽 `fn find_line<'a>(resp: &'a str, prefix: &str) -> Option<&'a str>` helper

### [LOW] #15 `auto_connect_at` 串行阻塞
- **问题**：循环里 `spawn_blocking` 同步等待，N 端口 × 3-8s = 3N~8s，期间 UI 卡住
- **建议**：`futures::future::select_all` 并行 probe；或加 `tokio::time::timeout(2s)`

### [LOW] #16 `serial.rs` drain 阶段无 max-loop 保护
- **建议**：加 `for _ in 0..64 { ... }` 限制循环次数

### [LOW] #17 `Result<T, String>` 迁移到 `thiserror`
- **问题**：modem-hal + src-tauri 共 244 处 `Result<T, String>` 签名
- **建议**：增量迁移，先在 modem-hal 引入 `enum HalError`

### [LOW] #18 `LoggingTransport` 锁覆盖 send_at 失败
- **文件**：`src-tauri/src/lib.rs` LoggingTransport 段
- **建议**：把 `log.push` 移到 `inner.send_at` 返回 `Ok` 之后

### [LOW] #19 napi-feature 是死代码
- **文件**：`modem-hal/src/lib.rs` napi-feature 段
- **问题**：`#[napi]` 暴露的 `ModemHandle` 在 `feature = "napi-feature"` 下，但 `Cargo.toml` 未声明此 feature
- **建议**：删除，或在 `Cargo.toml` 启用并补 README

---

## 3. 备注

- `with_vendor!` 宏：消除 34 处样板，价值高；错误消息建议统一为 i18n key
- `tauri.conf.json` `withGlobalTauri: true` 是有意保留，app.js 直接挂 `window.__TAURI__` 即可

---

## 附录：已修复归档

> 以下条目已全部修复，保留供参考。

| # | 严重度 | 问题 | 修复方式 |
|---|--------|------|----------|
| 1 | HIGH | `commands.rs` 死代码参与编译，64 处 `.unwrap()` | 文件已删除 |
| 2 | HIGH | `send_raw_at` 绕过输入校验 | 改用 `validate_raw_at_command` |
| 3 | HIGH | `redact_at_command` 仅 redact QSIMLOCK | 三层防御：QSIMLOCK 专项 + CGAUTH/CGDCONT 位置参数 + 通用 key=value 扫描 |
| 4 | HIGH | `set_plmn_lock` 默认硬编码密码 "12345678" | 强制必传 password + 后端 `validate_at_string` + 前端增加密码输入 |
| 5 | HIGH | heartbeat `.lock()` 阻塞 IPC | 3 处改为 `.try_lock()`，拿不到直接 `continue` |
| 6 | MEDIUM | `LoggingTransport` 1000 条静默截断 | 改用 `VecDeque` 环形 buffer |
| 7 | MEDIUM | `is_alive()` 跨平台不一致 | Linux 增加 `Path::exists` 文件存在校验 |
| 9 | MEDIUM | `validate_at_string` 未拦截分号串联 | 增加 `;` 校验 + 单测覆盖 |
| 20 | LOW | `commands.rs` vs `lib.rs` 锁顺序不一致 | 随 #1 删除 commands.rs 消失 |
