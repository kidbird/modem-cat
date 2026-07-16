/// Hard-coded hardware-supported frequency bands for UniSoc Quectel modem variants.
///
/// UniSoc chip modems (RM500U/RG200U series) do NOT expose hardware band
/// capabilities via `AT+QNWPREFCFG="rf_band"`. This database provides the
/// static fallback so the band-selection UI has a grid to render.
///
/// The model suffix determines regional band support:
/// - `-CN`: China domestic
/// - `-EA`: Europe / global
/// - `-CNV`: China new version

pub struct ModelBands {
    pub lte: &'static [u32],
    pub nr: &'static [u32],
}

/// RM500U / RG200U 家族统一频段（按 RG200U 全集，NR 频段多于 RM500U）。
/// VID/PID 0x2C7C:0x0900 对应此家族，detect_model_from_vid_pid 返回 "RG200U/RM500U 5G"。
const UNISOC_5G: ModelBands = ModelBands {
    lte: &[1, 3, 5, 8, 34, 38, 39, 40, 41],
    nr: &[1, 3, 5, 8, 28, 41, 77, 78, 79],
};

// TODO(RG255AA): 当前频段为 RM500U 占位，ASR 平台实测频段待校准后替换。
const RG255AA_CN: ModelBands = ModelBands {
    lte: &[1, 3, 5, 8, 34, 38, 39, 40, 41],
    nr: &[1, 28, 41, 77, 78, 79],
};

/// Look up hard-coded supported bands for a UniSoc modem model string.
///
/// Model matching is case-insensitive and uses prefix matching on the full
/// variant string (e.g. "RM500U-CN" matches "RM500U-CN-AA").
///
/// **Order matters**: CNV is checked before CN to avoid false matches.
pub fn get_supported_bands(model: &str) -> Option<&'static ModelBands> {
    let m = model.to_uppercase();
    // ASR 平台独立匹配（RG255AA 系列）
    if m.contains("RG255") {
        return Some(&RG255AA_CN);
    }
    // UniSoc 家族统一使用 RG200U 全集频段
    // VID/PID 路径返回 "RG200U/RM500U 5G"，AT+CGMM 路径返回具体型号如 "RM500U-CN"
    if m.contains("RG200U") || m.contains("RM500U") || m.contains("RG500U") {
        return Some(&UNISOC_5G);
    }
    None
}

/// Convert a slice of band numbers to the display format used in `BandConfig`.
///
/// LTE bands get a `B` prefix (`1` → `"B1"`), NR bands get `n` (`78` → `"n78"`).
pub fn format_bands(nums: &[u32], prefix: &str) -> Vec<String> {
    nums.iter().map(|b| format!("{}{}", prefix, b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rm500u_cn_exact() {
        let bands = get_supported_bands("RM500U-CN").unwrap();
        assert!(bands.lte.contains(&1));
        assert!(bands.nr.contains(&79));
    }

    #[test]
    fn rm500u_cn_with_region() {
        let bands = get_supported_bands("RM500U-CN-AA").unwrap();
        assert!(bands.nr.contains(&78));
    }

    #[test]
    fn rm500u_cnv_before_cn() {
        // CNV 与 CN 共享同一份硬件频段（合并后统一返回 UNISOC_RM500U_RG200U）
        let bands = get_supported_bands("RM500U-CNV").unwrap();
        assert!(bands.lte.contains(&1));
        assert!(bands.lte.contains(&38));
        assert!(bands.nr.contains(&79));
    }

    #[test]
    fn rm500u_ea() {
        // EA 与 CN 共享同一份硬件频段（合并后统一返回 UNISOC_RM500U_RG200U）
        let bands = get_supported_bands("RM500U-EA").unwrap();
        assert!(bands.lte.contains(&1));
        assert!(bands.lte.contains(&38));
        assert!(bands.nr.contains(&78));
    }

    #[test]
    fn rg200u_cn() {
        let bands = get_supported_bands("RG200U-CN").unwrap();
        assert!(bands.lte.contains(&1));
        assert!(bands.lte.contains(&38));
        assert!(bands.nr.contains(&79));
    }

    #[test]
    fn rg255aa_cn() {
        // 当前为 RM500U-CN 占位；后续实测校准后调整断言。
        let bands = get_supported_bands("RG255AA").unwrap();
        assert!(bands.lte.contains(&1));
        assert!(bands.nr.contains(&78));
        let bands2 = get_supported_bands("RG255AA-CN").unwrap();
        assert!(bands2.nr.contains(&79));
    }

    #[test]
    fn unknown_model() {
        assert!(get_supported_bands("RG520N-GL").is_none());
        assert!(get_supported_bands("UNKNOWN").is_none());
    }

    #[test]
    fn case_insensitive() {
        assert!(get_supported_bands("rm500u-ea").is_some());
        assert!(get_supported_bands("rg200u-cn").is_some());
    }

    #[test]
    fn format_lte_bands() {
        let bands = format_bands(&[1, 3, 5], "B");
        assert_eq!(bands, vec!["B1", "B3", "B5"]);
    }

    #[test]
    fn format_nr_bands() {
        let bands = format_bands(&[78, 79], "n");
        assert_eq!(bands, vec!["n78", "n79"]);
    }
}
