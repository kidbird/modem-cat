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
    pub pci: String,
    pub cell_id: String,
    pub arfcn: String,
    pub bandwidth: String,
    pub rsrp: String,
    pub rsrq: String,
    pub sinr: String,
    pub tx_power: String,
    pub ant_values: [String; 4],
    pub scs: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpInfo {
    pub ipv4_addr: String,
    pub ipv4_mask: String,
    pub ipv4_gw: String,
    pub ipv4_dns: String,
    pub ipv6_addr: String,
    pub ipv6_gw: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighborCells {
    pub lte: Vec<NeighborCell>,
    pub nr: Vec<NeighborCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandConfig {
    pub lte_supported: Vec<String>,
    pub nr_supported: Vec<String>,
    pub lte_locked: Vec<String>,
    pub nr_locked: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureToggles {
    pub pcie_mode: bool,
    pub ethernet: bool,
    pub proxyarp: bool,
    pub uartat: bool,
    pub eth_at: bool,
    pub adb: bool,
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
