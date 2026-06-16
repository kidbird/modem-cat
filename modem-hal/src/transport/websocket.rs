use crate::transport::AtTransport;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::{client, Message, WebSocket};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::header::AUTHORIZATION;

/// Custom Base64 encoder to avoid versioning or import issues with the base64 crate.
fn base64_encode(input: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        match chunk.len() {
            3 => {
                let b = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
                result.push(CHARSET[((b >> 18) & 0x3F) as usize] as char);
                result.push(CHARSET[((b >> 12) & 0x3F) as usize] as char);
                result.push(CHARSET[((b >> 6) & 0x3F) as usize] as char);
                result.push(CHARSET[(b & 0x3F) as usize] as char);
            }
            2 => {
                let b = ((chunk[0] as u32) << 8) | (chunk[1] as u32);
                result.push(CHARSET[((b >> 10) & 0x3F) as usize] as char);
                result.push(CHARSET[((b >> 4) & 0x3F) as usize] as char);
                result.push(CHARSET[((b << 2) & 0x3F) as usize] as char);
                result.push('=');
            }
            1 => {
                let b = chunk[0] as u32;
                result.push(CHARSET[((b >> 2) & 0x3F) as usize] as char);
                result.push(CHARSET[((b << 4) & 0x3F) as usize] as char);
                result.push('=');
                result.push('=');
            }
            _ => unreachable!(),
        }
    }
    result
}

pub struct WebSocketTransport {
    socket: WebSocket<TcpStream>,
}

impl WebSocketTransport {
    pub fn new(
        host: &str,
        port: u16,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self, String> {
        let url = format!("ws://{}:{}", host, port);

        // 1. Create client request
        let mut request = url
            .as_str()
            .into_client_request()
            .map_err(|e| format!("Invalid WebSocket URL: {}", e))?;

        // 2. Add HTTP Basic Authentication if credentials are provided
        if let (Some(u), Some(p)) = (username, password) {
            let auth = format!("{}:{}", u, p);
            let auth_base64 = base64_encode(auth.as_bytes());
            request.headers_mut().insert(
                AUTHORIZATION,
                format!("Basic {}", auth_base64).parse().unwrap(),
            );
        }

        // 3. Connect TCP Stream with a 5-second timeout
        let addr = format!("{}:{}", host, port);
        let tcp_stream = TcpStream::connect_timeout(
            &addr
                .parse()
                .map_err(|e| format!("Invalid address {}: {}", addr, e))?,
            Duration::from_secs(5),
        )
        .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

        // Set temporary short read/write timeouts for handshake
        tcp_stream.set_read_timeout(Some(Duration::from_millis(500))).ok();
        tcp_stream.set_write_timeout(Some(Duration::from_secs(3))).ok();

        // 4. Perform WebSocket client handshake
        let (mut socket, _response) = client(request, tcp_stream)
            .map_err(|e| format!("WebSocket handshake failed: {}", e))?;

        // 5. Interactive terminal/console authentication fallback
        // If the server doesn't use Basic Auth but presents a login prompt over the WebSocket stream,
        // we wait briefly and respond if requested.
        let _ = socket.get_ref().set_read_timeout(Some(Duration::from_millis(200)));
        if let Ok(msg) = socket.read() {
            if let Message::Text(ref text) = msg {
                let lower = text.to_lowercase();
                if lower.contains("login") || lower.contains("username") || lower.contains("user:") {
                    // Send username
                    let user_to_send = username.unwrap_or("admin");
                    socket
                        .send(Message::Text(format!("{}\r\n", user_to_send).into()))
                        .map_err(|e| format!("Failed to send username: {}", e))?;

                    // Wait and read password prompt
                    let _ = socket.get_ref().set_read_timeout(Some(Duration::from_millis(200)));
                    if let Ok(msg2) = socket.read() {
                        if let Message::Text(ref text2) = msg2 {
                            let lower2 = text2.to_lowercase();
                            if lower2.contains("password") || lower2.contains("pwd:") {
                                // Send password
                                let pass_to_send = password.unwrap_or("admin");
                                socket
                                    .send(Message::Text(format!("{}\r\n", pass_to_send).into()))
                                    .map_err(|e| format!("Failed to send password: {}", e))?;
                            }
                        }
                    }
                }
            }
        }

        // Restore normal AT timeouts (500ms read timeout)
        socket.get_ref().set_read_timeout(Some(Duration::from_millis(500))).ok();
        socket.get_ref().set_write_timeout(Some(Duration::from_secs(3))).ok();

        Ok(Self { socket })
    }
}

impl AtTransport for WebSocketTransport {
    fn send_at(&mut self, command: &str) -> Result<String, String> {
        // Send AT command as text frame
        self.socket
            .send(Message::Text(format!("{}\r\n", command).into()))
            .map_err(|e| format!("WebSocket write error: {}", e))?;

        // Read response loop
        let mut response = String::new();
        let start = Instant::now();
        let timeout = Duration::from_secs(5);

        loop {
            if start.elapsed() > timeout {
                break;
            }

            match self.socket.read() {
                Ok(Message::Text(text)) => {
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    response.push_str(trimmed);
                    response.push('\n');

                    // Limit response size to prevent OOM
                    if response.len() > 1024 * 1024 {
                        return Err("Response exceeded 1MB limit".to_string());
                    }

                    if trimmed == "OK"
                        || trimmed.starts_with("ERROR")
                        || trimmed.starts_with("+CME ERROR")
                    {
                        break;
                    }
                }
                Ok(Message::Binary(_)) => {
                    // Ignore binary frames
                }
                Ok(Message::Close(_)) => {
                    return Err("Connection closed by server".to_string());
                }
                Ok(_) => {
                    // Ignore Ping/Pong frames
                }
                Err(tungstenite::Error::Io(ref e))
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    if !response.is_empty() {
                        break;
                    }
                }
                Err(e) => return Err(format!("WebSocket read error: {}", e)),
            }
        }

        Ok(response.trim().to_string())
    }

    fn close(&mut self) {
        let _ = self.socket.close(None);
    }

    fn is_alive(&self) -> bool {
        if let Ok(stream) = self.socket.get_ref().try_clone() {
            stream.set_nonblocking(true).ok();
            let mut buf = [0; 1];
            let res = stream.peek(&mut buf);
            stream.set_nonblocking(false).ok();
            match res {
                Ok(0) => false, // Connection closed (EOF)
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => true, // Still open
                Err(_) => false,
                Ok(_) => true,
            }
        } else {
            false
        }
    }
}
