use crate::transport::AtTransport;
use crate::types::ChipsetVendor;

/// Detects the modem chipset vendor by sending identification commands
pub struct VendorDetector;

impl VendorDetector {
    /// Detect vendor by querying AT+CGMM (model) and analyzing the response
    /// 
    /// UniSoc models: RG200U, RM500U, RG500U
    /// Qualcomm models: RG520N, RM520N, RG525F, RG530F, RM530N
    pub fn detect(transport: &mut dyn AtTransport) -> Result<ChipsetVendor, String> {
        // Try AT+CGMM first
        let cgmm_resp = match transport.send_at("AT+CGMM") {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Failed to query model: {}", e)),
        };
        
        // Extract model from response
        let model = Self::extract_model(&cgmm_resp);
        log::info!("Detected modem model: {}", model);
        
        // Check for UniSoc models
        let unisoc_models = [
            "RG200U", "RG500U", "RM500U", "RG501U", "RM501U",
        ];
        for &m in &unisoc_models {
            if model.to_uppercase().contains(m) {
                log::info!("Detected UniSoc chipset for model: {}", model);
                return Ok(ChipsetVendor::UniSoc);
            }
        }
        
        // Check for Qualcomm models
        let qualcomm_models = [
            "RG520N", "RM520N", "RG525F", "RG530F", "RM530N",
            "RG540F", "RM540N",
        ];
        for &m in &qualcomm_models {
            if model.to_uppercase().contains(m) {
                log::info!("Detected Qualcomm chipset for model: {}", model);
                return Ok(ChipsetVendor::Qualcomm);
            }
        }
        
        // Fallback: try to detect by checking specific commands
        log::warn!("Model '{}' not in known list, trying command-based detection", model);
        Self::detect_by_commands(transport)
    }
    
    /// Detect vendor by testing chipset-specific commands
    fn detect_by_commands(transport: &mut dyn AtTransport) -> Result<ChipsetVendor, String> {
        // UniSoc specific: AT+QBASELINE exists
        match transport.send_at("AT+QBASELINE?") {
            Ok(resp) if !resp.contains("ERROR") && !resp.contains("+CME ERROR") => {
                log::info!("Detected UniSoc chipset via AT+QBASELINE command");
                return Ok(ChipsetVendor::UniSoc);
            }
            _ => {}
        }
        
        // Qualcomm specific: AT+QRSRP exists
        match transport.send_at("AT+QRSRP=?") {
            Ok(resp) if !resp.contains("ERROR") && !resp.contains("+CME ERROR") => {
                log::info!("Detected Qualcomm chipset via AT+QRSRP command");
                return Ok(ChipsetVendor::Qualcomm);
            }
            _ => {}
        }
        
        // Default fallback: try AT+QTEMP (UniSoc specific)
        match transport.send_at("AT+QTEMP?") {
            Ok(resp) if !resp.contains("ERROR") && !resp.contains("+CME ERROR") => {
                log::info!("Detected UniSoc chipset via AT+QTEMP command");
                return Ok(ChipsetVendor::UniSoc);
            }
            _ => {}
        }
        
        log::warn!("Could not detect chipset vendor, returning Unknown");
        Ok(ChipsetVendor::Unknown)
    }
    
    /// Extract model name from AT+CGMM response
    fn extract_model(response: &str) -> String {
        for line in response.lines() {
            let trimmed = line.trim();
            // Skip echo and OK/ERROR lines
            if trimmed.starts_with("AT+") 
                || trimmed == "OK" 
                || trimmed.starts_with("ERROR")
                || trimmed.starts_with("+CME ERROR") {
                continue;
            }
            // Skip lines that look like +CGMM: prefix
            if let Some(rest) = trimmed.strip_prefix("+CGMM:") {
                return rest.trim().to_string();
            }
            // Return first non-empty, non-echo, non-OK line
            if !trimmed.is_empty() && !trimmed.starts_with('+') {
                return trimmed.to_string();
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_model() {
        let resp = "+CGMM: RG200U-CN\r\nOK";
        assert_eq!(VendorDetector::extract_model(resp), "RG200U-CN");
        
        let resp2 = "RG520N-GL\r\nOK";
        assert_eq!(VendorDetector::extract_model(resp2), "RG520N-GL");
    }
    
    #[test]
    fn test_model_detection() {
        assert!(VendorDetector::extract_model("+CGMM: RG200U-CN").contains("RG200U"));
        assert!(VendorDetector::extract_model("+CGMM: RM520N-GL").contains("RM520N"));
    }
}
