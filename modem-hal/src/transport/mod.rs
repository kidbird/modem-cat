#[cfg(feature = "serial")]
pub mod serial;
pub mod tcp;

#[cfg(feature = "serial")]
pub use serial::SerialTransport;
pub use tcp::TcpTransport;

pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
    /// Returns false when the underlying hardware has been removed (e.g. USB unplug).
    /// Does not send any bytes — safe to call from a background heartbeat.
    fn is_alive(&self) -> bool { true }
}

pub struct MockTransport {
    pub responses: std::collections::VecDeque<String>,
}

impl MockTransport {
    pub fn new(responses: Vec<&str>) -> Self {
        Self {
            responses: responses.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl AtTransport for MockTransport {
    fn send_at(&mut self, _command: &str) -> Result<String, String> {
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
    fn test_redact_at_command() {
        assert_eq!(
            redact_at_command(r#"AT+QSIMLOCK="PN","12345678",2,"46001""#),
            r#"AT+QSIMLOCK="PN","******",2,"46001""#
        );
        assert_eq!(
            redact_at_command(r#"AT+QSIMLOCK="PN","87654321""#),
            r#"AT+QSIMLOCK="PN","******""#
        );
        assert_eq!(
            redact_at_command("AT+CSQ"),
            "AT+CSQ"
        );
    }
}

pub fn redact_at_command(command: &str) -> String {
    if command.contains("QSIMLOCK") {
        if let Some(pos) = command.find(r#""PN""#) {
            let prefix = &command[..pos + 4];
            let rest = &command[pos + 4..];
            if let Some(first_quote) = rest.find('"') {
                if let Some(second_quote) = rest[first_quote + 1..].find('"') {
                    let actual_second_quote = first_quote + 1 + second_quote;
                    let suffix = &rest[actual_second_quote + 1..];
                    return format!("{},\"******\"{}", prefix, suffix);
                }
            }
        }
    }
    command.to_string()
}

