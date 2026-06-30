use crate::transport::AtTransport;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::header::AUTHORIZATION;
use tungstenite::{client, Message, WebSocket};

#[derive(Debug, Clone, PartialEq, Eq)]
struct WebSocketCredentials {
    username: String,
    password: String,
}

fn normalize_credentials(
    username: Option<&str>,
    password: Option<&str>,
) -> Result<Option<WebSocketCredentials>, String> {
    let username = username.map(str::trim).filter(|s| !s.is_empty());
    let password = password.map(str::trim).filter(|s| !s.is_empty());

    match (username, password) {
        (None, None) => Ok(None),
        (Some(user), Some(pass)) => Ok(Some(WebSocketCredentials {
            username: user.to_string(),
            password: pass.to_string(),
        })),
        _ => Err(
            "WebSocket username/password must be provided together; public defaults are forbidden"
                .to_string(),
        ),
    }
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
        let credentials = normalize_credentials(username, password)?;

        // 1. Create client request
        let mut request = url
            .as_str()
            .into_client_request()
            .map_err(|e| format!("Invalid WebSocket URL: {}", e))?;

        // 2. Add HTTP Basic Authentication if credentials are provided
        if let Some(creds) = &credentials {
            let auth = format!("{}:{}", creds.username, creds.password);
            let auth_base64 = BASE64_STANDARD.encode(auth.as_bytes());
            request.headers_mut().insert(
                AUTHORIZATION,
                format!("Basic {}", auth_base64)
                    .parse()
                    .map_err(|e| format!("Invalid WebSocket authorization header: {}", e))?,
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
        tcp_stream
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();
        tcp_stream
            .set_write_timeout(Some(Duration::from_secs(3)))
            .ok();

        // 4. Perform WebSocket client handshake
        let (mut socket, _response) = client(request, tcp_stream)
            .map_err(|e| format!("WebSocket handshake failed: {}", e))?;

        // 5. Interactive terminal/console authentication fallback
        // If the server doesn't use Basic Auth but presents a login prompt over the WebSocket stream,
        // we wait briefly and respond if requested.
        let _ = socket
            .get_ref()
            .set_read_timeout(Some(Duration::from_millis(200)));
        if let Ok(msg) = socket.read() {
            if let Message::Text(ref text) = msg {
                let lower = text.to_lowercase();
                if lower.contains("login") || lower.contains("username") || lower.contains("user:")
                {
                    let creds = credentials.as_ref().ok_or_else(|| {
                        "WebSocket gateway requested credentials, but none were provided".to_string()
                    })?;
                    // Send username
                    socket
                        .send(Message::Text(format!("{}\r\n", creds.username).into()))
                        .map_err(|e| format!("Failed to send username: {}", e))?;

                    // Wait and read password prompt
                    let _ = socket
                        .get_ref()
                        .set_read_timeout(Some(Duration::from_millis(200)));
                    if let Ok(msg2) = socket.read() {
                        if let Message::Text(ref text2) = msg2 {
                            let lower2 = text2.to_lowercase();
                            if lower2.contains("password") || lower2.contains("pwd:") {
                                // Send password
                                socket
                                    .send(Message::Text(format!("{}\r\n", creds.password).into()))
                                    .map_err(|e| format!("Failed to send password: {}", e))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_credentials_rejects_partial_pair() {
        let err = normalize_credentials(Some("user"), None)
            .expect_err("partial credentials must be rejected");
        assert!(err.contains("username/password"), "unexpected error: {err}");
    }

    #[test]
    fn normalize_credentials_preserves_anonymous_mode() {
        let creds = normalize_credentials(None, None).expect("anonymous mode should be allowed");
        assert!(creds.is_none(), "anonymous mode must not synthesize admin/admin");
    }

    #[test]
    fn normalize_credentials_trims_and_returns_explicit_pair() {
        let creds = normalize_credentials(Some(" user "), Some(" pass "))
            .expect("explicit credentials should be accepted")
            .expect("credentials should be present");
        assert_eq!(creds.username, "user");
        assert_eq!(creds.password, "pass");
    }
}
