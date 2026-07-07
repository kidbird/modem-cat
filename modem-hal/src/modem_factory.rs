use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::ChipsetVendor;
use crate::vendors::quectel::QuectelModem;

pub struct ModemFactory;

impl ModemFactory {
    fn usb_model_fallback_for_vendor(vendor: ChipsetVendor, pid: u16) -> &'static str {
        match vendor {
            ChipsetVendor::UniSoc => "RG200U/RM500U 5G",
            ChipsetVendor::Qualcomm => match pid {
                0x0800 => "Quectel Qualcomm 5G (PID 0800)",
                0x0801 => "Quectel Qualcomm 5G (PID 0801)",
                _ => "Quectel Qualcomm 5G",
            },
            // Keep RG255 in the fallback label so the shared Quectel adapter
            // continues to identify this branch as ASR for parsing/UI purposes.
            ChipsetVendor::Asr => "RG255 ASR 5G RedCap",
            ChipsetVendor::Unknown => "Unknown USB modem",
        }
    }

    pub fn create(transport: &mut dyn AtTransport) -> Result<Box<dyn ModemVendor>, String> {
        let model = Self::query_model(transport)?;
        let vendor = Self::detect_vendor_from_model(&model);
        log::info!("Detected model: '{}', vendor: {:?}", model, vendor);
        Self::create_from_vendor(model, vendor)
    }

    pub fn create_with_usb_ids(
        transport: &mut dyn AtTransport,
        usb_ids: Option<(u16, u16)>,
    ) -> Result<Box<dyn ModemVendor>, String> {
        if let Some((vid, pid)) = usb_ids {
            if let Some(vendor) = Self::detect_vendor_from_vid_pid(vid, pid) {
                let model = Self::detect_model_from_vid_pid(vid, pid)
                    .unwrap_or_else(|| Self::usb_model_fallback_for_vendor(vendor, pid));
                log::info!(
                    "Detected chipset branch from USB VID/PID {:04X}:{:04X}: '{}', vendor: {:?}",
                    vid,
                    pid,
                    model,
                    vendor
                );
                return Self::create_from_vendor(model.to_string(), vendor);
            }

            log::info!(
                "No known USB chipset mapping for VID/PID {:04X}:{:04X}; falling back to AT+CGMM",
                vid,
                pid
            );
        }

        Self::create(transport)
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
            ChipsetVendor::Asr => {
                // ASR 平台现阶段复用 UniSoc AT 指令集（同一 Quectel 厂家共通），
                // 后续若出现 ASR 独有 AT，再拆分独立 adapter。
                log::info!("Creating ASR adapter (reusing UniSoc AT set) for {}", model);
                Ok(Box::new(QuectelModem::unisoc(model)))
            }
            ChipsetVendor::Unknown => Err(format!(
                "Unknown modem vendor for '{}': refusing to guess an adapter",
                model
            )),
        }
    }

    pub fn detect_vendor_from_model(model: &str) -> ChipsetVendor {
        let upper = model.to_uppercase();
        let qualcomm = [
            "RG500Q", "RM500Q", "RG520N", "RM520N", "RG525F", "RG530F", "RM530F", "RM530N",
            "RM551E", "RM501Q", "RG540F", "RM540N",
        ];
        for m in &qualcomm {
            if upper.contains(m) {
                return ChipsetVendor::Qualcomm;
            }
        }
        let asr = ["RG255"];
        for m in &asr {
            if upper.contains(m) {
                return ChipsetVendor::Asr;
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

    pub fn detect_vendor_from_vid_pid(vid: u16, pid: u16) -> Option<ChipsetVendor> {
        match (vid, pid) {
            (0x2C7C, 0x0900) => Some(ChipsetVendor::UniSoc),
            (0x2C7C, 0x0800) | (0x2C7C, 0x0801) => Some(ChipsetVendor::Qualcomm),
            (0x2C7C, 0x0600) | (0x2C7C, 0x600C) => Some(ChipsetVendor::Asr),
            _ => None,
        }
    }

    pub fn detect_model_from_vid_pid(vid: u16, pid: u16) -> Option<&'static str> {
        match (vid, pid) {
            (0x2C7C, 0x0900) => Some("RG200U/RM500U 5G"),
            (0x2C7C, 0x0800) => Some("Quectel Qualcomm 5G (PID 0800)"),
            (0x2C7C, 0x0801) => Some("Quectel Qualcomm 5G (PID 0801)"),
            (0x2C7C, 0x0600) | (0x2C7C, 0x600C) => Some("RG255 ASR 5G RedCap"),
            _ => None,
        }
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
    use crate::transport::AtTransport;
    use crate::types::ChipsetVendor;

    struct MockTransport {
        cgmm_response: Option<String>,
        send_count: usize,
        sent: Vec<String>,
    }

    impl MockTransport {
        fn rejecting() -> Self {
            Self {
                cgmm_response: None,
                send_count: 0,
                sent: Vec::new(),
            }
        }

        fn with_cgmm_response(response: &str) -> Self {
            Self {
                cgmm_response: Some(response.to_string()),
                send_count: 0,
                sent: Vec::new(),
            }
        }
    }

    impl AtTransport for MockTransport {
        fn send_at(&mut self, command: &str) -> Result<String, String> {
            self.send_count += 1;
            self.sent.push(command.to_string());
            match command {
                "AT+CGMM" => self
                    .cgmm_response
                    .clone()
                    .ok_or_else(|| format!("Unexpected AT command: {}", command)),
                _ => Err(format!("Unexpected AT command: {}", command)),
            }
        }

        fn close(&mut self) {}
    }

    #[test]
    fn detects_qualcomm_from_model() {
        for model in &[
            "RM500Q-GL",
            "RM500Q-CN",
            "RG500Q-EA",
            "RM520N-GL",
            "RG520N-CN",
            "RM551E-GL",
            "RM530F-CN",
            "RM530N-EU",
            "RG530F-EU",
            "RG525F-CN",
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
    fn detects_asr_from_model() {
        assert_eq!(
            ModemFactory::detect_vendor_from_model("RG255AA"),
            ChipsetVendor::Asr
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_model("RG255AA-CN"),
            ChipsetVendor::Asr
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_model("rg255aa"),
            ChipsetVendor::Asr
        );
    }

    #[test]
    fn unknown_model_returns_unknown() {
        assert_eq!(
            ModemFactory::detect_vendor_from_model("XYZ1234"),
            ChipsetVendor::Unknown
        );
    }

    #[test]
    fn detects_model_from_usb_vid_pid() {
        assert_eq!(
            ModemFactory::detect_model_from_vid_pid(0x2C7C, 0x0900),
            Some("RG200U/RM500U 5G")
        );
        assert_eq!(
            ModemFactory::detect_model_from_vid_pid(0x2C7C, 0x0800),
            Some("Quectel Qualcomm 5G (PID 0800)")
        );
        assert_eq!(
            ModemFactory::detect_model_from_vid_pid(0x2C7C, 0x0600),
            Some("RG255 ASR 5G RedCap")
        );
        assert_eq!(
            ModemFactory::detect_model_from_vid_pid(0x2C7C, 0x9999),
            None
        );
    }

    #[test]
    fn detects_vendor_from_usb_vid_pid() {
        assert_eq!(
            ModemFactory::detect_vendor_from_vid_pid(0x2C7C, 0x0900),
            Some(ChipsetVendor::UniSoc)
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_vid_pid(0x2C7C, 0x0801),
            Some(ChipsetVendor::Qualcomm)
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_vid_pid(0x2C7C, 0x0600),
            Some(ChipsetVendor::Asr)
        );
        assert_eq!(
            ModemFactory::detect_vendor_from_vid_pid(0x2C7C, 0x9999),
            None
        );
    }

    #[test]
    fn create_with_usb_ids_skips_cgmm_for_known_mapping() {
        let mut transport = MockTransport::rejecting();
        let modem = ModemFactory::create_with_usb_ids(&mut transport, Some((0x2C7C, 0x0900)))
            .expect("known USB VID/PID should map to a modem");

        assert_eq!(transport.send_count, 0);
        assert_eq!(modem.vendor(), ChipsetVendor::UniSoc);
        assert_eq!(modem.model(), "RG200U/RM500U 5G");
    }

    #[test]
    fn create_with_usb_ids_uses_qualcomm_branch_for_known_qualcomm_pid() {
        let mut transport = MockTransport::rejecting();
        let modem = ModemFactory::create_with_usb_ids(&mut transport, Some((0x2C7C, 0x0800)))
            .expect("known Qualcomm USB VID/PID should map to a modem");

        assert_eq!(transport.send_count, 0);
        assert_eq!(modem.vendor(), ChipsetVendor::Qualcomm);
        assert_eq!(modem.model(), "Quectel Qualcomm 5G (PID 0800)");
    }

    #[test]
    fn create_with_usb_ids_uses_asr_branch_for_known_asr_pid() {
        let mut transport = MockTransport::rejecting();
        let modem = ModemFactory::create_with_usb_ids(&mut transport, Some((0x2C7C, 0x0600)))
            .expect("known ASR USB VID/PID should map to a modem");

        assert_eq!(transport.send_count, 0);
        assert_eq!(modem.vendor(), ChipsetVendor::Asr);
        assert_eq!(modem.model(), "RG255 ASR 5G RedCap");
    }

    #[test]
    fn create_with_usb_ids_falls_back_to_cgmm_for_unknown_mapping() {
        let mut transport = MockTransport::with_cgmm_response("AT+CGMM\r\nRM520N-GL\r\nOK\r\n");
        let modem = ModemFactory::create_with_usb_ids(&mut transport, Some((0x2C7C, 0x9999)))
            .expect("unknown USB VID/PID should fall back to CGMM");

        assert_eq!(transport.sent, vec!["AT+CGMM"]);
        assert_eq!(modem.vendor(), ChipsetVendor::Qualcomm);
        assert_eq!(modem.model(), "RM520N-GL");
    }

    #[test]
    fn create_with_usb_ids_without_enumerated_usb_ids_uses_cgmm_only() {
        let mut transport = MockTransport::with_cgmm_response("AT+CGMM\r\nRG255AA\r\nOK\r\n");
        let modem = ModemFactory::create_with_usb_ids(&mut transport, None)
            .expect("CGMM should remain the only fallback when USB IDs are unavailable");

        assert_eq!(transport.sent, vec!["AT+CGMM"]);
        assert_eq!(modem.vendor(), ChipsetVendor::Asr);
        assert_eq!(modem.model(), "RG255AA");
    }

    #[test]
    fn create_from_unknown_vendor_returns_error() {
        assert!(
            ModemFactory::create_from_vendor("XYZ1234".to_string(), ChipsetVendor::Unknown,)
                .is_err()
        );
    }
}
