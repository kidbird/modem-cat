pub mod tcp;
#[cfg(feature = "serial")]
pub mod serial;

pub use tcp::TcpTransport;
#[cfg(feature = "serial")]
pub use serial::SerialTransport;

pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
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
        self.responses.pop_front().ok_or("no more responses".to_string())
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
}
