use crate::transport::is_complete_response;
use crate::transport::AtTransport;
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

/// Serial port transport
pub struct SerialTransport {
    port: Box<dyn SerialPort>,
}

// Timeout constants (all in one place)
const OPEN_TIMEOUT: Duration = Duration::from_millis(500);
const PROBE_TIMEOUT: Duration = Duration::from_millis(200);
const PROBE_READ_DEADLINE: Duration = Duration::from_millis(800);
const DRAIN_TIMEOUT: Duration = Duration::from_millis(1);
const READ_TIMEOUT: Duration = Duration::from_secs(3);
const RESPONSE_OVERALL: Duration = Duration::from_secs(8);
const RESPONSE_DATA_TIMEOUT: Duration = Duration::from_secs(2);
const TRAILING_DRAIN_TIMEOUT: Duration = Duration::from_millis(5);

impl SerialTransport {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let mut port = serialport::new(port_name, baud_rate)
            .timeout(OPEN_TIMEOUT)
            .open()
            .map_err(|e| format!("Failed to open {}: {}", port_name, e))?;
        let _ = port.write_data_terminal_ready(true);
        let _ = port.write_request_to_send(true);
        Ok(Self { port })
    }

    /// Quick probe: send AT and check for OK within a short timeout.
    /// Used for port detection. Returns true if the port responded with OK.
    pub fn probe_at(port_name: &str, baud_rate: u32) -> bool {
        let mut port = match serialport::new(port_name, baud_rate)
            .timeout(PROBE_TIMEOUT)
            .open()
        {
            Ok(p) => p,
            Err(_) => return false,
        };
        let _ = port.write_data_terminal_ready(true);
        let _ = port.write_request_to_send(true);
        let mut transport = Self { port };

        // Send AT command
        if transport.port.write_all(b"AT\r\n").is_err() {
            return false;
        }
        let _ = transport.port.flush();

        // Read response with short timeout
        let mut buf = [0u8; 256];
        let start = std::time::Instant::now();
        let mut response = String::new();

        while start.elapsed() < PROBE_READ_DEADLINE {
            match transport.port.read(&mut buf) {
                Ok(n) => {
                    response.push_str(&String::from_utf8_lossy(&buf[..n]));
                    if response.trim().ends_with("OK") {
                        return true;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if !response.is_empty() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        response.trim().ends_with("OK")
    }

    fn read_response(&mut self) -> Result<String, String> {
        let mut response = String::new();
        let mut buf = [0u8; 2048];
        let start = std::time::Instant::now();
        let overall_timeout = RESPONSE_OVERALL;

        loop {
            if start.elapsed() > overall_timeout {
                // Overall timeout reached. Only return Ok if the response is
                // terminally complete; otherwise surface as an error so parsers
                // don't misinterpret truncated data as valid state.
                let trimmed = response.trim();
                if !trimmed.is_empty() && is_complete_response(trimmed) {
                    log::debug!(
                        "read_response: got {} bytes (complete, edge of overall timeout)",
                        response.len()
                    );
                    return Ok(trimmed.to_string());
                }
                log::warn!("read_response: overall timeout after {:?} (response incomplete, {} bytes discarded)", overall_timeout, response.len());
                return Err("Read response timeout (incomplete)".to_string());
            }

            match self.port.read(&mut buf) {
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]);
                    response.push_str(&text);

                    let trimmed = response.trim();
                    if is_complete_response(trimmed) {
                        // Got a complete response, do one more short read to catch any trailing data
                        let old_timeout = self.port.timeout();
                        let _ = self.port.set_timeout(TRAILING_DRAIN_TIMEOUT);
                        while let Ok(n2) = self.port.read(&mut buf) {
                            if n2 == 0 {
                                break;
                            }
                            response.push_str(&String::from_utf8_lossy(&buf[..n2]));
                        }
                        let _ = self.port.set_timeout(old_timeout);
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if !response.is_empty() && is_complete_response(response.trim()) {
                        // Data complete before read timeout — safe to return.
                        break;
                    }
                    if response.is_empty() {
                        // No data yet, keep waiting up to overall_timeout
                        continue;
                    }
                    // Partial data + per-read timeout. Two cases:
                    //   - elapsed > RESPONSE_DATA_TIMEOUT → upper layer was slow;
                    //     if response is complete return it, otherwise keep waiting.
                    //   - otherwise → continue waiting for more data.
                    if start.elapsed() > RESPONSE_DATA_TIMEOUT
                        && is_complete_response(response.trim())
                    {
                        break;
                    }
                    // Otherwise keep looping; the overall_timeout check above will
                    // return Err if we never reach completeness.
                }
                Err(e) => return Err(format!("Read error: {}", e)),
            }
        }

        log::debug!(
            "read_response: got {} bytes in {:?}",
            response.len(),
            start.elapsed()
        );
        Ok(response.trim().to_string())
    }
}

impl AtTransport for SerialTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        // Quick drain of any stale data (1ms timeout to avoid wasting time).
        // Capped at MAX_DRAIN_READS iterations: a misbehaving driver that keeps
        // returning >0 bytes under DRAIN_TIMEOUT (1ms) would otherwise stall
        // the AT path indefinitely. 64 × 4096 B = 256 KB is far more than any
        // realistic stale-buffer size from a 5G modem's unsolicited URCs.
        const MAX_DRAIN_READS: usize = 64;
        let mut drain = [0u8; 4096];
        let _ = self.port.set_timeout(DRAIN_TIMEOUT);
        for _ in 0..MAX_DRAIN_READS {
            match self.port.read(&mut drain) {
                Ok(0) | Err(_) => break,
                Ok(_) => continue,
            }
        }
        // Restore normal timeout for command response
        let _ = self.port.set_timeout(READ_TIMEOUT);

        log::debug!("send_at: >>> {}", super::redact_at_command(command));

        self.port
            .write_all(format!("{}\r\n", command).as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        let _ = self.port.flush();

        let result = self.read_response();

        if let Ok(ref resp) = result {
            log::debug!("send_at: <<< {}", resp);
        }
        result
    }

    fn close(&mut self) {
        // Serial port is closed automatically when Box<dyn SerialPort> is dropped.
    }

    fn is_alive(&self) -> bool {
        // bytes_to_read() is a lightweight ioctl that fails immediately if the
        // underlying USB device has been physically removed on Windows.
        if !self.port.bytes_to_read().is_ok() {
            return false;
        }

        // On Linux, ioctl FIONREAD might still succeed (returning Ok(0)) for up to
        // 20s after a USB-serial adapter is unplugged due to driver-level caching.
        // Verifying that the serial device file still exists on the filesystem
        // provides near-instantaneous disconnection detection.
        #[cfg(target_os = "linux")]
        {
            if let Some(name) = self.port.name() {
                if !std::path::Path::new(&name).exists() {
                    return false;
                }
            }
        }

        true
    }
}
