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

/// Validate a complete raw AT command before sending it to the modem.
///
/// Unlike `validate_at_string`, this validates the whole command line, so
/// double quotes are allowed. Command terminators, control characters, and
/// command chaining are rejected to keep `send_raw_at` from becoming an
/// injection bypass.
pub fn validate_raw_at_command(command: &str) -> Result<(), String> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return Err("AT command cannot be empty".to_string());
    }
    if !trimmed.to_ascii_uppercase().starts_with("AT") {
        return Err("AT command must start with AT".to_string());
    }

    for (i, ch) in command.char_indices() {
        if ch == '\r' || ch == '\n' {
            return Err(format!(
                "Invalid character at position {}: line break not allowed in AT command",
                i
            ));
        }
        if ch == ';' {
            return Err(format!(
                "Invalid character at position {}: command chaining not allowed",
                i
            ));
        }
        if ch.is_control() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_at_command_allows_quoted_complete_commands() {
        assert!(validate_raw_at_command(r#"AT+QCFG="ims""#).is_ok());
        assert!(validate_raw_at_command(r#"AT+QNWLOCK="common/5g",1,630000,123"#).is_ok());
        assert!(validate_raw_at_command(r#"AT+QCFG="lanip_ex","192.168.8.1","192.168.8.2","192.168.8.254""#).is_ok());
    }

    #[test]
    fn raw_at_command_rejects_line_break_injection() {
        assert!(validate_raw_at_command("AT+CSQ\r\nAT+CFUN=1,1").is_err());
        assert!(validate_raw_at_command("AT+CSQ\nAT+CFUN=1,1").is_err());
    }

    #[test]
    fn raw_at_command_rejects_non_at_input() {
        assert!(validate_raw_at_command("").is_err());
        assert!(validate_raw_at_command("QCFG=\"ims\"").is_err());
    }

    #[test]
    fn at_parameter_rejects_quote_escape() {
        assert!(validate_at_string(r#"abc"def"#).is_err());
        assert!(validate_at_string("abc\r\nAT+CFUN=1,1").is_err());
        assert!(validate_at_string("cmnet").is_ok());
    }
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
