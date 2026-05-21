# 重构计划：统一 AT 命令层，消除双层适配器

## 目标

让 Tauri 桌面应用通过 `ModemVendor` trait 调用所有 AT 命令，而非绕过 trait 直接操作 `AtTransport`。完成 Quectel UniSoc 和 Qualcomm 的完整实现，TdTech 保持接口统一但暂空实现。每条 AT 命令之间延迟 5ms（在收到响应之后），而非发送后立即计时。

## 影响范围

- `modem-hal/src/modem_vendor.rs` — 扩展 trait，增加缺失的方法
- `modem-hal/src/types.rs` — 扩展类型，增加前端需要的字段
- `modem-hal/src/vendors/quectel/mod.rs` — 完善实现，合并 at_adapter 逻辑
- `modem-hal/src/vendors/quectel/parser.rs` — 合并 at_parser 中更完善的解析器
- `modem-hal/src/vendors/quectel/qualcomm.rs` — 补充 Qualcomm 特有功能
- `modem-hal/src/vendors/quectel/unisoc.rs` — 补充 UniSoc 特有功能
- `modem-hal/src/modem_factory.rs` — 补全缺失型号
- `modem-hal/src/transport/mod.rs` — 添加 send_at_with_delay 辅助
- `src-tauri/src/lib.rs` — 重构：AppState 存 ModemVendor，删除 direct at_adapter 调用
- `src-tauri/src/at_adapter.rs` — 删除（逻辑迁移到 modem-hal）
- `src-tauri/src/at_parser.rs` — 删除（解析器迁移到 modem-hal）

## 实现步骤

### 步骤 1：扩展 ModemVendor trait，增加缺失方法

在 `modem_vendor.rs` 中增加 Tauri 前端需要但 trait 中缺失的方法：

- `query_baseline(t) -> BaselineInfo` — AT+QBASELINE
- `query_ant_rssi(t) -> Vec<String>` — AT+QANTRSSI?
- `query_network_mode(t) -> String` — AT+QNWPREFCFG="mode_pref"
- `query_bands_with_spec(t) -> BandConfig` — 包含 spec_bands 的完整 band 查询
- `set_bands(t, lte, nr)` — 设置 LTE + NR band
- `reset_all_bands(t)` — AT+QNWPREFCFG="all_band_reset"
- `query_feature_toggles(t) -> FeatureToggles` — 不再是空实现
- `set_feature_toggle(t, feat, on)` — 不再是空实现
- `query_qos(t, cid) -> QosInfo` — 实现 AT+C5GQOSRDP
- `factory_reset(t)` — 恢复出厂
- `reboot(t)` — 重启（已有但实现不同：AT+CRESET vs AT+CFUN=1,1）
- `send_raw_at(t, cmd) -> String` — 裸 AT 命令
- `query_usbnet_mode(t) -> i32` — AT+QCFG="usbnet"
- `set_usbnet_mode(t, mode)` — AT+QCFG="usbnet",mode

为 TdTech 提供默认空实现（返回 Err 或空值）。

### 步骤 2：合并解析器 — 将 at_parser.rs 的丰富解析迁入 modem-hal

将 `src-tauri/src/at_parser.rs` 中以下解析器迁入 `modem-hal/src/vendors/quectel/parser.rs`：

- `parse_qeng_servingcell` → 替换 modem-hal 的 `parse_qeng_serving_cell`（Tauri 版支持 NR5G-NSA、格式化输出）
- `parse_qeng_neighbourcell` → 替换 modem-hal 的空实现 `parse_qeng_neighbour_cells`
- `parse_qicsgp` → 新增到 parser.rs
- `parse_qnwprefcfg_bands` / `parse_qnwprefcfg_supported` → 替换简单版 `parse_band_list`
- `parse_c5gqosrdp` → 新增
- `parse_qrsrp` / `parse_qantrssi` → 新增
- `parse_qtemp` → 替换 modem-hal 版本（增加 °C 格式化）
- `format_rsrp` / `format_rsrq` / `format_bw` / `format_bandwidth_bps` → 新增到 parser.rs
- `parse_qbaseline` → 新增
- `parse_qcfg_int` / `parse_qcfg_usbcfg_adb` / `parse_qcfg_usbnet` → 新增
- `parse_qnetdevstatus` → 新增（UniSoc IP 查询，替换 unisoc.rs 中的内联解析）
- `is_ok` → 新增

### 步骤 3：重构 cmd_delay — 改为 send_at 后延迟

**当前问题**: `cmd_delay()` 在 `send_at()` 调用之前或之后独立调用，不是原子操作。

**方案**: 不改 AtTransport trait，而是在 `QuectelModem` 和 `TdTechModem` 中封装一个 `send_and_delay()` 辅助方法：

```rust
fn send_and_delay(&self, t: &mut dyn AtTransport, cmd: &str) -> Result<String, String> {
    let result = t.send_at(cmd)?;
    std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(result)
}
```

这样确保 5ms 延迟在**收到响应之后**执行，而非发送后立即计时。

### 步骤 4：完善 QuectelModem 实现，合并 Tauri at_adapter 逻辑

将 `at_adapter.rs` 中所有 Quectel 特定逻辑迁入 `QuectelModem`：

- `query_modem_status` — 合并 Tauri 版的逻辑（带 QANTRSSI、COPS 等）
- `query_hardware_info` — 增加 QBASELINE、QTEMP
- `query_ip_info` — 区分 Qualcomm (QMAP) vs UniSoc (QNETDEVSTATUS)
- `query_apn_list` — 使用 QICSGP 而非 CGDCONT（Tauri 版更完整）
- `query_neighbor_cells` — 用合并后的 `parse_qeng_neighbourcell`
- `query_feature_toggles` — 实现 AT+QCFG 查询
- `set_feature_toggle` — 实现 AT+QCFG 设置
- `query_qos` — 实现 AT+C5GQOSRDP
- `reboot` — 使用 AT+CRESET（Quectel 推荐）
- `factory_reset` — AT+QFACT=0
- `send_raw_at` — t.send_at 直接透传
- `query_traffic` — 区分 Qualcomm (QGDNRCNT) vs UniSoc (QGDCNT)
- `query_usbnet_mode` / `set_usbnet_mode` — AT+QCFG="usbnet"

对 Qualcomm/UniSoc 分支，已有的 `qualcomm.rs` 和 `unisoc.rs` 保留，只需确保通过 `match self.chip` 正确分发。

### 步骤 5：更新类型定义

在 `modem-hal/src/types.rs` 增加前端需要的字段和方法：

- `HardwareInfo` 增加 `ap_baseline`, `cp_baseline`, `soc_temp`, `pa_temp`（已有）
- `TemperatureInfo` 增加 °C 格式化方法
- `ModemStatus` 增加 `ant_values: Vec<String>` — 已有
- 增加 `BaselineInfo` struct（ap_baseline, cp_baseline）
- 增加 `CancelableResult` 或直接用 `Result<(), String>`

### 步骤 6：重构 AppState 和 lib.rs

**关键改动**：AppState 存 `Box<dyn ModemVendor>` + `Box<dyn AtTransport>`

```rust
pub struct AppState {
    pub transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    pub vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
    pub data_cid: Arc<Mutex<i32>>,
    pub connected_port: Arc<Mutex<Option<String>>>,
}
```

连接时（`auto_connect_at` / `connect_serial` / `connect_tcp`）：
1. 建立 transport
2. 调用 `ModemFactory::create(&mut transport)` 检测型号
3. 将 `Box<dyn ModemVendor>` 存入 `AppState.vendor`

所有 Tauri command 改为：
```rust
let mut guard = transport.lock().unwrap();
let t = guard.as_deref_mut().ok_or("Not connected")?;
let mut vguard = vendor.lock().unwrap();
let v = vguard.as_deref_mut().ok_or("No vendor")?;
v.query_modem_status(t)  // 通过 trait 调用
```

### 步骤 7：补全 modem_factory.rs 缺失型号

```rust
let qualcomm = ["RM520N", "RM500Q", "RG520N", "RG525F", "RG530F", "RM530N", "RG540F", "RM540N"];
let unisoc = ["RG200U", "RM500U", "RG501U", "RM501U"];
```

### 步骤 8：删除 at_adapter.rs 和 at_parser.rs

完成迁移后，删除这两个文件，从 `lib.rs` 中移除 `pub mod at_adapter; pub mod at_parser;`。

### 步骤 9：TdTech 空实现 stub

对步骤 1 新增的 trait 方法，TdTech 全部提供 stub 实现（返回 Err 或空默认值），保持接口统一。

## 验证方法

- [ ] `cargo build --workspace` 编译通过
- [ ] `cargo test --workspace` 全部测试通过
- [ ] 所有 Tauri command 通过 `ModemVendor` trait 调用，没有直接 `t.send_at()` 的 Tauri 代码
- [ ] Quectel UniSoc 和 Qualcomm 的所有功能在 trait 中有对应方法
- [ ] TdTech compiles with stubs
- [ ] `atat` (napi) 也能通过 `ModemVendor` trait 正常工作

## 预计风险

1. **合并解析器时字段偏移**：Tauri 的 `parse_qeng_servingcell` 和 modem-hal 的 `parse_qeng_serving_cell` 索引体系不同，合并时需仔细对齐
2. **ModemVendor trait 加方法**：需要给 TdTech 所有新方法提供默认实现或 stub
3. **AppState 生命周期**：vendor 和 transport 需要同时持有可变引用，需确保 lock 不会死锁
4. **connect_serial / auto_connect_at 中检测型号失败**：需要 fallback 策略（如 Unknown vendor）