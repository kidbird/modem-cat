use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "napi-feature", napi_derive::napi(object))]
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
    pub ant_values: Vec<String>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighborCells {
    pub lte: Vec<NeighborCell>,
    pub nr: Vec<NeighborCell>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandConfig {
    /// Bands reported by AT+QNWPREFCFG=? (firmware-configurable range)
    pub lte_supported: Vec<String>,
    pub nr_supported: Vec<String>,
    /// Currently locked bands (from AT+QNWPREFCFG="lte_band" / "nr5g_band")
    pub lte_locked: Vec<String>,
    pub nr_locked: Vec<String>,
    /// Hardware spec bands from datasheet (static per model, not queried via AT)
    pub lte_spec: Vec<String>,
    pub nr_spec: Vec<String>,
}

/// Static hardware band specifications per model family.
/// Returns (lte_bands, nr_bands). Model string is matched as uppercase prefix.
pub fn spec_bands_for_model(model: &str) -> (Vec<String>, Vec<String>) {
    let upper = model.to_uppercase();

    // RM520N-GL and RM500Q-GL share the same spec
    if upper.contains("RM520N-GL") || upper.contains("RM500Q-GL") {
        return (
            bands_lte(&[
                1, 2, 3, 4, 5, 7, 8, 12, 13, 14, 17, 18, 19, 20, 25, 26, 28, 29, 30, 32, 66, 71,
                34, 38, 39, 40, 41, 42, 43, 48,
            ]),
            bands_nr(&[
                1, 2, 3, 5, 7, 8, 12, 13, 14, 18, 20, 25, 26, 28, 29, 30, 38, 40, 41, 48, 66, 70,
                71, 75, 76, 77, 78, 79,
            ]),
        );
    }

    // RM520N-CN and RM500Q-CN share the same spec
    if upper.contains("RM520N-CN") || upper.contains("RM500Q-CN") {
        return (
            bands_lte(&[1, 3, 5, 8, 34, 38, 39, 40, 41]),
            bands_nr(&[1, 3, 5, 8, 28, 41, 78, 79]),
        );
    }

    // RM520N-EU
    if upper.contains("RM520N-EU") {
        return (
            bands_lte(&[1, 3, 5, 7, 8, 20, 28, 32, 71, 38, 40, 41, 42, 43]),
            bands_nr(&[1, 3, 5, 7, 8, 20, 28, 38, 40, 41, 71, 75, 76, 77, 78]),
        );
    }

    // RM500U-CNV and RG200U-CN (展锐 UniSoc)
    if upper.contains("RM500U-CNV") || upper.contains("RG200U-CN") {
        return (
            bands_lte(&[1, 3, 5, 8, 34, 38, 39, 40, 41]),
            bands_nr(&[1, 3, 5, 8, 28, 41, 77, 78, 79]),
        );
    }

    // RM500U-EA (展锐 UniSoc)
    if upper.contains("RM500U-EA") {
        return (
            bands_lte(&[1, 2, 3, 4, 5, 7, 8, 20, 28, 66, 38, 40, 41]),
            bands_nr(&[1, 3, 5, 7, 8, 20, 28, 38, 40, 41, 66, 77, 78]),
        );
    }

    (vec![], vec![])
}

fn bands_lte(nums: &[u32]) -> Vec<String> {
    let mut v: Vec<u32> = nums.to_vec();
    v.sort_unstable();
    v.dedup();
    v.iter().map(|n| format!("B{}", n)).collect()
}

fn bands_nr(nums: &[u32]) -> Vec<String> {
    let mut v: Vec<u32> = nums.to_vec();
    v.sort_unstable();
    v.dedup();
    v.iter().map(|n| format!("n{}", n)).collect()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default)]
pub struct TemperatureInfo {
    pub soc_temp: String,
    pub pa_temp: String,
}

/// Baseline information (unified format)
#[derive(Debug, Clone, Default)]
pub struct BaselineInfo {
    pub ap_baseline: String,
    pub cp_baseline: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rm520n_gl_spec_bands() {
        let (lte, nr) = spec_bands_for_model("RM520N-GL");
        assert!(lte.contains(&"B1".to_string()));
        assert!(lte.contains(&"B66".to_string()));
        assert!(nr.contains(&"n78".to_string()));
        assert!(nr.contains(&"n79".to_string()));
        assert!(nr.contains(&"n41".to_string()));
    }

    #[test]
    fn rm520n_cn_spec_bands() {
        let (lte, nr) = spec_bands_for_model("RM520N-CN");
        assert!(lte.contains(&"B1".to_string()));
        assert!(lte.contains(&"B41".to_string()));
        assert!(!lte.contains(&"B66".to_string()));
        assert!(nr.contains(&"n78".to_string()));
        assert!(!nr.contains(&"n77".to_string()));
    }

    #[test]
    fn rm520n_eu_spec_bands() {
        let (lte, nr) = spec_bands_for_model("RM520N-EU");
        assert!(lte.contains(&"B20".to_string()));
        assert!(nr.contains(&"n77".to_string()));
        assert!(!nr.contains(&"n79".to_string()));
    }

    #[test]
    fn rm500q_gl_matches_rm520n_gl() {
        let (lte1, nr1) = spec_bands_for_model("RM500Q-GL");
        let (lte2, nr2) = spec_bands_for_model("RM520N-GL");
        assert_eq!(lte1, lte2);
        assert_eq!(nr1, nr2);
    }

    #[test]
    fn rm500q_cn_matches_rm520n_cn() {
        let (lte1, nr1) = spec_bands_for_model("RM500Q-CN");
        let (lte2, nr2) = spec_bands_for_model("RM520N-CN");
        assert_eq!(lte1, lte2);
        assert_eq!(nr1, nr2);
    }

    #[test]
    fn rm500u_cnv_spec_bands() {
        let (lte, nr) = spec_bands_for_model("RM500U-CNV");
        assert!(lte.contains(&"B34".to_string()));
        assert!(lte.contains(&"B41".to_string()));
        assert!(!lte.contains(&"B66".to_string()));
        assert!(nr.contains(&"n77".to_string()));
        assert!(nr.contains(&"n78".to_string()));
        assert!(nr.contains(&"n79".to_string()));
    }

    #[test]
    fn rg200u_cn_matches_rm500u_cnv() {
        let (lte1, nr1) = spec_bands_for_model("RG200U-CN");
        let (lte2, nr2) = spec_bands_for_model("RM500U-CNV");
        assert_eq!(lte1, lte2);
        assert_eq!(nr1, nr2);
    }

    #[test]
    fn rm500u_ea_spec_bands() {
        let (lte, nr) = spec_bands_for_model("RM500U-EA");
        assert!(lte.contains(&"B1".to_string()));
        assert!(lte.contains(&"B66".to_string()));
        assert!(lte.contains(&"B41".to_string()));
        assert!(!lte.contains(&"B12".to_string()));
        assert!(nr.contains(&"n77".to_string()));
        assert!(nr.contains(&"n78".to_string()));
        assert!(!nr.contains(&"n79".to_string()));
    }

    #[test]
    fn unknown_model_returns_empty() {
        let (lte, nr) = spec_bands_for_model("RM999X-XX");
        assert!(lte.is_empty());
        assert!(nr.is_empty());
    }
}
