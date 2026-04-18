use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use serialport::SerialPort;

/// Trait for AT command transport (Serial or TCP)
pub trait AtTransport: Send {
    fn send_at(&mut self, command: &str) -> Result<String, String>;
    fn close(&mut self);
}

/// Serial port transport
pub struct SerialTransport {
    port: Box<dyn SerialPort>,
}

impl SerialTransport {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(500))
            .open()
            .map_err(|e| format!("Failed to open {}: {}", port_name, e))?;
        Ok(Self { port })
    }

    /// Quick probe: send AT and check for OK within a short timeout.
    /// Used for port detection. Returns true if the port responded with OK.
    pub fn probe_at(port_name: &str, baud_rate: u32) -> bool {
        let port = match serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(200))
            .open()
        {
            Ok(p) => p,
            Err(_) => return false,
        };
        let mut transport = Self { port };

        // Send AT command
        if transport.port.write_all(b"AT\r\n").is_err() {
            return false;
        }
        let _ = transport.port.flush();

        // Wait briefly for modem to process
        std::thread::sleep(Duration::from_millis(200));

        // Read response with short timeout
        let mut buf = [0u8; 256];
        let start = std::time::Instant::now();
        let mut response = String::new();

        while start.elapsed() < Duration::from_millis(800) {
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
        let overall_timeout = Duration::from_secs(8);

        loop {
            if start.elapsed() > overall_timeout {
                log::warn!("read_response: overall timeout after {:?}", overall_timeout);
                break;
            }

            match self.port.read(&mut buf) {
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]);
                    response.push_str(&text);

                    let trimmed = response.trim();
                    if trimmed.ends_with("OK")
                        || trimmed.ends_with("ERROR")
                        || trimmed.contains("+CME ERROR")
                    {
                        // Got a complete response, do one more short read to catch any trailing data
                        let old_timeout = self.port.timeout();
                        let _ = self.port.set_timeout(Duration::from_millis(200));
                        while let Ok(n2) = self.port.read(&mut buf) {
                            if n2 == 0 { break; }
                            response.push_str(&String::from_utf8_lossy(&buf[..n2]));
                        }
                        let _ = self.port.set_timeout(old_timeout);
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if !response.is_empty() {
                        // We have data but got a timeout — check if it looks complete
                        let trimmed = response.trim();
                        if trimmed.ends_with("OK")
                            || trimmed.ends_with("ERROR")
                            || trimmed.contains("+CME ERROR")
                            || start.elapsed() > Duration::from_secs(2)
                        {
                            break;
                        }
                    }
                    // No data yet, keep waiting up to overall_timeout
                }
                Err(e) => return Err(format!("Read error: {}", e)),
            }
        }

        log::debug!("read_response: got {} bytes in {:?}", response.len(), start.elapsed());
        Ok(response.trim().to_string())
    }
}

/// TCP transport
pub struct TcpTransport {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
}

impl TcpTransport {
    pub fn new(host: &str, port: u16) -> Result<Self, String> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect_timeout(
            &addr.parse().map_err(|e| format!("Invalid address: {}", e))?,
            Duration::from_secs(5),
        )
        .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

        stream
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();
        stream
            .set_write_timeout(Some(Duration::from_secs(3)))
            .ok();

        let reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        Ok(Self {
            reader,
            writer: stream,
        })
    }

    fn read_response(&mut self) -> Result<String, String> {
        let mut response = String::new();
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(5);

        loop {
            if start.elapsed() > timeout {
                break;
            }

            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    response.push_str(trimmed);
                    response.push('\n');

                    if trimmed == "OK"
                        || trimmed.starts_with("ERROR")
                        || trimmed.starts_with("+CME ERROR")
                    {
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if !response.is_empty() {
                        break;
                    }
                }
                Err(e) => return Err(format!("Read error: {}", e)),
            }
        }

        Ok(response.trim().to_string())
    }
}

impl AtTransport for SerialTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        // Quick drain of any stale data
        let mut drain = [0u8; 4096];
        let _ = self.port.set_timeout(Duration::from_millis(50));
        loop {
            match self.port.read(&mut drain) {
                Ok(0) | Err(_) => break,
                Ok(_) => continue,
            }
        }
        // Restore normal timeout for command response
        let _ = self.port.set_timeout(Duration::from_secs(3));

        log::debug!("send_at: >>> {}", command);

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
}

impl AtTransport for TcpTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        self.writer
            .write_all(format!("{}\r\n", command).as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        let _ = self.writer.flush();

        self.read_response()
    }

    fn close(&mut self) {
        let _ = self.writer.shutdown(std::net::Shutdown::Both);
    }
}
