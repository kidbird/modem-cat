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

    pub fn create_from_vendor(
        model: String,
        vendor: ChipsetVendor,
    ) -> Result<Box<dyn ModemVendor>, String> {
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
                log::error!("Unknown vendor for '{}'", model);
                Err(format!("Unknown modem vendor for model '{}'", model))
            }
        }
    }

    pub fn detect_vendor_from_model(model: &str) -> ChipsetVendor {
        let upper = model.to_uppercase();
        let tdtech = ["MT5700"];
        for m in &tdtech {
            if upper.contains(m) {
                return ChipsetVendor::TdTech;
            }
        }
        let qualcomm = [
            "RG500Q", "RM500Q", "RG520N", "RM520N", "RG525F",
            "RG530F", "RM530F", "RM530N", "RM551E", "RM501Q",
            "RG540F", "RM540N",
        ];
        for m in &qualcomm {
            if upper.contains(m) {
                return ChipsetVendor::Qualcomm;
            }
        }
        let unisoc = ["RG200U", "RM500U", "RG500U", "RG501U", "RM501U"];
        for m in &unisoc {
            if upper.contains(m) {
                return ChipsetVendor::UniSoc;
            }
        }
        ChipsetVendor::Unknown
    }

    fn query_model(transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+CGMM")?;
        for line in resp.lines() {
            let t = line.trim();
            if t.starts_with("AT+") || t == "OK" || t.starts_with("ERROR") {
                continue;
            }
            if let Some(rest) = t.strip_prefix("+CGMM:") {
                return Ok(rest.trim().to_string());
            }
            if !t.is_empty() && !t.starts_with('+') {
                return Ok(t.to_string());
            }
        }
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChipsetVendor;

    #[test]
    fn detects_qualcomm_from_model() {
        for model in &[
            "RM500Q-GL", "RM500Q-CN", "RG500Q-EA",
            "RM520N-GL", "RG520N-CN",
            "RM551E-GL",
            "RM530F-CN", "RM530N-EU",
            "RG530F-EU", "RG525F-CN",
            "RM501Q-AE",
        ] {
            assert_eq!(
                ModemFactory::detect_vendor_from_model(model),
                ChipsetVendor::Qualcomm,
                "Expected Qualcomm for {}",
                model
            );
        }
    }

    #[test]
    fn detects_unisoc_from_model() {
        assert_eq!(
            ModemFactory::detect_vendor_from_model("RG200U-CN"),
            ChipsetVendor::UniSoc
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_model("RM500U-GL"),
            ChipsetVendor::UniSoc
        );
    }

    #[test]
    fn detects_tdtech_from_model() {
        assert_eq!(
            ModemFactory::detect_vendor_from_model("MT5700M-CN"),
            ChipsetVendor::TdTech
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_model("MT5700"),
            ChipsetVendor::TdTech
        );
    }

    #[test]
    fn unknown_model_returns_unknown() {
        assert_eq!(
            ModemFactory::detect_vendor_from_model("XYZ1234"),
            ChipsetVendor::Unknown
        );
    }
}
