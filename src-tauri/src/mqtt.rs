use modem_hal::transport::AtTransport;
use modem_hal::types::ModemStatus;
use modem_hal::ModemVendor;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;
use std::net::{IpAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqttCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub credentials: Option<MqttCredentials>,
}

impl MqttConfig {
    pub fn from_env() -> Result<Self, String> {
        Self::from_settings(
            std::env::var("MODEM_CAT_MQTT_HOST").ok().as_deref(),
            std::env::var("MODEM_CAT_MQTT_PORT").ok().as_deref(),
            std::env::var("MODEM_CAT_MQTT_USERNAME").ok().as_deref(),
            std::env::var("MODEM_CAT_MQTT_PASSWORD").ok().as_deref(),
        )
    }

    fn from_settings(
        host: Option<&str>,
        port: Option<&str>,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self, String> {
        let host = host
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                "MQTT is not configured: set MODEM_CAT_MQTT_HOST before enabling it".to_string()
            })?
            .to_string();
        let port_raw = port
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                "MQTT is not configured: set MODEM_CAT_MQTT_PORT before enabling it".to_string()
            })?;
        let port = port_raw.parse::<u16>().map_err(|_| {
            format!(
                "Invalid MODEM_CAT_MQTT_PORT value {:?}: expected an integer in 1-65535",
                port_raw
            )
        })?;

        let username = username.map(str::trim).filter(|s| !s.is_empty());
        let password = password.map(str::trim).filter(|s| !s.is_empty());
        let credentials = match (username, password) {
            (None, None) => None,
            (Some(user), Some(pass)) => Some(MqttCredentials {
                username: user.to_string(),
                password: pass.to_string(),
            }),
            _ => return Err(
                "MQTT username/password must be provided together; public defaults are forbidden"
                    .to_string(),
            ),
        };

        Ok(Self {
            host,
            port,
            credentials,
        })
    }
}

/// Detects the active outbound network interface IP address by simulating a UDP connect to the MQTT broker.
/// This queries the OS routing table without sending any actual network packets.
pub fn detect_outbound_ip(target: &str) -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect(target).ok()?;
    let local_addr = socket.local_addr().ok()?;

    let ip = local_addr.ip();
    if ip.is_loopback() || ip.is_unspecified() {
        None
    } else {
        Some(ip)
    }
}

/// MQTT must share the same live AT queue as IPC handlers.
/// We therefore lock in the same order (`transport -> vendor`) and use
/// `try_lock` so background publish never deadlocks or stalls foreground AT work.
fn try_query_modem_status(
    transport: &Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    vendor: &Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
) -> Result<Option<ModemStatus>, String> {
    let mut transport_guard = match transport.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Ok(None),
    };
    let mut vendor_guard = match vendor.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Ok(None),
    };

    let transport = match transport_guard.as_deref_mut() {
        Some(transport) => transport,
        None => return Ok(None),
    };
    let vendor = match vendor_guard.as_deref_mut() {
        Some(vendor) => vendor,
        None => return Ok(None),
    };

    vendor.query_modem_status(transport).map(Some)
}

/// The main MQTT client background loop.
/// Regularly queries the connected modem's status and publishes it to the remote broker.
pub async fn run_mqtt_loop(
    transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
    config: MqttConfig,
) {
    loop {
        log::info!("MQTT: Starting connection setup...");

        // 1. Detect outbound IP (the interface to bind to)
        let local_ip = detect_outbound_ip(&format!("{}:{}", config.host, config.port));
        log::info!("MQTT: Detected outbound IP: {:?}", local_ip);

        // 2. Query IMEI from the modem (if connected)
        let mut imei = "unknown_device".to_string();
        match try_query_modem_status(&transport, &vendor) {
            Ok(Some(status)) if !status.imei.is_empty() => {
                imei = status.imei.clone();
            }
            Ok(_) => {}
            Err(e) => log::warn!("MQTT: Failed to query modem status for client ID: {}", e),
        }

        let client_id = format!("modem_cat_{}", imei);
        log::info!("MQTT: Using Client ID: {}", client_id);

        // 3. Configure MQTT options
        let mut mqttoptions = MqttOptions::new(&client_id, &config.host, config.port);
        if let Some(creds) = &config.credentials {
            mqttoptions.set_credentials(&creds.username, &creds.password);
        }
        mqttoptions.set_keep_alive(Duration::from_secs(60));

        // 4. Create client & event loop
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // 5. Force bind to the detected outbound network interface
        // if let Some(ip) = local_ip {
        //     let mut net_opts = NetworkOptions::default();
        //     net_opts.bind_addr = Some(SocketAddr::new(ip, 0));
        //     eventloop.network_options = net_opts;
        // }

        // 6. Spawn the periodic publishing task (every 120 seconds)
        let publish_client = client.clone();
        let publish_vendor = vendor.clone();
        let publish_transport = transport.clone();
        let publish_imei = imei.clone();
        let mut interval = tokio::time::interval(Duration::from_secs(120));

        let pub_task = tokio::spawn(async move {
            loop {
                interval.tick().await;

                let mut payload = json!({
                    "clientId": format!("modem_cat_{}", publish_imei),
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                });

                // Fetch latest status if the modem is connected.
                //
                // AGENTS.md: "实时状态禁止 fallback" — 当 try_query_modem_status 因锁争用
                // 返回 Ok(None) 时，不能将"无法读取"与"调制解调器离线"混为一谈。
                // 只有在真正能拿到 OK/Err 结果时，才设置 modemConnected。
                let mut modem_connected = None; // None = could not read (lock contention)
                match try_query_modem_status(&publish_transport, &publish_vendor) {
                    Ok(Some(status)) => {
                        if let Ok(status_val) = serde_json::to_value(&status) {
                            if let Some(obj) = payload.as_object_mut() {
                                if let Some(status_obj) = status_val.as_object() {
                                    for (k, val) in status_obj {
                                        obj.insert(k.clone(), val.clone());
                                    }
                                }
                            }
                            modem_connected = Some(true);
                        }
                    }
                    Ok(None) => {
                        // Lock contention — transport busy. Leave modemConnected unset
                        // so consumer can distinguish offline from unreachable.
                    }
                    Err(e) => {
                        log::warn!("MQTT: Failed to query modem status for publish: {e}");
                        modem_connected = Some(false);
                    }
                }

                if let Some(connected) = modem_connected {
                    if let Some(obj) = payload.as_object_mut() {
                        obj.insert("modemConnected".to_string(), json!(connected));
                    }
                }

                let topic = format!("modem/status/{}", publish_imei);
                let payload_str = payload.to_string();
                log::info!("MQTT: Publishing status to {}: {}", topic, payload_str);

                if let Err(e) = publish_client
                    .publish(&topic, QoS::AtLeastOnce, false, payload_str)
                    .await
                {
                    log::error!("MQTT: Publish failed: {}", e);
                }
            }
        });

        // 7. Run the eventloop poll
        log::info!("MQTT: Starting event loop...");
        loop {
            match eventloop.poll().await {
                Ok(notification) => {
                    log::debug!("MQTT: Received notification: {:?}", notification);
                }
                Err(e) => {
                    log::error!("MQTT: Connection loop error: {}", e);
                    break; // break eventloop to reconnect
                }
            }
        }

        // Cleanup publish task before reconnecting
        pub_task.abort();
        log::warn!("MQTT: Disconnected. Reconnecting in 5 seconds...");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mqtt_config_requires_explicit_host_and_port() {
        let err = MqttConfig::from_settings(None, None, None, None)
            .expect_err("missing broker config must be rejected");
        assert!(
            err.contains("MODEM_CAT_MQTT_HOST"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn mqtt_config_rejects_partial_credentials() {
        let err =
            MqttConfig::from_settings(Some("broker.example"), Some("1883"), Some("user"), None)
                .expect_err("partial credentials must be rejected");
        assert!(err.contains("username/password"), "unexpected error: {err}");
    }

    #[test]
    fn mqtt_config_accepts_anonymous_and_explicit_modes() {
        let anonymous = MqttConfig::from_settings(Some("broker.example"), Some("1883"), None, None)
            .expect("anonymous broker should be allowed");
        assert!(anonymous.credentials.is_none());

        let explicit = MqttConfig::from_settings(
            Some("broker.example"),
            Some("1883"),
            Some("user"),
            Some("pass"),
        )
        .expect("explicit credentials should be allowed");
        let creds = explicit.credentials.expect("credentials should be present");
        assert_eq!(creds.username, "user");
        assert_eq!(creds.password, "pass");
    }
}
