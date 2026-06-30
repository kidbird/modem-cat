#[cfg(feature = "serial")]
pub mod serial;
pub mod tcp;
pub mod websocket;

#[cfg(feature = "serial")]
pub use serial::SerialTransport;
pub use tcp::TcpTransport;
pub use websocket::WebSocketTransport;

pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
    /// Returns false when the underlying hardware has been removed (e.g. USB unplug).
    /// Does not send any bytes — safe to call from a background heartbeat.
    fn is_alive(&self) -> bool {
        true
    }
    /// Non-blocking graceful shutdown for disconnect paths.
    ///
    /// Default implementation delegates to `close()`. Transports whose `close()`
    /// may block (e.g. WebSocket waiting for server close frame) should override
    /// this to perform a best-effort, non-blocking teardown. The caller will
    /// `drop()` the transport immediately after this returns, so any lingering
    /// resources will be reclaimed by the OS.
    fn force_shutdown(&mut self) {
        self.close();
    }
}

/// Check whether an AT response is terminally complete.
///
/// All valid AT responses end with one of: `OK`, `ERROR`, `+CME ERROR`,
/// `+CMS ERROR`. A response without any of these terminators is either still
/// streaming or was truncated by a timeout — parsers must treat such responses
/// as unreliable (AGENTS.md: "实时状态禁止 fallback / 禁止伪成功").
///
/// Used by `read_response` in serial.rs / tcp.rs / websocket.rs to decide
/// whether a timeout-terminated read should return `Ok(response)` or
/// `Err("incomplete")`.
#[inline]
pub fn is_complete_response(response: &str) -> bool {
    let t = response.trim();
    t.ends_with("OK")
        || t.ends_with("ERROR")
        || t.contains("+CME ERROR")
        || t.contains("+CMS ERROR")
}

pub struct MockTransport {
    pub responses: std::collections::VecDeque<String>,
    /// Records every command passed to `send_at`, in order — lets tests assert the exact AT string.
    pub sent: Vec<String>,
}

impl MockTransport {
    pub fn new(responses: Vec<&str>) -> Self {
        Self {
            responses: responses.iter().map(|s| s.to_string()).collect(),
            sent: Vec::new(),
        }
    }
}

impl AtTransport for MockTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        self.sent.push(command.to_string());
        self.responses
            .pop_front()
            .ok_or("no more responses".to_string())
    }
    fn close(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_transport_returns_responses_in_order() {
        let mut t = MockTransport::new(vec!["OK", "ERROR"]);
        assert_eq!(t.send_at("AT").unwrap(), "OK");
        assert_eq!(t.send_at("AT+FAIL").unwrap(), "ERROR");
        assert!(t.send_at("AT").is_err());
    }

    #[test]
    fn is_complete_response_accepts_terminators() {
        assert!(is_complete_response("OK"));
        assert!(is_complete_response("+CPIN: READY\r\nOK"));
        assert!(is_complete_response("ERROR"));
        assert!(is_complete_response("+CME ERROR: 10"));
        assert!(is_complete_response("+CMS ERROR: 304"));
        assert!(is_complete_response("+COPS: 0,2,\"46001\"\r\nOK"));
    }

    #[test]
    fn is_complete_response_rejects_truncated() {
        assert!(!is_complete_response(""));
        assert!(!is_complete_response("+CPIN: READY"));           // missing OK
        assert!(!is_complete_response("+QENG: \"servingcell\""));  // partial
        assert!(!is_complete_response("some intermediate data"));
        assert!(!is_complete_response("+COPS: 0,"));               // cut mid-line
    }

    #[test]
    fn test_redact_at_command_qsimlock() {
        assert_eq!(
            redact_at_command(r#"AT+QSIMLOCK="PN","12345678",2,"46001""#),
            r#"AT+QSIMLOCK="PN","******",2,"46001""#
        );
        assert_eq!(
            redact_at_command(r#"AT+QSIMLOCK="PN","87654321""#),
            r#"AT+QSIMLOCK="PN","******""#
        );
    }

    #[test]
    fn test_redact_at_command_cgauth_password() {
        // CGAUTH: AT+CGAUTH=<cid>,<auth>,"<user>","<password>"
        // Password is the LAST quoted value (positional arg). Generic
        // key-based matcher can't see it, so the command is in the
        // positional-override list.
        assert_eq!(
            redact_at_command(r#"AT+CGAUTH=1,1,"user","s3cret""#),
            r#"AT+CGAUTH=1,1,"user","******""#
        );
    }

    #[test]
    fn test_redact_at_command_cgdcnt_password() {
        // CGDCONT: AT+CGDCONT=<cid>,"<type>","<apn>","<user>","<password>"
        // Password is the LAST quoted value.
        assert_eq!(
            redact_at_command(r#"AT+CGDCONT=1,"IP","cmnet","u","hunter2""#),
            r#"AT+CGDCONT=1,"IP","cmnet","u","******""#
        );
    }

    #[test]
    fn test_redact_at_command_no_sensitive_key() {
        // Unrelated AT commands must NOT be modified.
        assert_eq!(redact_at_command("AT+CSQ"), "AT+CSQ");
        assert_eq!(redact_at_command("AT+CGMI"), "AT+CGMI");
    }

    #[test]
    fn test_redact_at_command_does_not_match_substring() {
        // "auth" inside another word (e.g. "author") must NOT trigger redaction.
        // The matcher uses left-side word boundary.
        assert_eq!(
            redact_at_command(r#"AT+FOO="author","actual""#),
            r#"AT+FOO="author","actual""#
        );
    }

    #[test]
    fn test_redact_at_command_keyed_password() {
        // Generic key-based matcher handles this case.
        // AT+FOO=arg1,password="secret",arg2=...
        assert_eq!(
            redact_at_command(r#"AT+FOO=1,password="topsecret",2"#),
            r#"AT+FOO=1,password="******",2"#
        );
    }
}

/// AT command names whose quoted value is a credential. The first quoted
/// string following any of these names is replaced with `"******"` before
/// logging. Case-insensitive on the name; the value itself is opaque.
///
/// Covers the common leak vectors we actually use:
/// - `password` / `passwd` / `pwd` — APN, VPN, HTTP auth
/// - `token` — QSSLCFG / QHTTPURL bearer tokens
/// - `auth` — generic auth field
/// - `secret` — generic
const SENSITIVE_KEYS: &[&str] = &["password", "passwd", "pwd", "token", "auth", "secret"];

/// AT commands whose LAST quoted value is a credential, used positionally
/// (no `key="value"` form). The generic key-matcher cannot see these.
///
/// Driven by the commands the codebase actually issues.
const POSITIONAL_PASSWORD_CMDS: &[&str] = &["CGAUTH", "CGDCONT"];

/// Replace credentials in an AT command with `******` before logging.
///
/// Three layers of defense:
/// 1. QSIMLOCK uses an unusual `"PN","<password>"` quoting style.
/// 2. AT commands in `POSITIONAL_PASSWORD_CMDS` have their LAST quoted
///    value redacted (e.g. `CGAUTH=<cid>,"user","password"`).
/// 3. Every other command goes through the generic `,key="value"` scan
///    against `SENSITIVE_KEYS`.
pub fn redact_at_command(command: &str) -> String {
    if command.contains("QSIMLOCK") {
        if let Some(r) = redact_qsimlock_password(command) {
            return r;
        }
    }
    if let Some(r) = redact_last_quoted_for_known_cmds(command, POSITIONAL_PASSWORD_CMDS) {
        return r;
    }
    redact_quoted_value_after_key(command, SENSITIVE_KEYS)
}

/// For each prefix in `cmds`, if the command starts with that prefix,
/// redact its LAST quoted value. Returns the first match.
fn redact_last_quoted_for_known_cmds(command: &str, cmds: &[&str]) -> Option<String> {
    let upper = command.to_ascii_uppercase();
    for cmd in cmds {
        // Match `AT+CMD` (or `CMD` at start of string)
        if upper.starts_with(&format!("AT+{}", cmd)) || upper.starts_with(cmd) {
            if let Some(r) = redact_last_quoted_value(command) {
                return Some(r);
            }
        }
    }
    None
}

/// Replace the LAST `"..."` value in the string with `"******"`.
/// Preserves all bytes outside the redacted segment.
fn redact_last_quoted_value(command: &str) -> Option<String> {
    let bytes = command.as_bytes();
    // Find the last `"` that is followed by a closing `"` later in the string.
    let mut last_open: Option<(usize, usize)> = None;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            // Find the matching close (assume no escaped quotes in AT params).
            if let Some(close_rel) = command[i + 1..].find('"') {
                let close = i + 1 + close_rel;
                if close > i + 1 {
                    last_open = Some((i, close));
                }
                i = close + 1;
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }
    last_open
        .map(|(open, close)| format!("{}\"******\"{}", &command[..open], &command[close + 1..]))
}

/// QSIMLOCK: keep `"PN"` literal, replace the next quoted value.
fn redact_qsimlock_password(command: &str) -> Option<String> {
    let pos = command.find(r#""PN""#)?;
    let prefix = &command[..pos + 4]; // includes `"PN"`
    let rest = &command[pos + 4..];
    let first_quote = rest.find('"')?;
    let second_quote_rel = rest[first_quote + 1..].find('"')?;
    let actual_second_quote = first_quote + 1 + second_quote_rel;
    let suffix = &rest[actual_second_quote + 1..];
    Some(format!("{},\"******\"{}", prefix, suffix))
}

/// Walk the command string. For any occurrence of `<key><sep>"<value>"` where
/// `key` is in `SENSITIVE_KEYS` (case-insensitive, word-bounded on the left
/// by start / `,` / `=` / whitespace), replace `<value>` with `******`.
///
/// Conservative: if the next token after `key` is not `,` / `=` + optional
/// whitespace + `"`, leave it alone.
fn redact_quoted_value_after_key(command: &str, keys: &[&str]) -> String {
    let lower = command.to_ascii_lowercase();
    let mut out = String::with_capacity(command.len());
    let mut i = 0;
    while i < command.len() {
        let matched_key = keys.iter().find(|k| {
            let kb = k.as_bytes();
            let end = i + kb.len();
            if end > command.len() {
                return false;
            }
            if lower.as_bytes()[i..end] != *kb {
                return false;
            }
            // Word boundary on the left: start-of-string, or non-alphanumeric
            // char immediately before. (Also accept whitespace + ,/= as
            // "boundary-like" so we don't break on cosmetic spaces.)
            if i > 0 {
                let prev = command.as_bytes()[i - 1];
                if prev.is_ascii_alphanumeric() || prev == b'_' {
                    return false;
                }
            }
            true
        });
        if let Some(k) = matched_key {
            out.push_str(&command[i..i + k.len()]);
            let mut j = i + k.len();
            // Skip whitespace
            while j < command.len() && matches!(command.as_bytes()[j], b' ' | b'\t') {
                out.push(command.as_bytes()[j] as char);
                j += 1;
            }
            // Expect , or =
            if j < command.len() && matches!(command.as_bytes()[j], b',' | b'=') {
                out.push(command.as_bytes()[j] as char);
                j += 1;
                // Skip whitespace
                while j < command.len() && matches!(command.as_bytes()[j], b' ' | b'\t') {
                    out.push(command.as_bytes()[j] as char);
                    j += 1;
                }
                // Expect opening "
                if j < command.len() && command.as_bytes()[j] == b'"' {
                    out.push('"');
                    j += 1;
                    let val_start = j;
                    while j < command.len() && command.as_bytes()[j] != b'"' {
                        j += 1;
                    }
                    if j < command.len() {
                        if j > val_start {
                            out.push_str("******");
                        }
                        out.push('"');
                        j += 1;
                    }
                }
            }
            i = j;
        } else {
            // Push one Unicode char (handle multi-byte correctly)
            let next = command[i..].chars().next().unwrap();
            out.push(next);
            i += next.len_utf8();
        }
    }
    out
}
