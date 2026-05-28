use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi-feature", napi_derive::napi(object))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtTimingEntry {
    pub command: String,
    pub duration_ms: u64,
    pub success: bool,
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtTimingStats {
    pub entries: Vec<AtTimingEntry>,
    pub total_ms: u64,
    pub init_start_ms: u64,
    pub init_end_ms: u64,
}

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
    pub is_at_port: bool,
    pub display_name: String,
}

/// Vendor/Chipset identification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChipsetVendor {
    UniSoc,   // 展锐 - RG200U, RM500U系列
    Qualcomm, // 高通 - RG520N, RM520N系列
    TdTech,   // 鼎桥 MT5700 series
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
#[cfg_attr(feature = "napi-feature", napi_derive::napi(object))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalInfo {
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub ant_values: Vec<String>,
}

/// Temperature information (unified format)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
