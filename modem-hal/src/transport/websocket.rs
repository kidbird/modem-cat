use crate::transport::AtTransport;
use crate::transport::is_complete_response;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::header::AUTHORIZATION;
use tungstenite::{client, Message, WebSocket};

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
            let auth_base64 = BASE64_STANDARD.encode(auth.as_bytes());
            let auth_value = format!("Basic {}", auth_base64)
                .parse()
                .map_err(|e| format!("Invalid WS auth header value: {e}"))?;
            request.headers_mut().insert(AUTHORIZATION, auth_value);
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
        tcp_stream
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();
        tcp_stream
            .set_write_timeout(Some(Duration::from_secs(3)))
            .ok();

        // 4. Perform WebSocket client handshake
        let (mut socket, _response) = client(request, tcp_stream)
            .map_err(|e| format!("WebSocket handshake failed: {}", e))?;

        // 5. Interactive terminal/console authentication fallback.
        // If the server doesn't use Basic Auth but presents a login prompt over the WebSocket stream,
        // we wait briefly and respond if requested.
        //
        // AGENTS.md: "敏感信息禁止公开默认值 / WebSocket 凭据不得偷补公开默认值" —
        // 若用户未提供 WS 凭据，则跳过交互式认证分支，让服务器拒绝未认证连接。
        // 不再 fallback 到硬编码的 "admin"。
        if let (Some(user_provided), Some(pass_provided)) = (username, password) {
            let _ = socket
                .get_ref()
                .set_read_timeout(Some(Duration::from_millis(200)));
            if let Ok(msg) = socket.read() {
                if let Message::Text(ref text) = msg {
                    let lower = text.to_ascii_lowercase();
                    if lower.contains("login") || lower.contains("username") || lower.contains("user:")
                    {
                        // Send username
                        socket
                            .send(Message::Text(format!("{}\r\n", user_provided).into()))
                            .map_err(|e| format!("Failed to send username: {e}"))?;

                        // Wait and read password prompt
                        let _ = socket
                            .get_ref()
                            .set_read_timeout(Some(Duration::from_millis(200)));
                        if let Ok(msg2) = socket.read() {
                            if let Message::Text(ref text2) = msg2 {
                                let lower2 = text2.to_ascii_lowercase();
                                if lower2.contains("password") || lower2.contains("pwd:") {
                                    // Send password
                                    socket
                                        .send(Message::Text(format!("{}\r\n", pass_provided).into()))
                                        .map_err(|e| format!("Failed to send password: {e}"))?;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Restore normal AT timeouts (500ms read timeout)
        socket
            .get_ref()
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();
        socket
            .get_ref()
            .set_write_timeout(Some(Duration::from_secs(3)))
            .ok();

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
                // Overall timeout: only return Ok if response is terminally complete.
                let trimmed = response.trim();
                if !trimmed.is_empty() && is_complete_response(trimmed) {
                    return Ok(trimmed.to_string());
                }
                return Err(format!(
                    "WebSocket read timeout after {:?} (response incomplete, {} bytes discarded)",
                    timeout,
                    response.len()
                ));
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

                    if is_complete_response(trimmed) {
                        break;
                    }
                }
                Ok(Message::Binary(_)) => {
                    // Ignore binary frames
                }
                Ok(Message::Close(_)) => {
                    // Server closed. Return what we have if complete.
                    let trimmed = response.trim();
                    if !trimmed.is_empty() && is_complete_response(trimmed) {
                        return Ok(trimmed.to_string());
                    }
                    return Err("Connection closed by server (incomplete response)".to_string());
                }
                Ok(_) => {
                    // Ignore Ping/Pong frames
                }
                Err(tungstenite::Error::Io(ref e))
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    if !response.is_empty() && is_complete_response(response.trim()) {
                        break;
                    }
                    // Partial or no data + wouldblock: keep waiting. The overall
                    // timeout check above handles the truncation failure case.
                }
                Err(e) => return Err(format!("WebSocket read error: {}", e)),
            }
        }

        Ok(response.trim().to_string())
    }

    fn close(&mut self) {
        let _ = self.socket.close(None);
    }

    fn force_shutdown(&mut self) {
        // Close the underlying TCP stream directly without sending a WS close
        // frame. shutdown(Both) sends FIN immediately — non-blocking — so this
        // never hangs even if the gateway is gone (USB-serial gateway unplugged).
        // The kernel reclaims the fd; Drop then drops the now-dead socket.
        let _ = self.socket.get_ref().shutdown(std::net::Shutdown::Both);
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
