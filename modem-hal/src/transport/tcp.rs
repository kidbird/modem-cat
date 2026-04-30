use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use crate::transport::AtTransport;

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
