use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModemStatus {
    pub sim_status: String,
    pub reg_status: String,
    pub conn_status: String,
    pub imei: String,
    pub iccid: String,
    pub operator: String,
    pub network_type: String,
    pub band: String,
    pub pci: String,
    pub cell_id: String,
    pub arfcn: String,
    pub bandwidth: String,
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub tx_power: String,
    pub rx_level: String,
    pub ant_values: Vec<String>,
    pub scs: String,
    pub chip_vendor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QosInfo {
    pub cqi: String,
    pub ul_bandwidth: String,
    pub dl_bandwidth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub model: String,
    pub manufacturer: String,
    pub firmware: String,
    pub ap_baseline: String,
    pub cp_baseline: String,
    pub soc_temp: String,
    pub pa_temp: String,
    pub usb_vid: Option<u16>,
    pub usb_pid: Option<u16>,
    /// Modem serial number, read via `AT+EGMR=0,5`. Populated by
    /// query_hardware_info; empty string if the command fails (not all
    /// platforms/ firmware variants support it).
    pub serial_number: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpInfo {
    pub ipv4_addr: String,
    pub ipv4_mask: String,
    pub ipv4_gw: String,
    pub ipv4_dns: String,
    pub ipv6_addr: String,
    pub ipv6_dns: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanConfig {
    pub gateway: String,
    pub netmask: Option<String>,
    pub dhcp_start: String,
    pub dhcp_end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApnEntry {
    pub cid: i32,
    pub apn_name: String,
    pub ip_type: String,
    pub auth_type: i32,
    pub username: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L5GanEntry {
    pub cid: i32,
    pub enabled: bool,
    pub vlan_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Qualcomm5GlanStatus {
    pub eth_pdu_enabled: bool,
    pub mpdn_cid: Option<i32>,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellLockEntry {
    /// "cell" (QNWLOCK, has PCI) or "freq" (QNWLOCKFREQ, no PCI)
    pub lock_type: String,
    pub arfcn: String,
    pub pci: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighborCell {
    pub cell_id: String,
    pub pci: String,
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub earfcn: String,
    pub arfcn: String,
    pub offset: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighborCells {
    pub lte: Vec<NeighborCell>,
    pub nr: Vec<NeighborCell>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandConfig {
    /// Currently locked bands (from AT+QNWPREFCFG="lte_band" / "nr5g_band")
    pub lte_locked: Vec<String>,
    pub nr_locked: Vec<String>,
    /// Supported bands queried from modem via AT+QNWPREFCFG="rf_band"
    pub lte_spec: Vec<String>,
    pub nr_spec: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureToggles {
    pub pcie_mode: bool,
    pub ethernet: bool,
    pub proxyarp: bool,
    pub uart_at: bool,
    pub eth_at: bool,
    pub adb: bool,
    pub napt: bool,
    pub netmask: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualcommConfig {
    pub usbnet: i32,
    pub data_interface: String,
    pub pcie_mode: i32,
    pub usbspeed: String,
    pub eth_driver: String,
    pub ippt_mode: i32,
    pub auto_connect: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficInfo {
    pub ul_bytes: u64,
    pub dl_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    pub port_name: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub usb_vid: Option<u16>,
    pub usb_pid: Option<u16>,
    pub detected_model: Option<String>,
    pub detected_chipset: Option<String>,
    pub is_at_port: bool,
    pub display_name: String,
}

/// Vendor/Chipset identification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChipsetVendor {
    UniSoc,   // 展锐 - RG200U, RM500U系列
    Qualcomm, // 高通 - RG520N, RM520N系列
    Asr,      // ASR  - RG255AA 系列（当前 AT 指令集复用 UniSoc）
    Unknown,
}

impl ChipsetVendor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChipsetVendor::UniSoc => "unisoc",
            ChipsetVendor::Qualcomm => "qualcomm",
            ChipsetVendor::Asr => "asr",
            ChipsetVendor::Unknown => "unknown",
        }
    }
}

/// Serving cell information (unified format)
#[derive(Debug, Clone, Default)]
pub struct ServingCellInfo {
    pub connected: bool,
    pub mobility_state: String,
    pub tech: String,
    pub operator_mcc: String,
    pub operator_mnc: String,
    pub cell_id: String,
    pub pci: String,
    pub arfcn: String,
    pub band: String,
    pub bandwidth: String,
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub tx_power: String,
    pub rx_level: String,
    pub scs: String,
}

/// Signal strength information (unified format)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalInfo {
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub ant_values: Vec<String>,
}

/// Temperature information (unified format)
#[derive(Debug, Clone, Default)]
pub struct TemperatureInfo {
    pub soc_temp: String,
    pub pa_temp: String,
}

/// Baseline information (unified format)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineInfo {
    pub ap_baseline: String,
    pub cp_baseline: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the HardwareInfo payload that get_hardware_info returns
    /// carries USB VID/PID under camelCase keys (`usbVid`, `usbPid`) so the
    /// frontend `formatUsbVidPid(hw.usbVid, hw.usbPid)` call in app.js reads
    /// them correctly. If this test fails, the rename_all attribute on
    /// HardwareInfo has been removed/changed and the 模组设置 page will show
    /// `--` for USB VID/PID.
    #[test]
    fn hardware_info_json_uses_camelcase_usb_fields() {
        let info = HardwareInfo {
            model: "RM500U".to_string(),
            manufacturer: "Quectel".to_string(),
            firmware: "RM500UQDLAR02A03M4G".to_string(),
            ap_baseline: "M4G_01".to_string(),
            cp_baseline: "M4G_01".to_string(),
            soc_temp: "30.0".to_string(),
            pa_temp: "31.0".to_string(),
            usb_vid: Some(0x2C7C),
            usb_pid: Some(0x0900),
            serial_number: String::new(),
        };
        let json = serde_json::to_string(&info).expect("serialize");
        assert!(
            json.contains("\"usbVid\":11388"),
            "expected usbVid as camelCase key with decimal 0x2C7C, got: {json}"
        );
        assert!(
            json.contains("\"usbPid\":2304"),
            "expected usbPid as camelCase key with decimal 0x0900, got: {json}"
        );
        // Frontend decoding path: Tauri serialises back from the same
        // HardwareInfo struct, but a round-trip through serde_json also
        // exercises the #[serde(rename_all = "camelCase")] Deserialize impl.
        let parsed: HardwareInfo = serde_json::from_str(&json).expect("round-trip");
        assert_eq!(parsed.usb_vid, Some(0x2C7C));
        assert_eq!(parsed.usb_pid, Some(0x0900));
    }

    /// The frontend can also receive HardwareInfo with USB IDs set to null
    /// (e.g. when connected over TCP/WebSocket or when serialport failed to
    /// resolve the port's VID/PID). Make sure the None case round-trips as
    /// `null` and not as a missing key, otherwise the JS side would see
    /// `undefined` and `formatUsbVidPid` would still return `--`.
    #[test]
    fn hardware_info_json_round_trips_null_usb_ids() {
        let info = HardwareInfo {
            model: "RM500U".to_string(),
            manufacturer: "Quectel".to_string(),
            firmware: "RM500UQDLAR02A03M4G".to_string(),
            ap_baseline: String::new(),
            cp_baseline: String::new(),
            soc_temp: String::new(),
            pa_temp: String::new(),
            usb_vid: None,
            usb_pid: None,
            serial_number: String::new(),
        };
        let value = serde_json::to_value(&info).expect("to_value");
        assert!(
            value.get("usbVid").is_some(),
            "usbVid must be present (null), not missing"
        );
        assert!(
            value.get("usbPid").is_some(),
            "usbPid must be present (null), not missing"
        );
        assert!(value["usbVid"].is_null());
        assert!(value["usbPid"].is_null());
    }
}
