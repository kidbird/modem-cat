use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::ChipsetVendor;
use crate::vendor_detector::VendorDetector;
use crate::vendors::qualcomm::QualcommModem;
use crate::vendors::unisoc::UniSocModem;

/// Factory for creating modem vendor adapters
pub struct ModemFactory;

impl ModemFactory {
    /// Create a modem adapter by auto-detecting the chipset vendor
    pub fn create(transport: &mut dyn AtTransport) -> Result<Box<dyn ModemVendor>, String> {
        let vendor = VendorDetector::detect(transport)?;
        Self::create_from_vendor(transport, vendor)
    }

    /// Create a modem adapter for a specific vendor
    pub fn create_from_vendor(
        transport: &mut dyn AtTransport,
        vendor: ChipsetVendor,
    ) -> Result<Box<dyn ModemVendor>, String> {
        let model = Self::query_model(transport)?;
        match vendor {
            ChipsetVendor::UniSoc => {
                log::info!("Creating UniSoc modem adapter for {}", model);
                Ok(Box::new(UniSocModem::new(model)))
            }
            ChipsetVendor::Qualcomm => {
                log::info!("Creating Qualcomm modem adapter for {}", model);
                Ok(Box::new(QualcommModem::new(model)))
            }
            ChipsetVendor::Unknown => {
                log::warn!("Unknown chipset vendor, defaulting to UniSoc adapter");
                Ok(Box::new(UniSocModem::new(model)))
            }
        }
    }

    /// Create a modem adapter by explicitly specifying model name
    pub fn create_from_model(model: &str) -> Result<Box<dyn ModemVendor>, String> {
        let vendor = Self::detect_vendor_from_model(model);
        match vendor {
            ChipsetVendor::UniSoc => Ok(Box::new(UniSocModem::new(model.to_string()))),
            ChipsetVendor::Qualcomm => Ok(Box::new(QualcommModem::new(model.to_string()))),
            ChipsetVendor::Unknown => {
                log::warn!("Unknown model '{}', defaulting to UniSoc adapter", model);
                Ok(Box::new(UniSocModem::new(model.to_string())))
            }
        }
    }

    /// Detect vendor from model name string
    pub fn detect_vendor_from_model(model: &str) -> ChipsetVendor {
        let model_upper = model.to_uppercase();
        let unisoc_models = [
            "RG200U", "RG500U", "RM500U", "RG501U", "RM501U",
        ];
        for m in &unisoc_models {
            if model_upper.contains(m) {
                return ChipsetVendor::UniSoc;
            }
        }
        let qualcomm_models = [
            "RG520N", "RM520N", "RG525F", "RG530F", "RM530N",
            "RG540F", "RM540N",
        ];
        for m in &qualcomm_models {
            if model_upper.contains(m) {
                return ChipsetVendor::Qualcomm;
            }
        }
        ChipsetVendor::Unknown
    }

    fn query_model(transport: &mut dyn AtTransport) -> Result<String, String> {
        let resp = transport.send_at("AT+CGMM")?;
        for line in resp.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("AT+") || trimmed == "OK" || trimmed.starts_with("ERROR") {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("+CGMM:") {
                return Ok(rest.trim().to_string());
            }
            if !trimmed.is_empty() && !trimmed.starts_with('+') {
                return Ok(trimmed.to_string());
            }
        }
        Ok(String::new())
    }
}
