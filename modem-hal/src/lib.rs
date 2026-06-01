pub mod modem_factory;
pub mod modem_vendor;
pub mod transport;
pub mod types;
pub mod vendors;

pub use modem_factory::ModemFactory;
pub use modem_vendor::ModemVendor;
pub use types::*;

/// Validate a string parameter before embedding it in an AT command.
/// Rejects characters that could break out of quoted AT parameters or inject extra commands.
///
/// Dangerous characters:
/// - `\r`, `\n` — AT command terminators, enable command injection
/// - `"` — breaks out of quoted string parameters
/// - Other control characters (0x00-0x1F except common whitespace)
pub fn validate_at_string(s: &str) -> Result<(), String> {
    for (i, ch) in s.char_indices() {
        if ch == '\r' || ch == '\n' {
            return Err(format!(
                "Invalid character at position {}: line break not allowed in AT parameter",
                i
            ));
        }
        if ch == '"' {
            return Err(format!(
                "Invalid character at position {}: double quote not allowed in AT parameter",
                i
            ));
        }
        if ch.is_control() && ch != '\t' {
            return Err(format!(
                "Invalid character at position {}: control character not allowed",
                i
            ));
        }
    }
    Ok(())
}

/// Validate that a CID (PDP context identifier) is within the valid range (1-16).
pub fn validate_cid(cid: i32) -> Result<(), String> {
    if cid < 1 || cid > 16 {
        return Err(format!(
            "Invalid CID {}: must be between 1 and 16",
            cid
        ));
    }
    Ok(())
}

// ── napi-rs surface for Bun/TS ──
#[cfg(feature = "napi-feature")]
mod napi_exports {
    use crate::transport::SerialTransport;
    use crate::ModemFactory;
    use napi_derive::napi;

    #[napi]
    pub struct ModemHandle {
        inner: Box<dyn crate::ModemVendor + Send>,
        transport: SerialTransport,
    }

    #[napi]
    impl ModemHandle {
        #[napi(factory)]
        pub fn connect(port: String, baud: u32) -> napi::Result<Self> {
            let mut transport =
                SerialTransport::new(&port, baud).map_err(|e| napi::Error::from_reason(e))?;
            let modem =
                ModemFactory::create(&mut transport).map_err(|e| napi::Error::from_reason(e))?;
            Ok(Self {
                inner: modem,
                transport,
            })
        }

        #[napi]
        pub fn query_signal(&mut self) -> napi::Result<crate::types::SignalInfo> {
            self.inner
                .query_signal_strength(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn query_status(&mut self) -> napi::Result<crate::types::ModemStatus> {
            self.inner
                .query_modem_status(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn connect_data(&mut self, cid: i32) -> napi::Result<()> {
            self.inner
                .connect_data(&mut self.transport, cid)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn disconnect_data(&mut self, cid: i32) -> napi::Result<()> {
            self.inner
                .disconnect_data(&mut self.transport, cid)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn reboot(&mut self) -> napi::Result<()> {
            self.inner
                .reboot(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn close(&mut self) {
            use crate::transport::AtTransport;
            self.transport.close();
        }
    }
}
