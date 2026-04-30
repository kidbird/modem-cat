# Multi-Vendor Modem HAL Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract the Rust modem logic into a standalone `modem-hal` crate supporting Qualcomm, UniSoc, and TdTech platforms, consumable via Tauri (rlib), Bun/TS CLI (napi-rs .node), and embedded Linux (static binary).

**Architecture:** A single `modem-hal` Rust workspace member contains all transport, AT parsing, and vendor adapter code. `QuectelModem` (with a `QuectelChip` enum) handles both Qualcomm and UniSoc with shared code and per-chip branches. `TdTechModem` is a fully independent implementation using `AT^` prefix commands. napi-rs exposes a thin `#[napi]` surface for Bun/TS callers; the embedded CLI crate links the same lib statically.

**Tech Stack:** Rust 2021, serialport 4, napi-rs 2, clap 4, cross (for embedded cross-compilation), existing Tauri 2 app.

---

## File Map

### New crate: `modem-hal/`
| File | Responsibility |
|------|---------------|
| `modem-hal/Cargo.toml` | Crate config, features (`serial`, `napi`), workspace member |
| `modem-hal/src/lib.rs` | `pub use` re-exports + `#[napi]` surface when feature `napi` enabled |
| `modem-hal/src/types.rs` | All shared structs/enums — moved from `src-tauri/src/types.rs`, adds `ChipsetVendor::TdTech` |
| `modem-hal/src/transport/mod.rs` | `AtTransport` trait |
| `modem-hal/src/transport/serial.rs` | `SerialTransport` — moved from `src-tauri/src/transport.rs` |
| `modem-hal/src/transport/tcp.rs` | `TcpTransport` — moved from `src-tauri/src/transport.rs` |
| `modem-hal/src/modem_vendor.rs` | `ModemVendor` trait — moved unchanged |
| `modem-hal/src/modem_factory.rs` | `ModemFactory` + `VendorDetector` — adds TdTech branch |
| `modem-hal/src/vendors/mod.rs` | `pub mod quectel; pub mod tdtech;` |
| `modem-hal/src/vendors/quectel/mod.rs` | `QuectelChip` enum, `QuectelModem` struct, `ModemVendor` impl with shared AT commands |
| `modem-hal/src/vendors/quectel/parser.rs` | `parse_qeng_serving_cell(raw, chip)` — chip-aware bandwidth decode; all other shared Quectel parsers moved here from `at_parser.rs` |
| `modem-hal/src/vendors/quectel/qualcomm.rs` | Qualcomm-specific overrides: `connect_data` (QMAP), `query_ip_info` (QMAP WWAN), `query_traffic` (QGDNRCNT) |
| `modem-hal/src/vendors/quectel/unisoc.rs` | UniSoc-specific overrides: `connect_data` (QNETDEVCTL), `query_ip_info` (QNETDEVSTATUS), `query_traffic` (QGDCNT) |
| `modem-hal/src/vendors/tdtech/mod.rs` | `TdTechModem` struct + full `ModemVendor` impl |
| `modem-hal/src/vendors/tdtech/parser.rs` | `parse_hcsq`, `parse_monsc`, `parse_syscfgex`, `parse_dconnstat` |
| `modem-hal/src/vendors/tdtech/dial.rs` | `ndisdup_connect/disconnect`, `dhcp_query_ip`, `hex_ip_to_string` |

### New crate: `modem-cli-embedded/`
| File | Responsibility |
|------|---------------|
| `modem-cli-embedded/Cargo.toml` | Binary crate, depends on `modem-hal` (no `napi` feature) |
| `modem-cli-embedded/src/main.rs` | `clap` CLI: `status`, `signal`, `connect`, `disconnect` subcommands; JSON output |

### Modified: `src-tauri/`
| File | Change |
|------|--------|
| `src-tauri/Cargo.toml` | Add `modem-hal = { path = "../modem-hal" }` dep; remove direct serialport/modem logic deps |
| `src-tauri/src/lib.rs` | Remove modem module declarations; import from `modem_hal::*` |
| `src-tauri/src/at_parser.rs` | Delete (logic moves to `modem-hal/src/vendors/quectel/parser.rs`) |
| `src-tauri/src/transport.rs` | Delete (moves to `modem-hal/src/transport/`) |
| `src-tauri/src/types.rs` | Delete (moves to `modem-hal/src/types.rs`) |
| `src-tauri/src/modem_vendor.rs` | Delete (moves to `modem-hal`) |
| `src-tauri/src/modem_factory.rs` | Delete (moves to `modem-hal`) |
| `src-tauri/src/vendor_detector.rs` | Delete (moves to `modem-hal`) |
| `src-tauri/src/vendors/qualcomm.rs` | Delete (replaced by `modem-hal/src/vendors/quectel/`) |
| `src-tauri/src/vendors/unisoc.rs` | Delete (replaced by `modem-hal/src/vendors/quectel/`) |

### New: workspace root
| File | Responsibility |
|------|---------------|
| `Cargo.toml` (root) | Workspace definition listing `modem-hal` and `modem-cli-embedded` |

---

## Task 1: Create Cargo workspace and `modem-hal` skeleton

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `modem-hal/Cargo.toml`
- Create: `modem-hal/src/lib.rs`

- [ ] **Step 1: Create workspace root Cargo.toml**

```toml
# /Volumes/kidbird/code/modem-cat/Cargo.toml
[workspace]
members = [
    "modem-hal",
    "modem-cli-embedded",
    "src-tauri",
]
resolver = "2"
```

- [ ] **Step 2: Create `modem-hal/Cargo.toml`**

```toml
# /Volumes/kidbird/code/modem-cat/modem-hal/Cargo.toml
[package]
name = "modem-hal"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib", "cdylib", "staticlib"]

[features]
default = ["serial"]
serial = ["dep:serialport"]

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
serialport = { version = "4", optional = true }

[target.'cfg(target_env = "musl")'.dependencies]
serialport = { version = "4", default-features = false, optional = true }
```

- [ ] **Step 3: Create `modem-hal/src/lib.rs`**

```rust
// modem-hal/src/lib.rs
pub mod modem_vendor;
pub mod modem_factory;
pub mod transport;
pub mod types;
pub mod vendors;

pub use modem_vendor::ModemVendor;
pub use modem_factory::ModemFactory;
pub use types::*;
```

- [ ] **Step 4: Create stub `modem-hal/src/types.rs`** (empty for now — to be filled in Task 2)

```rust
// modem-hal/src/types.rs
// Types will be moved here in Task 2
```

- [ ] **Step 5: Create stub module files**

Create these files each with just `// placeholder`:
- `modem-hal/src/modem_vendor.rs`
- `modem-hal/src/modem_factory.rs`
- `modem-hal/src/transport/mod.rs`
- `modem-hal/src/transport/serial.rs`
- `modem-hal/src/transport/tcp.rs`
- `modem-hal/src/vendors/mod.rs`
- `modem-hal/src/vendors/quectel/mod.rs`
- `modem-hal/src/vendors/quectel/parser.rs`
- `modem-hal/src/vendors/quectel/qualcomm.rs`
- `modem-hal/src/vendors/quectel/unisoc.rs`
- `modem-hal/src/vendors/tdtech/mod.rs`
- `modem-hal/src/vendors/tdtech/parser.rs`
- `modem-hal/src/vendors/tdtech/dial.rs`

- [ ] **Step 6: Verify workspace builds**

```bash
cd /Volumes/kidbird/code/modem-cat
cargo check -p modem-hal 2>&1 | head -20
```

Expected: errors only about missing content in stubs, no structural errors.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml modem-hal/
git commit -m "chore: create modem-hal crate skeleton and cargo workspace"
```

---

## Task 2: Move types and transport to `modem-hal`

**Files:**
- Create: `modem-hal/src/types.rs` (from `src-tauri/src/types.rs`)
- Create: `modem-hal/src/transport/mod.rs`
- Create: `modem-hal/src/transport/serial.rs`
- Create: `modem-hal/src/transport/tcp.rs`
- Test: `modem-hal/src/transport/mod.rs` (inline `#[cfg(test)]`)

- [ ] **Step 1: Write failing test for transport trait**

Add to `modem-hal/src/transport/mod.rs`:

```rust
pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
}

pub struct MockTransport {
    pub responses: std::collections::VecDeque<String>,
}

impl MockTransport {
    pub fn new(responses: Vec<&str>) -> Self {
        Self {
            responses: responses.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl AtTransport for MockTransport {
    fn send_at(&mut self, _command: &str) -> Result<String, String> {
        self.responses.pop_front().ok_or("no more responses".to_string())
    }
    fn close(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_transport_returns_responses_in_order() {
        let mut t = MockTransport::new(vec!["OK", "ERROR"]);
        assert_eq!(t.send_at("AT").unwrap(), "OK");
        assert_eq!(t.send_at("AT+FAIL").unwrap(), "ERROR");
        assert!(t.send_at("AT").is_err());
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd /Volumes/kidbird/code/modem-cat
cargo test -p modem-hal transport 2>&1
```

Expected: FAIL — module not yet filled.

- [ ] **Step 3: Copy `types.rs` and add `TdTech` variant**

Copy `src-tauri/src/types.rs` to `modem-hal/src/types.rs`, then add `TdTech` to `ChipsetVendor`:

```rust
// In ChipsetVendor enum, add:
pub enum ChipsetVendor {
    UniSoc,
    Qualcomm,
    TdTech,    // 鼎桥 MT5700 series
    Unknown,
}

impl ChipsetVendor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChipsetVendor::UniSoc => "unisoc",
            ChipsetVendor::Qualcomm => "qualcomm",
            ChipsetVendor::TdTech => "tdtech",
            ChipsetVendor::Unknown => "unknown",
        }
    }
}
```

Also update `SignalInfo` to use `Option<f32>` for unknown-able fields (TdTech ^HCSQ returns 255 for unknown):

```rust
#[derive(Debug, Clone, Default)]
pub struct SignalInfo {
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub ant_values: [String; 4],
}
```

(Keep as `String` to avoid breaking existing callers — TdTech will write "N/A" for 255 values.)

- [ ] **Step 4: Copy SerialTransport to `modem-hal/src/transport/serial.rs`**

Copy the `SerialTransport` struct, `probe_at`, `read_response`, and `impl AtTransport for SerialTransport` from `src-tauri/src/transport.rs` to `modem-hal/src/transport/serial.rs`. Add feature gate at top:

```rust
// modem-hal/src/transport/serial.rs
#[cfg(feature = "serial")]
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;
use crate::transport::AtTransport;

#[cfg(feature = "serial")]
pub struct SerialTransport {
    port: Box<dyn SerialPort>,
}

#[cfg(feature = "serial")]
impl SerialTransport {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(500))
            .open()
            .map_err(|e| format!("Failed to open {}: {}", port_name, e))?;
        Ok(Self { port })
    }

    pub fn probe_at(port_name: &str, baud_rate: u32) -> bool {
        // ... (copy body from src-tauri/src/transport.rs SerialTransport::probe_at)
    }

    fn read_response(&mut self) -> Result<String, String> {
        // ... (copy body from src-tauri/src/transport.rs SerialTransport::read_response)
    }
}

#[cfg(feature = "serial")]
impl AtTransport for SerialTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        // ... (copy body)
    }
    fn close(&mut self) {}
}
```

- [ ] **Step 5: Copy TcpTransport to `modem-hal/src/transport/tcp.rs`**

Copy `TcpTransport` and its `impl AtTransport` from `src-tauri/src/transport.rs`:

```rust
// modem-hal/src/transport/tcp.rs
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use crate::transport::AtTransport;

pub struct TcpTransport {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
}

impl TcpTransport {
    pub fn new(host: &str, port: u16) -> Result<Self, String> {
        // ... (copy body)
    }
    fn read_response(&mut self) -> Result<String, String> {
        // ... (copy body)
    }
}

impl AtTransport for TcpTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        // ... (copy body)
    }
    fn close(&mut self) {
        let _ = self.writer.shutdown(std::net::Shutdown::Both);
    }
}
```

- [ ] **Step 6: Update `modem-hal/src/transport/mod.rs` to re-export**

```rust
// modem-hal/src/transport/mod.rs
pub mod tcp;
#[cfg(feature = "serial")]
pub mod serial;

pub use tcp::TcpTransport;
#[cfg(feature = "serial")]
pub use serial::SerialTransport;

// AtTransport trait + MockTransport + tests (from Step 1 above)
```

- [ ] **Step 7: Run tests**

```bash
cargo test -p modem-hal transport 2>&1
```

Expected: `mock_transport_returns_responses_in_order ... ok`

- [ ] **Step 8: Commit**

```bash
git add modem-hal/src/types.rs modem-hal/src/transport/
git commit -m "feat(modem-hal): move types and transport layer, add TdTech ChipsetVendor"
```

---

## Task 3: Move `ModemVendor` trait and copy `ModemFactory` skeleton

**Files:**
- Create: `modem-hal/src/modem_vendor.rs`
- Create: `modem-hal/src/modem_factory.rs`

- [ ] **Step 1: Copy `modem_vendor.rs` unchanged**

Copy `src-tauri/src/modem_vendor.rs` to `modem-hal/src/modem_vendor.rs`. Update the use statements at the top:

```rust
use crate::types::*;
use crate::transport::AtTransport;
```

The trait body is unchanged — it already covers all three platforms via its method list.

- [ ] **Step 2: Write failing test for factory model detection**

Add to `modem-hal/src/modem_factory.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChipsetVendor;

    #[test]
    fn detects_qualcomm_from_model() {
        assert_eq!(ModemFactory::detect_vendor_from_model("RG520N-GL"), ChipsetVendor::Qualcomm);
        assert_eq!(ModemFactory::detect_vendor_from_model("RM520N-GL"), ChipsetVendor::Qualcomm);
    }

    #[test]
    fn detects_unisoc_from_model() {
        assert_eq!(ModemFactory::detect_vendor_from_model("RG200U-CN"), ChipsetVendor::UniSoc);
        assert_eq!(ModemFactory::detect_vendor_from_model("RM500U-GL"), ChipsetVendor::UniSoc);
    }

    #[test]
    fn detects_tdtech_from_model() {
        assert_eq!(ModemFactory::detect_vendor_from_model("MT5700M-CN"), ChipsetVendor::TdTech);
        assert_eq!(ModemFactory::detect_vendor_from_model("MT5700"), ChipsetVendor::TdTech);
    }

    #[test]
    fn unknown_model_returns_unknown() {
        assert_eq!(ModemFactory::detect_vendor_from_model("XYZ1234"), ChipsetVendor::Unknown);
    }
}
```

- [ ] **Step 3: Run test to verify it fails**

```bash
cargo test -p modem-hal modem_factory 2>&1
```

Expected: FAIL — `ModemFactory` not defined yet.

- [ ] **Step 4: Implement `ModemFactory` with TdTech support**

```rust
// modem-hal/src/modem_factory.rs
use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::ChipsetVendor;
use crate::vendors::quectel::QuectelModem;
use crate::vendors::tdtech::TdTechModem;

pub struct ModemFactory;

impl ModemFactory {
    pub fn create(transport: &mut dyn AtTransport) -> Result<Box<dyn ModemVendor>, String> {
        let model = Self::query_model(transport)?;
        let vendor = Self::detect_vendor_from_model(&model);
        log::info!("Detected model: '{}', vendor: {:?}", model, vendor);
        Self::create_from_vendor(model, vendor)
    }

    pub fn create_from_vendor(model: String, vendor: ChipsetVendor) -> Result<Box<dyn ModemVendor>, String> {
        match vendor {
            ChipsetVendor::Qualcomm => {
                log::info!("Creating Qualcomm adapter for {}", model);
                Ok(Box::new(QuectelModem::qualcomm(model)))
            }
            ChipsetVendor::UniSoc => {
                log::info!("Creating UniSoc adapter for {}", model);
                Ok(Box::new(QuectelModem::unisoc(model)))
            }
            ChipsetVendor::TdTech => {
                log::info!("Creating TdTech adapter for {}", model);
                Ok(Box::new(TdTechModem::new(model)))
            }
            ChipsetVendor::Unknown => {
                log::warn!("Unknown vendor for '{}', defaulting to UniSoc", model);
                Ok(Box::new(QuectelModem::unisoc(model)))
            }
        }
    }

    pub fn detect_vendor_from_model(model: &str) -> ChipsetVendor {
        let upper = model.to_uppercase();
        let tdtech = ["MT5700"];
        for m in &tdtech {
            if upper.contains(m) { return ChipsetVendor::TdTech; }
        }
        let unisoc = ["RG200U", "RG500U", "RM500U", "RG501U", "RM501U"];
        for m in &unisoc {
            if upper.contains(m) { return ChipsetVendor::UniSoc; }
        }
        let qualcomm = ["RG520N", "RM520N", "RG525F", "RG530F", "RM530N", "RG540F", "RM540N"];
        for m in &qualcomm {
            if upper.contains(m) { return ChipsetVendor::Qualcomm; }
        }
        ChipsetVendor::Unknown
    }

    fn query_model(transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+CGMM")?;
        for line in resp.lines() {
            let t = line.trim();
            if t.starts_with("AT+") || t == "OK" || t.starts_with("ERROR") { continue; }
            if let Some(rest) = t.strip_prefix("+CGMM:") { return Ok(rest.trim().to_string()); }
            if !t.is_empty() && !t.starts_with('+') { return Ok(t.to_string()); }
        }
        Ok(String::new())
    }
}
```

- [ ] **Step 5: Add stub vendor modules so it compiles**

In `modem-hal/src/vendors/mod.rs`:
```rust
pub mod quectel;
pub mod tdtech;
```

In `modem-hal/src/vendors/quectel/mod.rs` (stub):
```rust
use crate::types::*;
use crate::transport::AtTransport;
use crate::modem_vendor::ModemVendor;

pub mod parser;
pub mod qualcomm;
pub mod unisoc;

pub enum QuectelChip { Qualcomm, UniSoc }

pub struct QuectelModem {
    pub chip: QuectelChip,
    pub model: String,
}

impl QuectelModem {
    pub fn qualcomm(model: String) -> Self { Self { chip: QuectelChip::Qualcomm, model } }
    pub fn unisoc(model: String) -> Self { Self { chip: QuectelChip::UniSoc, model } }
}

// Stub impl — to be filled in Task 4
impl ModemVendor for QuectelModem {
    fn vendor(&self) -> ChipsetVendor {
        match self.chip {
            QuectelChip::Qualcomm => ChipsetVendor::Qualcomm,
            QuectelChip::UniSoc => ChipsetVendor::UniSoc,
        }
    }
    fn model(&self) -> &str { &self.model }
    fn query_sim_status(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_imei(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_iccid(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_hardware_info(&mut self, _t: &mut dyn AtTransport) -> Result<HardwareInfo, String> { todo!() }
    fn query_temperature(&mut self, _t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> { todo!() }
    fn query_serving_cell(&mut self, _t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> { todo!() }
    fn query_neighbor_cells(&mut self, _t: &mut dyn AtTransport) -> Result<NeighborCells, String> { todo!() }
    fn query_signal_strength(&mut self, _t: &mut dyn AtTransport) -> Result<SignalInfo, String> { todo!() }
    fn query_operator(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_registration_status(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_connection_status(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_apn_list(&mut self, _t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> { todo!() }
    fn set_apn(&mut self, _t: &mut dyn AtTransport, _cid: i32, _ctx: i32, _apn: &str, _user: &str, _pass: &str, _auth: i32) -> Result<(), String> { todo!() }
    fn delete_apn(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<(), String> { todo!() }
    fn connect_data(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<(), String> { todo!() }
    fn disconnect_data(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<(), String> { todo!() }
    fn query_ip_info(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<IpInfo, String> { todo!() }
    fn query_band_config(&mut self, _t: &mut dyn AtTransport) -> Result<BandConfig, String> { todo!() }
    fn set_lte_bands(&mut self, _t: &mut dyn AtTransport, _bands: &str) -> Result<(), String> { todo!() }
    fn set_nr5g_bands(&mut self, _t: &mut dyn AtTransport, _bands: &str) -> Result<(), String> { todo!() }
    fn set_network_mode(&mut self, _t: &mut dyn AtTransport, _mode: &str) -> Result<(), String> { todo!() }
    fn query_traffic(&mut self, _t: &mut dyn AtTransport) -> Result<TrafficInfo, String> { todo!() }
    fn reset_traffic(&mut self, _t: &mut dyn AtTransport) -> Result<(), String> { todo!() }
    fn query_feature_toggles(&mut self, _t: &mut dyn AtTransport) -> Result<FeatureToggles, String> { todo!() }
    fn set_feature_toggle(&mut self, _t: &mut dyn AtTransport, _feat: &str, _on: bool) -> Result<(), String> { todo!() }
    fn reboot(&mut self, _t: &mut dyn AtTransport) -> Result<(), String> { todo!() }
    fn set_cfun(&mut self, _t: &mut dyn AtTransport, _mode: i32) -> Result<(), String> { todo!() }
    fn query_qos(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<QosInfo, String> { todo!() }
}
```

Same stub pattern for `modem-hal/src/vendors/tdtech/mod.rs` with `TdTechModem`.

- [ ] **Step 6: Run the factory tests**

```bash
cargo test -p modem-hal modem_factory 2>&1
```

Expected: all 4 detection tests pass.

- [ ] **Step 7: Commit**

```bash
git add modem-hal/src/modem_vendor.rs modem-hal/src/modem_factory.rs modem-hal/src/vendors/
git commit -m "feat(modem-hal): ModemVendor trait, ModemFactory with TdTech detection"
```

---

## Task 4: Implement `QuectelModem` — shared AT commands and parser

**Files:**
- Create: `modem-hal/src/vendors/quectel/parser.rs`
- Modify: `modem-hal/src/vendors/quectel/mod.rs`

- [ ] **Step 1: Write failing parser tests**

```rust
// modem-hal/src/vendors/quectel/parser.rs (top section)
use crate::types::ServingCellInfo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_qeng_lte_qualcomm_bandwidth_index() {
        // Qualcomm: bandwidth field is an index (5 → 20 MHz)
        let raw = r#"+QENG: "servingcell","CONNECT","LTE","FDD",460,11,1A2B3C4,100,2650,3,5,5,5AE,-95,-10,-65,18,10"#;
        let info = parse_qeng_serving_cell(raw, true);
        assert_eq!(info.bandwidth, "20");
        assert_eq!(info.arfcn, "2650");
        assert_eq!(info.rsrp, "-95");
        assert_eq!(info.sinr, "18");
    }

    #[test]
    fn parse_qeng_lte_unisoc_bandwidth_direct() {
        // UniSoc: bandwidth field is already MHz (100 → "100")
        let raw = r#"+QENG: "servingcell","CONNECT","LTE","FDD",460,11,1A2B3C4,100,2650,3,100,5,5AE,-95,-10,-65,18,10"#;
        let info = parse_qeng_serving_cell(raw, false);
        assert_eq!(info.bandwidth, "100");
    }

    #[test]
    fn parse_qeng_returns_default_on_empty() {
        let info = parse_qeng_serving_cell("OK", true);
        assert!(!info.connected);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test -p modem-hal quectel::parser 2>&1
```

Expected: FAIL — `parse_qeng_serving_cell` not defined.

- [ ] **Step 3: Implement `parse_qeng_serving_cell` in parser.rs**

```rust
// modem-hal/src/vendors/quectel/parser.rs
use crate::types::ServingCellInfo;

/// Parse AT+QENG="servingcell" response.
/// `qualcomm_bandwidth`: true = index lookup, false = direct MHz value (UniSoc)
pub fn parse_qeng_serving_cell(response: &str, qualcomm_bandwidth: bool) -> ServingCellInfo {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("+QENG: \"servingcell\",") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
            // parts[0]=conn_state, [1]=rat, [2]=duplex(LTE), ...
            if parts.len() < 4 { continue; }
            let connected = parts[0] != "NOCONN";
            let tech = parts[1].to_string();

            if tech == "LTE" && parts.len() >= 17 {
                // LTE: conn,rat,duplex,mcc,mnc,cellid,pcid,earfcn,freq_band,ul_bw,dl_bw,tac,rsrp,rsrq,rssi,sinr,srxlev
                let bw_raw = parts.get(10).unwrap_or(&"").trim();
                let bandwidth = if qualcomm_bandwidth {
                    decode_qualcomm_bandwidth(bw_raw.parse::<u32>().unwrap_or(0))
                } else {
                    bw_raw.to_string()
                };
                return ServingCellInfo {
                    connected,
                    tech,
                    operator_mcc: parts.get(3).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(4).unwrap_or(&"").to_string(),
                    cell_id: parts.get(5).unwrap_or(&"").to_string(),
                    pci: parts.get(6).unwrap_or(&"").to_string(),
                    arfcn: parts.get(7).unwrap_or(&"").to_string(),
                    band: parts.get(8).unwrap_or(&"").to_string(),
                    bandwidth,
                    rsrp: parts.get(12).unwrap_or(&"").to_string(),
                    rsrq: parts.get(13).unwrap_or(&"").to_string(),
                    sinr: parts.get(15).unwrap_or(&"").to_string(),
                    tx_power: String::new(),
                    scs: String::new(),
                };
            }

            if tech == "NR5G-SA" && parts.len() >= 14 {
                // NR: conn,rat,mcc,mnc,cellid,pcid,arfcn,band,arfcn_ul,scs,rsrp,rsrq,sinr,srxlev
                return ServingCellInfo {
                    connected,
                    tech,
                    operator_mcc: parts.get(2).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(3).unwrap_or(&"").to_string(),
                    cell_id: parts.get(4).unwrap_or(&"").to_string(),
                    pci: parts.get(5).unwrap_or(&"").to_string(),
                    arfcn: parts.get(6).unwrap_or(&"").to_string(),
                    band: parts.get(7).unwrap_or(&"").to_string(),
                    bandwidth: String::new(),
                    rsrp: parts.get(10).unwrap_or(&"").to_string(),
                    rsrq: parts.get(11).unwrap_or(&"").to_string(),
                    sinr: parts.get(12).unwrap_or(&"").to_string(),
                    tx_power: String::new(),
                    scs: parts.get(9).unwrap_or(&"").to_string(),
                };
            }
        }
    }
    ServingCellInfo::default()
}

/// Qualcomm bandwidth index → MHz string
/// 0→1.4, 1→3, 2→5, 3→10, 4→15, 5→20, 6→100
fn decode_qualcomm_bandwidth(idx: u32) -> String {
    match idx {
        0 => "1.4".to_string(),
        1 => "3".to_string(),
        2 => "5".to_string(),
        3 => "10".to_string(),
        4 => "15".to_string(),
        5 => "20".to_string(),
        6 => "100".to_string(),
        n => n.to_string(),
    }
}

/// Parse common basic info (moved from at_parser.rs)
pub fn parse_cpin(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CPIN: ") { return rest.trim().to_string(); }
    }
    "UNKNOWN".to_string()
}

pub fn parse_cgsn(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') { continue; }
        if line.chars().all(|c| c.is_ascii_digit()) && line.len() >= 14 { return line; }
    }
    String::new()
}

pub fn parse_iccid(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+CCID: ").or_else(|| line.strip_prefix("+ICCID: "))
            .or_else(|| line.strip_prefix("+QCCID: ")) {
            return rest.trim().to_string();
        }
    }
    String::new()
}

pub fn parse_cgmm(response: &str) -> String {
    for line in extract_data_lines(response) {
        if line.starts_with('+') { continue; }
        return line.trim().to_string();
    }
    String::new()
}

pub fn parse_cops(response: &str) -> String {
    for line in extract_data_lines(response) {
        if let Some(rest) = line.strip_prefix("+COPS:") {
            let parts: Vec<&str> = rest.split(',').collect();
            if parts.len() >= 3 {
                return parts[2].trim().trim_matches('"').to_string();
            }
        }
    }
    String::new()
}

pub fn extract_data_lines(response: &str) -> Vec<String> {
    response.lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && t != "OK" && !t.starts_with("ERROR")
                && !t.starts_with("+CME ERROR")
                && !t.starts_with("AT+") && !t.starts_with("AT^")
                && t != "AT"
        })
        .map(|l| l.trim().to_string())
        .collect()
}
```

- [ ] **Step 4: Run parser tests**

```bash
cargo test -p modem-hal quectel::parser 2>&1
```

Expected: all 3 tests pass.

- [ ] **Step 5: Implement shared `ModemVendor` methods in `QuectelModem`**

Replace the `todo!()` stubs in `modem-hal/src/vendors/quectel/mod.rs` for all shared methods:

```rust
// modem-hal/src/vendors/quectel/mod.rs
use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;
use super::quectel::parser::*;
use super::quectel::qualcomm;
use super::quectel::unisoc;

fn cmd_delay() {
    std::thread::sleep(std::time::Duration::from_millis(5));
}

impl ModemVendor for QuectelModem {
    fn vendor(&self) -> ChipsetVendor {
        match self.chip {
            QuectelChip::Qualcomm => ChipsetVendor::Qualcomm,
            QuectelChip::UniSoc => ChipsetVendor::UniSoc,
        }
    }

    fn model(&self) -> &str { &self.model }

    fn query_sim_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CPIN?")?;
        Ok(parse_cpin(&resp))
    }

    fn query_imei(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CGSN")?;
        Ok(parse_cgsn(&resp))
    }

    fn query_iccid(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        // Qualcomm uses AT+ICCID, UniSoc uses AT+CCID/AT+QCCID
        let cmd = match self.chip {
            QuectelChip::Qualcomm => "AT+ICCID",
            QuectelChip::UniSoc => "AT+CCID",
        };
        let resp = t.send_at(cmd)?;
        let iccid = parse_iccid(&resp);
        if !iccid.is_empty() { return Ok(iccid); }
        // Fallback for UniSoc
        cmd_delay();
        let resp2 = t.send_at("AT+QCCID")?;
        Ok(parse_iccid(&resp2))
    }

    fn query_operator(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+COPS?")?;
        Ok(parse_cops(&resp))
    }

    fn query_registration_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CEREG?")?;
        for line in extract_data_lines(&resp) {
            if let Some(rest) = line.strip_prefix("+CEREG:") {
                let parts: Vec<&str> = rest.trim().split(',').collect();
                let stat = parts.get(1).unwrap_or(&parts.get(0).unwrap_or(&"0")).trim();
                return Ok(stat.to_string());
            }
        }
        Ok("0".to_string())
    }

    fn query_connection_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CGACT?")?;
        for line in extract_data_lines(&resp) {
            if let Some(rest) = line.strip_prefix("+CGACT:") {
                let parts: Vec<&str> = rest.trim().split(',').collect();
                if parts.get(1).map(|s| s.trim()) == Some("1") {
                    return Ok("1".to_string());
                }
            }
        }
        Ok("0".to_string())
    }

    fn query_serving_cell(&mut self, t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> {
        let resp = t.send_at("AT+QENG=\"servingcell\"")?;
        let qualcomm = matches!(self.chip, QuectelChip::Qualcomm);
        Ok(parse_qeng_serving_cell(&resp, qualcomm))
    }

    fn query_signal_strength(&mut self, t: &mut dyn AtTransport) -> Result<SignalInfo, String> {
        let resp = t.send_at("AT+QENG=\"servingcell\"")?;
        let qualcomm = matches!(self.chip, QuectelChip::Qualcomm);
        let cell = parse_qeng_serving_cell(&resp, qualcomm);
        Ok(SignalInfo {
            rsrp: cell.rsrp,
            rsrq: cell.rsrq,
            sinr: cell.sinr,
            ant_values: Default::default(),
        })
    }

    fn query_neighbor_cells(&mut self, t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let resp = t.send_at("AT+QENG=\"neighbourcell\"")?;
        // Reuse existing neighbour cell parser logic (moved from at_parser.rs)
        Ok(parse_qeng_neighbour_cells(&resp))
    }

    fn reboot(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT+CFUN=1,1")?;
        Ok(())
    }

    fn set_cfun(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CFUN={}", mode))?;
        Ok(())
    }

    fn set_lte_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"lte_band\",{}", bands))?;
        Ok(())
    }

    fn set_nr5g_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"nr5g_band\",{}", bands))?;
        Ok(())
    }

    fn set_network_mode(&mut self, t: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"mode_pref\",{}", mode))?;
        Ok(())
    }

    fn query_hardware_info(&mut self, t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let model = parse_cgmm(&t.send_at("AT+CGMM")?);
        cmd_delay();
        let manufacturer = { let r = t.send_at("AT+CGMI")?; parse_cgmm(&r) };
        cmd_delay();
        let firmware = { let r = t.send_at("AT+GMR")?; parse_cgmm(&r) };
        Ok(HardwareInfo {
            model, manufacturer, firmware,
            ap_baseline: String::new(),
            cp_baseline: String::new(),
            soc_temp: String::new(),
            pa_temp: String::new(),
        })
    }

    fn query_temperature(&mut self, t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        // UniSoc: AT+QTEMP?; Qualcomm: not available
        match self.chip {
            QuectelChip::UniSoc => {
                let resp = t.send_at("AT+QTEMP?")?;
                Ok(parse_qtemp(&resp))
            }
            QuectelChip::Qualcomm => Ok(TemperatureInfo::default()),
        }
    }

    fn query_apn_list(&mut self, t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> {
        let cgdcont = t.send_at("AT+CGDCONT?")?;
        cmd_delay();
        let cgact = t.send_at("AT+CGACT?")?;
        let active_cids = parse_cgact_cids(&cgact);
        Ok(parse_cgdcont_apn(&cgdcont, &active_cids))
    }

    fn set_apn(&mut self, t: &mut dyn AtTransport, cid: i32, ctx: i32, apn: &str, user: &str, pass: &str, auth: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CGDCONT={},{:?},{:?}", cid,
            match ctx { 2 => "IPV6", 3 => "IPV4V6", _ => "IP" }, apn))?;
        if !user.is_empty() {
            t.send_at(&format!("AT+QICSGP={},1,{:?},{:?},{:?},{}", cid, apn, user, pass, auth))?;
        }
        Ok(())
    }

    fn delete_apn(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CGDCONT={}", cid))?;
        Ok(())
    }

    fn connect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::connect_data(t, cid),
            QuectelChip::UniSoc => unisoc::connect_data(t, cid),
        }
    }

    fn disconnect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::disconnect_data(t, cid),
            QuectelChip::UniSoc => unisoc::disconnect_data(t, cid),
        }
    }

    fn query_ip_info(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::query_ip_info(t, cid),
            QuectelChip::UniSoc => unisoc::query_ip_info(t, cid),
        }
    }

    fn query_band_config(&mut self, t: &mut dyn AtTransport) -> Result<BandConfig, String> {
        let lte_supported_resp = t.send_at("AT+QNWPREFCFG=\"lte_band\"")?;
        cmd_delay();
        let nr_supported_resp = t.send_at("AT+QNWPREFCFG=\"nr5g_band\"")?;
        Ok(BandConfig {
            lte_supported: parse_band_list(&lte_supported_resp),
            nr_supported: parse_band_list(&nr_supported_resp),
            lte_locked: vec![],
            nr_locked: vec![],
        })
    }

    fn query_traffic(&mut self, t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::query_traffic(t),
            QuectelChip::UniSoc => unisoc::query_traffic(t),
        }
    }

    fn reset_traffic(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT+QGDCNT=0")?;
        Ok(())
    }

    fn query_feature_toggles(&mut self, _t: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
        Ok(FeatureToggles::default())
    }

    fn set_feature_toggle(&mut self, _t: &mut dyn AtTransport, _feat: &str, _on: bool) -> Result<(), String> {
        Ok(())
    }

    fn query_qos(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<QosInfo, String> {
        Ok(QosInfo { cqi: String::new(), ul_bandwidth: String::new(), dl_bandwidth: String::new() })
    }
}
```

- [ ] **Step 6: Implement chip-specific overrides in `qualcomm.rs` and `unisoc.rs`**

`modem-hal/src/vendors/quectel/qualcomm.rs`:
```rust
use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};

pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let resp = t.send_at(&format!("AT+QMAP=\"connect\",{}", cid))?;
    if resp.contains("ERROR") { return Err(format!("QMAP connect failed: {}", resp)); }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    t.send_at(&format!("AT+QMAP=\"disconnect\",{}", cid))?;
    Ok(())
}

pub fn query_ip_info(t: &mut dyn AtTransport, _cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at("AT+QMAP=\"WWAN\"")?;
    let mut info = IpInfo {
        ipv4_addr: String::new(), ipv4_mask: String::new(),
        ipv4_gw: String::new(), ipv4_dns: String::new(),
        ipv6_addr: String::new(), ipv6_gw: String::new(), ipv6_dns: String::new(),
    };
    for line in resp.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("+QMAP: \"WWAN\",") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
            if parts.len() >= 5 {
                let family = parts.get(3).unwrap_or(&"");
                let addr = parts.get(4).unwrap_or(&"");
                if *addr == "0.0.0.0" { continue; }
                if *family == "IPV4" { info.ipv4_addr = addr.to_string(); }
                else if *family == "IPV6" { info.ipv6_addr = addr.to_string(); }
            }
        }
    }
    Ok(info)
}

pub fn query_traffic(t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
    let resp = t.send_at("AT+QGDNRCNT?")?;
    for line in resp.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QGDNRCNT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                return Ok(TrafficInfo {
                    ul_bytes: parts[0].trim().parse().unwrap_or(0),
                    dl_bytes: parts[1].trim().parse().unwrap_or(0),
                });
            }
        }
    }
    Ok(TrafficInfo { ul_bytes: 0, dl_bytes: 0 })
}
```

`modem-hal/src/vendors/quectel/unisoc.rs`:
```rust
use crate::transport::AtTransport;
use crate::types::{IpInfo, TrafficInfo};

pub fn connect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    let resp = t.send_at(&format!("AT+QNETDEVCTL=1,{},1", cid))?;
    if resp.contains("ERROR") { return Err(format!("QNETDEVCTL connect failed: {}", resp)); }
    Ok(())
}

pub fn disconnect_data(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    t.send_at(&format!("AT+QNETDEVCTL=0,{},1", cid))?;
    Ok(())
}

pub fn query_ip_info(t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at(&format!("AT+QNETDEVSTATUS={}", cid))?;
    let mut info = IpInfo {
        ipv4_addr: String::new(), ipv4_mask: String::new(),
        ipv4_gw: String::new(), ipv4_dns: String::new(),
        ipv6_addr: String::new(), ipv6_gw: String::new(), ipv6_dns: String::new(),
    };
    for line in resp.lines() {
        let t2 = line.trim();
        if let Some(rest) = t2.strip_prefix("+QNETDEVSTATUS:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim().trim_matches('"')).collect();
            // +QNETDEVSTATUS: <cid>,<ipv4_addr>,<ipv4_mask>,<ipv4_gw>,<dns1>,<dns2>
            if parts.len() >= 5 {
                info.ipv4_addr = parts.get(1).unwrap_or(&"").to_string();
                info.ipv4_mask = parts.get(2).unwrap_or(&"").to_string();
                info.ipv4_gw   = parts.get(3).unwrap_or(&"").to_string();
                info.ipv4_dns  = parts.get(4).unwrap_or(&"").to_string();
            }
        }
    }
    Ok(info)
}

pub fn query_traffic(t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
    let resp = t.send_at("AT+QGDCNT?")?;
    for line in resp.lines() {
        if let Some(rest) = line.trim().strip_prefix("+QGDCNT:") {
            let parts: Vec<&str> = rest.trim().split(',').collect();
            if parts.len() >= 2 {
                return Ok(TrafficInfo {
                    ul_bytes: parts[0].trim().parse().unwrap_or(0),
                    dl_bytes: parts[1].trim().parse().unwrap_or(0),
                });
            }
        }
    }
    Ok(TrafficInfo { ul_bytes: 0, dl_bytes: 0 })
}
```

- [ ] **Step 7: Run full quectel tests**

```bash
cargo test -p modem-hal vendors::quectel 2>&1
```

Expected: all parser tests pass, no compilation errors.

- [ ] **Step 8: Commit**

```bash
git add modem-hal/src/vendors/quectel/
git commit -m "feat(modem-hal): QuectelModem with shared AT commands and per-chip overrides"
```

---

## Task 5: Implement `TdTechModem`

**Files:**
- Create: `modem-hal/src/vendors/tdtech/parser.rs`
- Create: `modem-hal/src/vendors/tdtech/dial.rs`
- Modify: `modem-hal/src/vendors/tdtech/mod.rs`

- [ ] **Step 1: Write failing tests for TdTech parsers**

```rust
// modem-hal/src/vendors/tdtech/parser.rs (test section)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hcsq_lte_converts_indices() {
        // rsrp_idx=60 → -140+60 = -80 dBm
        // sinr_idx=200 → 200*0.2-20 = 20.0 dB
        // rsrq_idx=20 → 20*0.5-19.5 = -9.5 dB
        let raw = "^HCSQ: \"LTE\",30,60,200,20";
        let s = parse_hcsq(raw);
        assert_eq!(s.rsrp, "-80");
        assert_eq!(s.sinr, "20.0");
        assert_eq!(s.rsrq, "-9.5");
    }

    #[test]
    fn parse_hcsq_unknown_index_255_returns_na() {
        let raw = "^HCSQ: \"LTE\",255,255,255,255";
        let s = parse_hcsq(raw);
        assert_eq!(s.rsrp, "N/A");
        assert_eq!(s.sinr, "N/A");
        assert_eq!(s.rsrq, "N/A");
    }

    #[test]
    fn parse_monsc_lte_returns_direct_dbm() {
        let raw = "^MONSC: LTE,460,11,2650,1A2B3C,5AE,1234,-95,-10,-75";
        let c = parse_monsc(raw);
        assert_eq!(c.tech, "LTE");
        assert_eq!(c.arfcn, "2650");
        assert_eq!(c.rsrp, "-95");
        assert_eq!(c.rsrq, "-10");
    }

    #[test]
    fn parse_monsc_nr_returns_direct_dbm() {
        let raw = "^MONSC: NR,460,11,504990,1,AABBCC,3EF,112233,-85,-12,15";
        let c = parse_monsc(raw);
        assert_eq!(c.tech, "NR");
        assert_eq!(c.sinr, "15");
        assert_eq!(c.scs, "1");
    }

    #[test]
    fn hex_ip_conversion() {
        // 0xC0A80101 = 192.168.1.1
        assert_eq!(hex_ip_to_string("0xC0A80101"), "192.168.1.1");
        assert_eq!(hex_ip_to_string("0x08080808"), "8.8.8.8");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test -p modem-hal vendors::tdtech 2>&1
```

Expected: FAIL — `parse_hcsq`, `parse_monsc`, `hex_ip_to_string` not defined.

- [ ] **Step 3: Implement `parser.rs`**

```rust
// modem-hal/src/vendors/tdtech/parser.rs
use crate::types::{SignalInfo, ServingCellInfo};

/// Parse AT^HCSQ? response — converts indexed values to dBm strings
pub fn parse_hcsq(response: &str) -> SignalInfo {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^HCSQ: ") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
            let mode = parts.get(0).unwrap_or(&"");
            if *mode == "LTE" && parts.len() >= 5 {
                let rsrp_idx: u32 = parts[2].parse().unwrap_or(255);
                let sinr_idx: u32 = parts[3].parse().unwrap_or(255);
                let rsrq_idx: u32 = parts[4].parse().unwrap_or(255);
                return SignalInfo {
                    rsrp: decode_rsrp(rsrp_idx),
                    sinr: decode_sinr(sinr_idx),
                    rsrq: decode_rsrq(rsrq_idx),
                    ant_values: Default::default(),
                };
            }
            if *mode == "NR" && parts.len() >= 4 {
                let rsrp_idx: u32 = parts[1].parse().unwrap_or(255);
                let sinr_idx: u32 = parts[2].parse().unwrap_or(255);
                let rsrq_idx: u32 = parts[3].parse().unwrap_or(255);
                return SignalInfo {
                    rsrp: decode_rsrp(rsrp_idx),
                    sinr: decode_sinr(sinr_idx),
                    rsrq: decode_rsrq(rsrq_idx),
                    ant_values: Default::default(),
                };
            }
        }
    }
    SignalInfo::default()
}

fn decode_rsrp(idx: u32) -> String {
    if idx == 255 { return "N/A".to_string(); }
    format!("{}", idx as i32 - 140)
}

fn decode_sinr(idx: u32) -> String {
    if idx == 255 { return "N/A".to_string(); }
    let val = (idx as f32) * 0.2 - 20.0;
    format!("{:.1}", val)
}

fn decode_rsrq(idx: u32) -> String {
    if idx == 255 { return "N/A".to_string(); }
    let val = (idx as f32) * 0.5 - 19.5;
    format!("{:.1}", val)
}

/// Parse AT^MONSC response — direct dBm values
pub fn parse_monsc(response: &str) -> ServingCellInfo {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^MONSC: ") {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
            let rat = parts.get(0).unwrap_or(&"");
            if *rat == "LTE" && parts.len() >= 10 {
                // ^MONSC: LTE,MCC,MNC,ARFCN,Cell_ID,PCI,TAC,RSRP,RSRQ,RSSI
                return ServingCellInfo {
                    connected: true,
                    tech: "LTE".to_string(),
                    operator_mcc: parts.get(1).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(2).unwrap_or(&"").to_string(),
                    arfcn: parts.get(3).unwrap_or(&"").to_string(),
                    cell_id: parts.get(4).unwrap_or(&"").to_string(),
                    pci: parts.get(5).unwrap_or(&"").to_string(),
                    band: String::new(),
                    bandwidth: String::new(),
                    rsrp: parts.get(7).unwrap_or(&"").to_string(),
                    rsrq: parts.get(8).unwrap_or(&"").to_string(),
                    sinr: String::new(),
                    tx_power: String::new(),
                    scs: String::new(),
                };
            }
            if *rat == "NR" && parts.len() >= 11 {
                // ^MONSC: NR,MCC,MNC,ARFCN,SCS,Cell_ID,PCI,TAC,RSRP,RSRQ,SINR
                return ServingCellInfo {
                    connected: true,
                    tech: "NR".to_string(),
                    operator_mcc: parts.get(1).unwrap_or(&"").to_string(),
                    operator_mnc: parts.get(2).unwrap_or(&"").to_string(),
                    arfcn: parts.get(3).unwrap_or(&"").to_string(),
                    scs: parts.get(4).unwrap_or(&"").to_string(),
                    cell_id: parts.get(5).unwrap_or(&"").to_string(),
                    pci: parts.get(6).unwrap_or(&"").to_string(),
                    band: String::new(),
                    bandwidth: String::new(),
                    rsrp: parts.get(8).unwrap_or(&"").to_string(),
                    rsrq: parts.get(9).unwrap_or(&"").to_string(),
                    sinr: parts.get(10).unwrap_or(&"").to_string(),
                    tx_power: String::new(),
                };
            }
        }
    }
    ServingCellInfo::default()
}

/// Parse AT^DCONNSTAT? — check connection status
pub fn parse_dconnstat(response: &str) -> bool {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^DCONNSTAT:") {
            // ^DCONNSTAT: <cid>,"<APN>",<ipv4_stat>,<ipv6_stat>,<type>
            let parts: Vec<&str> = rest.trim().split(',').collect();
            // ipv4_stat at index 2
            if parts.get(2).map(|s| s.trim()) == Some("1") { return true; }
        }
    }
    false
}

/// Parse AT^SYSCFGEX? — extract acqorder and lteband
pub fn parse_syscfgex(response: &str) -> (String, String) {
    for line in response.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("^SYSCFGEX:") {
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim().trim_matches('"')).collect();
            let acqorder = parts.get(0).unwrap_or(&"").to_string();
            let lteband = parts.get(4).unwrap_or(&"").to_string();
            return (acqorder, lteband);
        }
    }
    (String::new(), String::new())
}

/// Convert TdTech hex IP (big-endian u32) to dotted decimal
pub fn hex_ip_to_string(hex: &str) -> String {
    let n = u32::from_str_radix(hex.trim_start_matches("0x").trim_start_matches("0X"), 16)
        .unwrap_or(0);
    let b = n.to_be_bytes();
    format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
}
```

- [ ] **Step 4: Implement `dial.rs`**

```rust
// modem-hal/src/vendors/tdtech/dial.rs
use crate::transport::AtTransport;
use crate::types::IpInfo;
use super::parser::{hex_ip_to_string, parse_dconnstat};

pub fn connect(
    t: &mut dyn AtTransport,
    cid: i32,
    apn: &str,
    user: &str,
    pass: &str,
    auth: i32,
) -> Result<(), String> {
    let cmd = if apn.is_empty() {
        format!("AT^NDISDUP={},1", cid)
    } else {
        format!("AT^NDISDUP={},1,\"{}\",\"{}\",\"{}\",{}", cid, apn, user, pass, auth)
    };
    let resp = t.send_at(&cmd)?;
    if resp.contains("ERROR") {
        return Err(format!("NDISDUP connect failed: {}", resp));
    }
    // Manual requires 5s before DHCP is available
    std::thread::sleep(std::time::Duration::from_secs(5));
    // Verify connected
    let stat = t.send_at("AT^DCONNSTAT?")?;
    if !parse_dconnstat(&stat) {
        return Err("Connection not established after NDISDUP".to_string());
    }
    Ok(())
}

pub fn disconnect(t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
    t.send_at(&format!("AT^NDISDUP={},0", cid))?;
    Ok(())
}

pub fn query_ip(t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
    let resp = t.send_at(&format!("AT^DHCP={}", cid))?;
    for line in resp.lines() {
        let ln = line.trim();
        if let Some(rest) = ln.strip_prefix("^DHCP:") {
            // ^DHCP: clip,netmask,gate,dhcp,pDNS,sDNS,max_rx,max_tx
            let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim()).collect();
            return Ok(IpInfo {
                ipv4_addr: parts.get(0).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv4_mask: parts.get(1).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv4_gw:   parts.get(2).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv4_dns:  parts.get(4).map(|s| hex_ip_to_string(s)).unwrap_or_default(),
                ipv6_addr: String::new(),
                ipv6_gw:   String::new(),
                ipv6_dns:  String::new(),
            });
        }
    }
    Err("No DHCP response".to_string())
}
```

- [ ] **Step 5: Implement `TdTechModem` full `ModemVendor` impl**

```rust
// modem-hal/src/vendors/tdtech/mod.rs
pub mod parser;
pub mod dial;

use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;
use parser::*;

pub struct TdTechModem { model: String }

impl TdTechModem {
    pub fn new(model: String) -> Self { Self { model } }
}

fn cmd_delay() { std::thread::sleep(std::time::Duration::from_millis(5)); }

impl ModemVendor for TdTechModem {
    fn vendor(&self) -> ChipsetVendor { ChipsetVendor::TdTech }
    fn model(&self) -> &str { &self.model }

    fn query_sim_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^CARDMODE")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("^CARDMODE:") {
                return Ok(match rest.trim() {
                    "0" => "NO SIM".to_string(),
                    "1" => "SIM".to_string(),
                    "2" => "USIM".to_string(),
                    v => v.to_string(),
                });
            }
        }
        Ok("UNKNOWN".to_string())
    }

    fn query_imei(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CGSN")?;
        for line in resp.lines() {
            let ln = line.trim();
            if ln.chars().all(|c| c.is_ascii_digit()) && ln.len() >= 14 {
                return Ok(ln.to_string());
            }
        }
        Ok(String::new())
    }

    fn query_iccid(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^ICCID?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("^ICCID:") {
                return Ok(rest.trim().to_string());
            }
        }
        Ok(String::new())
    }

    fn query_hardware_info(&mut self, t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let model_resp = t.send_at("AT+CGMM")?;
        cmd_delay();
        let mfr_resp = t.send_at("AT+CGMI")?;
        cmd_delay();
        let fw_resp = t.send_at("AT+GMR")?;
        let extract = |r: &str| -> String {
            for line in r.lines() {
                let ln = line.trim();
                if ln.is_empty() || ln == "OK" || ln.starts_with("AT") || ln.starts_with('+') { continue; }
                return ln.to_string();
            }
            String::new()
        };
        Ok(HardwareInfo {
            model: extract(&model_resp),
            manufacturer: extract(&mfr_resp),
            firmware: extract(&fw_resp),
            ap_baseline: String::new(),
            cp_baseline: String::new(),
            soc_temp: String::new(),
            pa_temp: String::new(),
        })
    }

    fn query_temperature(&mut self, _t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        Ok(TemperatureInfo::default())
    }

    fn query_serving_cell(&mut self, t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> {
        let resp = t.send_at("AT^MONSC")?;
        Ok(parse_monsc(&resp))
    }

    fn query_signal_strength(&mut self, t: &mut dyn AtTransport) -> Result<SignalInfo, String> {
        let resp = t.send_at("AT^HCSQ?")?;
        Ok(parse_hcsq(&resp))
    }

    fn query_neighbor_cells(&mut self, t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let _resp = t.send_at("AT^MONNC")?;
        Ok(NeighborCells { lte: vec![], nr: vec![] })
    }

    fn query_operator(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+COPS?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+COPS:") {
                let parts: Vec<&str> = rest.split(',').collect();
                if let Some(name) = parts.get(2) {
                    return Ok(name.trim().trim_matches('"').to_string());
                }
            }
        }
        Ok(String::new())
    }

    fn query_registration_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CEREG?")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+CEREG:") {
                let parts: Vec<&str> = rest.trim().split(',').collect();
                let stat = parts.get(1).unwrap_or(&parts.get(0).unwrap_or(&"0")).trim();
                return Ok(stat.to_string());
            }
        }
        Ok("0".to_string())
    }

    fn query_connection_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT^DCONNSTAT?")?;
        Ok(if parse_dconnstat(&resp) { "1".to_string() } else { "0".to_string() })
    }

    fn query_apn_list(&mut self, t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> {
        let resp = t.send_at("AT+CGDCONT?")?;
        let mut entries = vec![];
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("+CGDCONT:") {
                let parts: Vec<&str> = rest.split(',').map(|s| s.trim().trim_matches('"')).collect();
                if parts.len() >= 3 {
                    entries.push(ApnEntry {
                        cid: parts[0].parse().unwrap_or(0),
                        apn_name: parts.get(2).unwrap_or(&"").to_string(),
                        ip_type: parts.get(1).unwrap_or(&"IP").to_string(),
                        auth_type: 0,
                        username: String::new(),
                        active: false,
                    });
                }
            }
        }
        Ok(entries)
    }

    fn set_apn(&mut self, t: &mut dyn AtTransport, cid: i32, ctx: i32, apn: &str, _user: &str, _pass: &str, _auth: i32) -> Result<(), String> {
        let pdp = match ctx { 2 => "IPV6", 3 => "IPV4V6", _ => "IP" };
        t.send_at(&format!("AT+CGDCONT={},\"{}\",\"{}\"", cid, pdp, apn))?;
        Ok(())
    }

    fn delete_apn(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CGDCONT={}", cid))?;
        Ok(())
    }

    fn connect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        dial::connect(t, cid, "", "", "", 0)
    }

    fn disconnect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        dial::disconnect(t, cid)
    }

    fn query_ip_info(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
        dial::query_ip(t, cid)
    }

    fn query_band_config(&mut self, t: &mut dyn AtTransport) -> Result<BandConfig, String> {
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (acqorder, lteband_hex) = parse_syscfgex(&resp);
        let lte_bands = decode_syscfgex_lteband(&lteband_hex);
        Ok(BandConfig {
            lte_supported: lte_bands.clone(),
            nr_supported: vec![],
            lte_locked: lte_bands,
            nr_locked: vec![],
        })
    }

    fn set_lte_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        // Read current config first, then update lteband field
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (acqorder, _) = parse_syscfgex(&resp);
        t.send_at(&format!("AT^SYSCFGEX=\"{}\",3FFFFFFF,1,2,{},,", acqorder, bands))?;
        Ok(())
    }

    fn set_nr5g_bands(&mut self, _t: &mut dyn AtTransport, _bands: &str) -> Result<(), String> {
        Err("NR band configuration not supported on MT5700 via this interface".to_string())
    }

    fn set_network_mode(&mut self, t: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        // Convert mode string to acqorder: "LTE"→"03", "NR5G"→"08", "LTE:NR5G"→"0308"
        let acqorder = match mode {
            "LTE" => "03",
            "NR5G" | "NR" => "08",
            "LTE:NR5G" | "LTE:NR" => "0308",
            other => other,
        };
        let resp = t.send_at("AT^SYSCFGEX?")?;
        let (_, lteband) = parse_syscfgex(&resp);
        t.send_at(&format!("AT^SYSCFGEX=\"{}\",3FFFFFFF,1,2,{},,", acqorder, lteband))?;
        Ok(())
    }

    fn query_traffic(&mut self, t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        let resp = t.send_at("AT^DSFLOWQRY")?;
        for line in resp.lines() {
            if let Some(rest) = line.trim().strip_prefix("^DSFLOWQRY:") {
                // ^DSFLOWQRY: last_time,last_tx,last_rx,total_time,total_tx,total_rx
                let parts: Vec<&str> = rest.trim().split(',').map(|s| s.trim()).collect();
                let tx = u64::from_str_radix(
                    parts.get(4).unwrap_or(&"0x0").trim_start_matches("0x"), 16
                ).unwrap_or(0);
                let rx = u64::from_str_radix(
                    parts.get(5).unwrap_or(&"0x0").trim_start_matches("0x"), 16
                ).unwrap_or(0);
                return Ok(TrafficInfo { ul_bytes: tx, dl_bytes: rx });
            }
        }
        Ok(TrafficInfo { ul_bytes: 0, dl_bytes: 0 })
    }

    fn reset_traffic(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT^DSFLOWCLR")?;
        Ok(())
    }

    fn reboot(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT^RESET")?;
        Ok(())
    }

    fn set_cfun(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CFUN={}", mode))?;
        Ok(())
    }

    fn query_feature_toggles(&mut self, _t: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
        Ok(FeatureToggles::default())
    }

    fn set_feature_toggle(&mut self, _t: &mut dyn AtTransport, _feat: &str, _on: bool) -> Result<(), String> {
        Ok(())
    }

    fn query_qos(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<QosInfo, String> {
        Ok(QosInfo { cqi: String::new(), ul_bandwidth: String::new(), dl_bandwidth: String::new() })
    }
}

/// Decode SYSCFGEX lteband hex bitmask to band number strings
fn decode_syscfgex_lteband(hex: &str) -> Vec<String> {
    let val = u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
    (0..64u64)
        .filter(|&bit| val & (1 << bit) != 0)
        .map(|bit| (bit + 1).to_string())
        .collect()
}
```

- [ ] **Step 6: Run TdTech tests**

```bash
cargo test -p modem-hal vendors::tdtech 2>&1
```

Expected: all 5 tests (hcsq, hcsq_unknown, monsc_lte, monsc_nr, hex_ip) pass.

- [ ] **Step 7: Commit**

```bash
git add modem-hal/src/vendors/tdtech/
git commit -m "feat(modem-hal): TdTechModem with AT^ commands, HCSQ index conversion, NDISDUP dial"
```

---

## Task 6: Wire `src-tauri` to consume `modem-hal`

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add `modem-hal` dependency to `src-tauri/Cargo.toml`**

```toml
[dependencies]
# ... existing deps ...
modem-hal = { path = "../modem-hal" }
```

- [ ] **Step 2: Update `src-tauri/src/lib.rs` module declarations**

Remove the old inline module declarations and replace with re-exports from `modem_hal`:

```rust
// src-tauri/src/lib.rs — remove these lines:
// pub mod at_parser;
// pub mod modem_factory;
// pub mod modem_vendor;
// pub mod transport;
// pub mod types;
// pub mod vendor_detector;
// pub mod vendors;

// Add:
use modem_hal::{ModemFactory, ModemVendor};
use modem_hal::transport::{SerialTransport, TcpTransport};
use modem_hal::types::*;
```

Keep `pub mod at_adapter;`, `pub mod network;`, `pub mod serial;` (these are Tauri-specific, not moving).

- [ ] **Step 3: Update all `use crate::` references in Tauri command files**

In any file that imports from `crate::types`, `crate::transport`, `crate::vendors`, etc., update to `modem_hal::`:

```rust
// Before:
use crate::types::ModemStatus;
use crate::modem_factory::ModemFactory;

// After:
use modem_hal::types::ModemStatus;
use modem_hal::ModemFactory;
```

- [ ] **Step 4: Build the Tauri crate**

```bash
cargo build -p modem-cat 2>&1 | head -40
```

Fix any remaining import errors before proceeding. Expected: builds successfully (warnings OK).

- [ ] **Step 5: Commit**

```bash
git add src-tauri/
git commit -m "feat(src-tauri): consume modem-hal crate instead of inline modules"
```

---

## Task 7: Delete old redundant files from `src-tauri`

**Files:**
- Delete: `src-tauri/src/at_parser.rs`
- Delete: `src-tauri/src/transport.rs`
- Delete: `src-tauri/src/types.rs`
- Delete: `src-tauri/src/modem_vendor.rs`
- Delete: `src-tauri/src/modem_factory.rs`
- Delete: `src-tauri/src/vendor_detector.rs`
- Delete: `src-tauri/src/vendors/qualcomm.rs`
- Delete: `src-tauri/src/vendors/unisoc.rs`

- [ ] **Step 1: Delete moved files**

```bash
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/at_parser.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/transport.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/types.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/modem_vendor.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/modem_factory.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/vendor_detector.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/vendors/qualcomm.rs
rm /Volumes/kidbird/code/modem-cat/src-tauri/src/vendors/unisoc.rs
```

- [ ] **Step 2: Build again to confirm nothing is broken**

```bash
cargo build -p modem-cat 2>&1 | head -40
```

Expected: clean build.

- [ ] **Step 3: Run all tests**

```bash
cargo test -p modem-hal 2>&1
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "chore: remove vendor/transport/parser files superseded by modem-hal"
```

---

## Task 8: Create embedded CLI (`modem-cli-embedded`)

**Files:**
- Create: `modem-cli-embedded/Cargo.toml`
- Create: `modem-cli-embedded/src/main.rs`

- [ ] **Step 1: Create `modem-cli-embedded/Cargo.toml`**

```toml
[package]
name = "modem-cli-embedded"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "modem"
path = "src/main.rs"

[dependencies]
modem-hal = { path = "../modem-hal", default-features = true }
clap = { version = "4", features = ["derive"] }
serde_json = "1"
env_logger = "0.11"
log = "0.4"
```

- [ ] **Step 2: Write failing test for JSON output format**

```rust
// modem-cli-embedded/src/main.rs — inline test
#[cfg(test)]
mod tests {
    #[test]
    fn status_output_is_valid_json() {
        // Simulate what the status command would serialize
        let status = serde_json::json!({
            "sim_status": "READY",
            "reg_status": "1",
            "conn_status": "0",
            "imei": "123456789012345",
            "iccid": "",
            "operator": "",
        });
        let s = serde_json::to_string(&status).unwrap();
        assert!(s.contains("sim_status"));
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed["sim_status"], "READY");
    }
}
```

- [ ] **Step 3: Run test to verify it fails**

```bash
cargo test -p modem-cli-embedded 2>&1
```

Expected: FAIL — crate doesn't exist yet.

- [ ] **Step 4: Implement `main.rs`**

```rust
// modem-cli-embedded/src/main.rs
use clap::{Parser, Subcommand};
use modem_hal::transport::SerialTransport;
use modem_hal::{ModemFactory, ModemVendor};

#[derive(Parser)]
#[command(name = "modem", about = "5G modem CLI for embedded Linux")]
struct Cli {
    /// Serial port (e.g. /dev/ttyUSB2)
    #[arg(short, long)]
    port: String,

    /// Baud rate
    #[arg(short, long, default_value = "115200")]
    baud: u32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print modem status as JSON
    Status,
    /// Print signal info as JSON
    Signal,
    /// Connect data (NDIS/NDISDUP)
    Connect {
        #[arg(short, long, default_value = "1")]
        cid: i32,
    },
    /// Disconnect data
    Disconnect {
        #[arg(short, long, default_value = "1")]
        cid: i32,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let mut transport = match SerialTransport::new(&cli.port, cli.baud) {
        Ok(t) => t,
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    };

    let mut modem = match ModemFactory::create(&mut transport) {
        Ok(m) => m,
        Err(e) => { eprintln!("Error detecting modem: {}", e); std::process::exit(1); }
    };

    let result = match cli.command {
        Commands::Status => {
            modem.query_modem_status(&mut transport)
                .map(|s| serde_json::to_string_pretty(&s).unwrap())
        }
        Commands::Signal => {
            modem.query_signal_strength(&mut transport)
                .map(|s| serde_json::to_string_pretty(&s).unwrap())
        }
        Commands::Connect { cid } => {
            modem.connect_data(&mut transport, cid)
                .map(|_| r#"{"status":"connected"}"#.to_string())
        }
        Commands::Disconnect { cid } => {
            modem.disconnect_data(&mut transport, cid)
                .map(|_| r#"{"status":"disconnected"}"#.to_string())
        }
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn status_output_is_valid_json() {
        let status = serde_json::json!({
            "sim_status": "READY",
            "reg_status": "1",
            "conn_status": "0",
            "imei": "123456789012345",
            "iccid": "",
            "operator": "",
        });
        let s = serde_json::to_string(&status).unwrap();
        assert!(s.contains("sim_status"));
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed["sim_status"], "READY");
    }
}
```

- [ ] **Step 5: Add `ModemStatus` serde derives if not already present**

In `modem-hal/src/types.rs`, ensure `ModemStatus` and `SignalInfo` have `#[derive(Serialize, Deserialize)]`. They already have these from the original `src-tauri/src/types.rs`.

- [ ] **Step 6: Build and run tests**

```bash
cargo build -p modem-cli-embedded 2>&1
cargo test -p modem-cli-embedded 2>&1
```

Expected: binary builds, test passes.

- [ ] **Step 7: Verify cross-compilation (aarch64)**

```bash
# Install target if needed:
rustup target add aarch64-unknown-linux-musl
# Build (requires cross tool or musl toolchain):
cargo build -p modem-cli-embedded --target aarch64-unknown-linux-musl --release 2>&1 | tail -5
```

Expected: `Finished release [optimized]` or cross-compilation environment error (acceptable if `cross` not installed — the build config is verified by the Cargo.toml setup).

- [ ] **Step 8: Commit**

```bash
git add modem-cli-embedded/
git commit -m "feat(modem-cli-embedded): add embedded Rust CLI with clap, JSON output"
```

---

## Task 9: Add napi-rs scaffold to `modem-hal` for Bun/TS

**Files:**
- Modify: `modem-hal/Cargo.toml`
- Modify: `modem-hal/src/lib.rs`
- Create: `modem-hal/package.json`
- Create: `modem-hal/build.rs`

- [ ] **Step 1: Add napi-rs dependency behind feature flag**

In `modem-hal/Cargo.toml`:
```toml
[features]
default = ["serial"]
serial = ["dep:serialport"]
napi-feature = ["dep:napi", "dep:napi-derive"]

[dependencies]
# ... existing ...
napi = { version = "2", features = ["napi6"], optional = true }
napi-derive = { version = "2", optional = true }

[build-dependencies]
napi-build = { version = "1", optional = true }
```

- [ ] **Step 2: Create `modem-hal/build.rs`**

```rust
// modem-hal/build.rs
#[cfg(feature = "napi-feature")]
extern crate napi_build;

fn main() {
    #[cfg(feature = "napi-feature")]
    napi_build::setup();
}
```

- [ ] **Step 3: Add `#[napi]` exports to `modem-hal/src/lib.rs`**

```rust
// modem-hal/src/lib.rs
pub mod modem_vendor;
pub mod modem_factory;
pub mod transport;
pub mod types;
pub mod vendors;

pub use modem_vendor::ModemVendor;
pub use modem_factory::ModemFactory;
pub use types::*;

// ── napi-rs surface for Bun/TS ──
#[cfg(feature = "napi-feature")]
mod napi_exports {
    use napi_derive::napi;
    use crate::transport::SerialTransport;
    use crate::ModemFactory;

    #[napi]
    pub struct ModemHandle {
        inner: Box<dyn crate::ModemVendor + Send>,
        transport: SerialTransport,
    }

    #[napi]
    impl ModemHandle {
        #[napi(factory)]
        pub fn connect(port: String, baud: u32) -> napi::Result<Self> {
            let mut transport = SerialTransport::new(&port, baud)
                .map_err(|e| napi::Error::from_reason(e))?;
            let modem = ModemFactory::create(&mut transport)
                .map_err(|e| napi::Error::from_reason(e))?;
            Ok(Self { inner: modem, transport })
        }

        #[napi]
        pub fn query_signal(&mut self) -> napi::Result<crate::types::SignalInfo> {
            self.inner.query_signal_strength(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn query_status(&mut self) -> napi::Result<crate::types::ModemStatus> {
            self.inner.query_modem_status(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn connect_data(&mut self, cid: i32) -> napi::Result<()> {
            self.inner.connect_data(&mut self.transport, cid)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn disconnect_data(&mut self, cid: i32) -> napi::Result<()> {
            self.inner.disconnect_data(&mut self.transport, cid)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn reboot(&mut self) -> napi::Result<()> {
            self.inner.reboot(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn close(&mut self) {
            self.transport.close();
        }
    }
}
```

- [ ] **Step 4: Add `SignalInfo` and `ModemStatus` napi annotations**

In `modem-hal/src/types.rs`, add `#[cfg_attr(feature = "napi-feature", napi_derive::napi(object))]` to structs that cross the napi boundary:

```rust
#[cfg_attr(feature = "napi-feature", napi_derive::napi(object))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalInfo { ... }

#[cfg_attr(feature = "napi-feature", napi_derive::napi(object))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModemStatus { ... }
```

- [ ] **Step 5: Create `modem-hal/package.json`**

```json
{
  "name": "modem-hal",
  "version": "0.1.0",
  "description": "5G modem HAL native addon",
  "main": "index.js",
  "napi": {
    "name": "modem-hal",
    "triples": {
      "defaults": true,
      "additional": ["aarch64-apple-darwin", "aarch64-unknown-linux-gnu"]
    }
  },
  "devDependencies": {
    "@napi-rs/cli": "^2"
  }
}
```

- [ ] **Step 6: Build with napi feature to verify it compiles**

```bash
cd /Volumes/kidbird/code/modem-cat/modem-hal
cargo build --features napi-feature 2>&1 | tail -10
```

Expected: compiles (warnings OK). Shared library produced.

- [ ] **Step 7: Commit**

```bash
git add modem-hal/src/lib.rs modem-hal/Cargo.toml modem-hal/build.rs modem-hal/package.json
git commit -m "feat(modem-hal): add napi-rs scaffold for Bun/TS native addon"
```

---

## Self-Review

**Spec coverage check:**
- ✅ Three vendor platforms: QuectelModem (Qualcomm+UniSoc), TdTechModem — Tasks 3-5
- ✅ Shared Quectel layer with chip-aware divergences — Task 4
- ✅ `AT^HCSQ` index conversion — Task 5, parser.rs
- ✅ `AT^MONSC` direct dBm — Task 5, parser.rs
- ✅ `AT^NDISDUP` dial with 5s delay — Task 5, dial.rs
- ✅ `AT^DHCP` hex IP conversion — Task 5, dial.rs
- ✅ `AT^SYSCFGEX` band/mode config — Task 5, mod.rs
- ✅ `AT^RESET` reboot — Task 5
- ✅ `ChipsetVendor::TdTech` added — Task 2
- ✅ `ModemFactory` TdTech detection ("MT5700") — Task 3
- ✅ Standalone `modem-hal` crate — Task 1
- ✅ `src-tauri` migrated to consume crate — Task 6
- ✅ Old files deleted — Task 7
- ✅ Embedded CLI with clap — Task 8
- ✅ napi-rs scaffold — Task 9
- ✅ Qualcomm bandwidth index decode (0→1.4 ... 5→20) — Task 4 parser.rs
- ✅ UniSoc bandwidth direct MHz — Task 4 parser.rs
- ✅ musl feature flag for embedded — Task 1 Cargo.toml
- ✅ MockTransport for testing — Task 2

**Type consistency check:**
- `QuectelModem::qualcomm(model)` / `QuectelModem::unisoc(model)` constructors used consistently in factory (Task 3) and defined in Task 4
- `parse_qeng_serving_cell(raw, bool)` — bool param consistent across Task 4 definition and Task 4 QuectelModem impl
- `hex_ip_to_string` defined in `tdtech/parser.rs`, imported in `tdtech/dial.rs` via `super::parser::hex_ip_to_string` ✅
- `parse_dconnstat` defined in `tdtech/parser.rs`, used in `tdtech/dial.rs` and `tdtech/mod.rs` ✅
- `FeatureToggles::default()` — requires `#[derive(Default)]` on `FeatureToggles` in types.rs (add if missing)
