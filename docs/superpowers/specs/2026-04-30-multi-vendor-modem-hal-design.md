# Multi-Vendor 5G Modem HAL Design

**Date:** 2026-04-30  
**Status:** Approved  
**Approach:** B — Quectel shared layer + independent TdTech

## Overview

Add a unified Hardware Abstraction Layer (HAL) in the Rust backend (`src-tauri/`) that supports three 5G modem platforms through a single `ModemVendor` trait interface. After auto-detecting the modem model via AT commands, the factory creates the appropriate adapter; all callers (CLI, Tauri frontend) work against the same trait.

**Platforms in scope:**
| Platform | Models | Chipset | Detection string |
|----------|--------|---------|-----------------|
| Quectel Qualcomm | RG520N, RM520N, RG525F, RG530F, RM530N, RG540F, RM540N | SDX55/SDX65 | contains above |
| Quectel UniSoc | RG200U, RG500U, RM500U, RG501U, RM501U | 展锐 | contains above |
| TDTech | MT5700M-CN | Huawei-derived | contains "MT5700" |

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│             Callers (CLI / Tauri commands)           │
└────────────────────────┬────────────────────────────┘
                         │  Box<dyn ModemVendor>
┌────────────────────────▼────────────────────────────┐
│                ModemVendor trait                     │
│  query_signal_strength, query_serving_cell,          │
│  connect_data, set_lte_bands, query_traffic, ...     │
└───────────┬──────────────────────────┬──────────────┘
            │                          │
┌───────────▼──────────┐  ┌────────────▼─────────────┐
│    QuectelModem       │  │      TdTechModem          │
│  chip: QuectelChip   │  │  Full independent impl    │
│  ┌──────┬──────────┐ │  │  AT^ prefix commands      │
│  │Qualcomm│ UniSoc  │ │  └───────────────────────────┘
│  │overrides overrides│
│  └──────┴──────────┘ │
└──────────────────────┘
            │
┌───────────▼──────────┐
│   AtTransport trait  │
│   Serial / TCP       │
└──────────────────────┘
```

---

## File Structure

```
src-tauri/src/
├── modem_vendor.rs          # ModemVendor trait (unchanged)
├── modem_factory.rs         # Factory: add TdTech detection + MT5700 model
├── vendor_detector.rs       # AT+CGMM auto-detection (add TdTech branch)
├── types.rs                 # Add ChipsetVendor::TdTech variant
└── vendors/
    ├── mod.rs
    ├── quectel/             # NEW: replaces vendors/qualcomm.rs + vendors/unisoc.rs
    │   ├── mod.rs           # QuectelModem struct, ModemVendor impl, shared AT commands
    │   ├── parser.rs        # AT+QENG parser with QuectelChip-aware bandwidth lookup
    │   ├── qualcomm.rs      # QualcommModem type alias + overrides (QMAP dial, bandwidth table)
    │   └── unisoc.rs        # UniSocModem type alias + overrides (QNETDEVCTL dial, bandwidth table)
    └── tdtech/              # NEW: fully independent TdTech implementation
        ├── mod.rs           # TdTechModem struct + ModemVendor impl
        ├── parser.rs        # AT^HCSQ/AT^MONSC/AT^DHCP/AT^SYSCFGEX parsers
        └── dial.rs          # AT^NDISDUP dial logic + AT^DHCP IP extraction
```

---

## Component Design

### `ChipsetVendor` enum (types.rs)

Add `TdTech` variant:
```rust
pub enum ChipsetVendor {
    UniSoc,
    Qualcomm,
    TdTech,   // NEW
    Unknown,
}
```

### `QuectelModem` (vendors/quectel/mod.rs)

Single struct handles both Qualcomm and UniSoc. Shared AT commands go in the `ModemVendor` impl directly; per-chip divergences use a `match self.chip { ... }` branch.

```rust
pub enum QuectelChip { Qualcomm, UniSoc }

pub struct QuectelModem {
    chip: QuectelChip,
    model: String,
}
```

**Shared (same AT command, same parsing):**
- `AT+CPIN?` / `AT+CIMI` — SIM status
- `AT+CGSN` — IMEI
- `AT+ICCID` / `AT+QCCID` — ICCID
- `AT+CGMM` / `AT+CGMR` / `AT+CGMI` — hardware info
- `AT+COPS?` — operator
- `AT+CREG?` / `AT+CEREG?` — registration status
- `AT+CGDCONT?` — APN list query
- `AT+CGDCONT=` — APN set/delete
- `AT+QENG="servingcell"` — serving cell (shared command, chip-aware bandwidth decode)
- `AT+QENG="neighbourcell"` — neighbor cells
- `AT+CFUN=1,1` — reboot
- `AT+CFUN=` — set functionality mode

**Per-chip divergences:**
| Operation | Qualcomm | UniSoc |
|-----------|----------|--------|
| Data connect | `AT+QMAP="connect",<cid>` | `AT+QNETDEVCTL=1,<cid>,1` |
| Data disconnect | `AT+QMAP="disconnect",<cid>` | `AT+QNETDEVCTL=0,<cid>,1` |
| Connection status | `AT+QMAP="WWAN"` | `AT+QNETDEVSTATUS=<cid>` |
| IP info | `AT+QMAP="WWAN"` parse | `AT+QNETDEVSTATUS=<cid>` parse |
| Bandwidth decode | lookup table (0→1.4, 1→3, ..., 5→100 MHz) | direct integer MHz |
| LTE band set | `AT+QNWPREFCFG="lte_band",<hex>` | `AT+QNWPREFCFG="lte_band",<hex>` (same) |
| NR band set | `AT+QNWPREFCFG="nr5g_band",<hex>` | `AT+QNWPREFCFG="nr5g_band",<hex>` (same) |
| Network mode | `AT+QNWPREFCFG="mode_pref",<mode>` | `AT+QNWPREFCFG="mode_pref",<mode>` (same) |
| Traffic | `AT+QGDCNT?` | `AT+QGDCNT?` (same) |

### `AT+QENG` Bandwidth Decode (vendors/quectel/parser.rs)

The `ServingCellInfo.bandwidth` field maps differently:
- **Qualcomm**: integer index → actual MHz: `0→1.4, 1→3, 2→5, 3→10, 4→15, 5→20, 6→100`
- **UniSoc**: direct integer value in MHz (e.g., `100` means 100 MHz)

Parser accepts `QuectelChip` to select decode path.

### `TdTechModem` (vendors/tdtech/mod.rs)

Completely independent implementation. No shared code with Quectel layer.

**Signal (two commands merged into one logical query):**

`query_signal_strength`:
```
AT^HCSQ? → ^HCSQ: "LTE",<rssi_idx>,<rsrp_idx>,<sinr_idx>,<rsrq_idx>
```
Index → dBm conversion:
- `rsrp_actual = rsrp_idx - 140` (idx 0-97, 255=unknown)
- `sinr_actual = (sinr_idx as f32 * 0.2) - 20.0` (idx 0-251, 255=unknown)
- `rsrq_actual = (rsrq_idx as f32 * 0.5) - 19.5` (idx 0-34, 255=unknown)

`query_serving_cell`:
```
AT^MONSC → ^MONSC: LTE,<MCC>,<MNC>,<ARFCN>,<Cell_ID(hex)>,<PCI(hex)>,<TAC(hex)>,<RSRP>,<RSRQ>,<RSSI>
           ^MONSC: NR,<MCC>,<MNC>,<ARFCN>,<SCS>,<Cell_ID(hex)>,<PCI(hex)>,<TAC(hex)>,<RSRP>,<RSRQ>,<SINR>
```
MONSC gives direct dBm — no conversion needed. Cell_ID, PCI, TAC are hex strings, parse with `u64::from_str_radix(s, 16)`.

**Data dial (vendors/tdtech/dial.rs):**
```
connect:    AT^NDISDUP=<cid>,1[,<APN>,<user>,<pass>,<auth>]
disconnect: AT^NDISDUP=<cid>,0
status:     AT^DCONNSTAT?  → ^DCONNSTAT: <cid>,"<APN>",<ipv4_stat>,<ipv6_stat>,<type>[,<ether_stat>]
IP:         AT^DHCP=<cid>  → ^DHCP: <clip_hex>,<netmask_hex>,<gate_hex>,<dhcp_hex>,<pDNS_hex>,<sDNS_hex>,<max_rx>,<max_tx>
```

IP hex → dotted decimal conversion: `0xC0A80101` (big-endian u32) → `192.168.1.1`
```rust
fn hex_ip_to_string(hex: &str) -> String {
    let n = u32::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
    // TdTech encodes big-endian (MSB first)
    let bytes = n.to_be_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}
```

**Band/mode config:**
```
AT^SYSCFGEX=<acqorder>,<band>,<roam>,<srvdomain>,<lteband>,,
```
- Query: `AT^SYSCFGEX?`
- acqorder string: `"03"`=LTE only, `"08"`=NR only, `"0308"`=LTE+NR, `"030802"`=LTE+NR+WCDMA
- lteband: hex bitmask (bit N = band N+1). Band 1=0x1, Band 3=0x4, Band 28=0x8000000

**Traffic:**
```
query:  AT^DSFLOWQRY[=<cid>]  → ^DSFLOWQRY: <last_ds_time>,<last_tx_hex>,<last_rx_hex>,<total_time>,<total_tx_hex>,<total_rx_hex>
reset:  AT^DSFLOWCLR
```
tx/rx values are 16-digit hex, parse as u64.

**Reboot:** `AT^RESET` (no parameters, unlike `AT+CFUN=1,1`)

**ICCID:** `AT^ICCID?` → `^ICCID: <iccid>`

**SIM status:** `AT^CARDMODE` → `^CARDMODE: <type>` (0=no card, 1=SIM, 2=USIM)

### `ModemFactory` updates (modem_factory.rs)

```rust
// detect_vendor_from_model additions:
let tdtech_models = ["MT5700"];
// ...
ChipsetVendor::TdTech => Ok(Box::new(TdTechModem::new(model)))
```

`VendorDetector` also needs a TdTech branch: if `AT^CARDMODE` returns `^CARDMODE:` prefix, it's TdTech (or check manufacturer string from `AT+CGMI`).

---

## Data Flow

### Connection setup sequence
```
1. AtTransport::connect() (serial open or TCP connect)
2. ModemFactory::create(transport)
   a. VendorDetector::detect() → send AT+CGMM, parse model string
   b. match vendor → instantiate QuectelModem or TdTechModem
3. Caller holds Box<dyn ModemVendor>
4. All queries go through trait methods
```

### Signal query (TdTech)
```
query_modem_status()
  → query_signal_strength() → AT^HCSQ → parse indexed values → SignalInfo
  → query_serving_cell()    → AT^MONSC → parse direct dBm → ServingCellInfo
  → (combine into ModemStatus via default trait impl)
```

---

## Error Handling

- All trait methods return `Result<T, String>` — `String` carries the AT error text or parse failure description.
- `AT^DHCP` requires 5s delay after `AT^NDISDUP` before querying. `TdTechModem::connect_data()` should `std::thread::sleep(Duration::from_secs(5))` then verify with `AT^DCONNSTAT?` before returning.
- 255 index values in `AT^HCSQ` mean "unknown/unmeasurable" — map these to `Option<f32>` or a sentinel in `SignalInfo`. The existing `SignalInfo` struct uses `f32` — treat unknown as `f32::NAN` or add `Option` wrapping (prefer `Option<f32>` for correctness).
- `AT^MONSC` returns `-1256` / `-348` / `-188` as invalid sentinels — clamp or map to `None`.

---

## Testing Strategy

- Unit tests for each parser function (`parse_hcsq`, `parse_monsc`, `parse_qeng_serving_cell`) using captured AT response strings from real hardware.
- `#[cfg(test)]` mock transport that returns pre-recorded responses.
- Integration test: factory creates correct adapter type from model string without hardware.
- No mocking of the `ModemVendor` trait itself at this layer — test the concrete structs directly.

---

## Build & Library Compatibility

The Rust crate already configures `crate-type = ["staticlib", "cdylib", "rlib"]` in `Cargo.toml`. The new code adds no FFI-unsafe constructs. `Box<dyn ModemVendor>` is not `#[repr(C)]` — external C callers must use a thin C wrapper layer (out of scope for this spec).

---

## Out of Scope

- Traffic statistics persistence across reboots
- URC (unsolicited response code) event handling / async notification
- Frequency lock (`AT^FREQLOCK`, `AT^LTEFREQLOCK`, `AT^NRFREQLOCK`) for TdTech — noted as a future extension
- CA (carrier aggregation) query for Quectel (`AT+QCAINFO`) and TdTech (`AT^LENDC`)
- Temperature query for TdTech (no equivalent to `AT+QTEMP` found in manual)
