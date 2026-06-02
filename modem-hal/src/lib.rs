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
        if ch == '&' {
            // Hayes "AT&F" (factory reset) / "AT&W" (write profile) / "AT&V" (view)
            // can wipe modem NV. send_raw_at is a debugging affordance, not a
            // S-register / Hayes configuration path — refuse it.
            return Err(format!(
                "Invalid character at position {}: Hayes '&' commands not allowed in send_raw_at (use the dedicated UI)",
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
    // Reject S-register writes like "ATS0=0" / "ATS13=1" (auto-answer,
    // echo-off, etc.). Hayes allows multi-digit register numbers, so we
    // scan past all leading digits before checking for '='.
    let after_at = trimmed.get(2..).unwrap_or("");
    if !after_at.is_empty() && after_at.as_bytes()[0].eq_ignore_ascii_case(&b'S') {
        let bytes = after_at.as_bytes();
        let mut i = 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        // i is the first non-digit; if it's '=' we have ATSn=… (a write).
        // Reads like ATS0? / ATS5 (no '=') stay allowed.
        if i > 1 && i < bytes.len() && bytes[i] == b'=' {
            return Err(
                "S-register writes (e.g. ATS0=0, ATS13=1) are not allowed in send_raw_at"
                    .to_string(),
            );
        }
    }
    Ok(())
}

/// Validate that a CID (PDP context identifier) is within the valid range (1-16).
pub fn validate_cid(cid: i32) -> Result<(), String> {
    if cid < 1 || cid > 16 {
        return Err(format!("Invalid CID {}: must be between 1 and 16", cid));
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
        assert!(validate_raw_at_command(
            r#"AT+QCFG="lanip_ex","192.168.8.1","192.168.8.2","192.168.8.254""#
        )
        .is_ok());
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
    fn raw_at_command_rejects_hayes_amp_and_s_register() {
        // Hayes & commands (factory reset, profile write, etc.)
        assert!(validate_raw_at_command("AT&F").is_err());
        assert!(validate_raw_at_command("AT&F0").is_err());
        assert!(validate_raw_at_command("AT&W").is_err());
        // S-register writes (auto-answer, echo, etc.)
        assert!(validate_raw_at_command("ATS0=0").is_err());
        assert!(validate_raw_at_command("ATS3=13").is_err());
        assert!(validate_raw_at_command("ats13=1").is_err()); // case-insensitive

        // S-registers READS are still allowed (no '=')
        assert!(validate_raw_at_command("ATS0?").is_ok());
        assert!(validate_raw_at_command("ATS5").is_ok());
    }

    #[test]
    fn at_parameter_rejects_quote_escape() {
        assert!(validate_at_string(r#"abc"def"#).is_err());
        assert!(validate_at_string("abc\r\nAT+CFUN=1,1").is_err());
        assert!(validate_at_string("cmnet").is_ok());
    }
}
// ── (history) napi-rs surface for Bun/TS removed 2026-06-02 ──
//   The `napi-feature` Cargo feature was declared but never enabled, so the
//   `ModemHandle` napi bindings were dead code at every commit. Deleted the
//   module + the feature + the `napi` / `napi-derive` / `napi-build` deps.
//   Re-enable from git history if a Node/Bun consumer actually materialises.
