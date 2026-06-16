use std::net::{IpAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use modem_hal::transport::AtTransport;
use modem_hal::ModemVendor;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;

/// Detects the active outbound network interface IP address by simulating a UDP connect to the MQTT broker.
/// This queries the OS routing table without sending any actual network packets.
pub fn detect_outbound_ip() -> Option<IpAddr> {
    let target = "82.157.177.161:1883";
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

/// The main MQTT client background loop.
/// Regularly queries the connected modem's status and publishes it to the remote broker.
pub async fn run_mqtt_loop(
    transport: Arc<Mutex<Option<Box<dyn AtTransport>>>>,
    vendor: Arc<Mutex<Option<Box<dyn ModemVendor>>>>,
) {
    loop {
        log::info!("MQTT: Starting connection setup...");
        
        // 1. Detect outbound IP (the interface to bind to)
        let local_ip = detect_outbound_ip();
        log::info!("MQTT: Detected outbound IP: {:?}", local_ip);
        
        // 2. Query IMEI from the modem (if connected)
        let mut imei = "unknown_device".to_string();
        if let Ok(mut vguard) = vendor.lock() {
            if let Some(v) = vguard.as_deref_mut() {
                if let Ok(mut tguard) = transport.lock() {
                    if let Some(t) = tguard.as_deref_mut() {
                        if let Ok(status) = v.query_modem_status(t) {
                            if !status.imei.is_empty() {
                                imei = status.imei.clone();
                            }
                        }
                    }
                }
            }
        }
        
        let client_id = format!("modem_cat_{}", imei);
        log::info!("MQTT: Using Client ID: {}", client_id);
        
        // 3. Configure MQTT options
        let mut mqttoptions = MqttOptions::new(&client_id, "82.157.177.161", 1883);
        mqttoptions.set_credentials("iot_client", "6yvqYJ6Y9dAa9p");
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
                
                // Fetch latest status if the modem is connected
                let mut status_fetched = false;
                if let Ok(mut vguard) = publish_vendor.lock() {
                    if let Some(v) = vguard.as_deref_mut() {
                        if let Ok(mut tguard) = publish_transport.lock() {
                            if let Some(t) = tguard.as_deref_mut() {
                                if let Ok(status) = v.query_modem_status(t) {
                                    if let Ok(status_val) = serde_json::to_value(&status) {
                                        if let Some(obj) = payload.as_object_mut() {
                                            if let Some(status_obj) = status_val.as_object() {
                                                for (k, val) in status_obj {
                                                    obj.insert(k.clone(), val.clone());
                                                }
                                            }
                                        }
                                        status_fetched = true;
                                    }
                                }
                            }
                        }
                    }
                }
                
                if !status_fetched {
                    if let Some(obj) = payload.as_object_mut() {
                        obj.insert("modemConnected".to_string(), json!(false));
                    }
                } else {
                    if let Some(obj) = payload.as_object_mut() {
                        obj.insert("modemConnected".to_string(), json!(true));
                    }
                }
                
                let topic = format!("modem/status/{}", publish_imei);
                let payload_str = payload.to_string();
                log::info!("MQTT: Publishing status to {}: {}", topic, payload_str);
                
                if let Err(e) = publish_client.publish(
                    &topic,
                    QoS::AtLeastOnce,
                    false,
                    payload_str,
                ).await {
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

    #[tokio::test]
    async fn test_mqtt_connection() {
        let ip = detect_outbound_ip();
        println!("Test MQTT: Outbound IP detected = {:?}", ip);
        
        let client_id = "modem_cat_test_client";
        let mut mqttoptions = MqttOptions::new(client_id, "82.157.177.161", 1883);
        mqttoptions.set_credentials("iot_client", "6yvqYJ6Y9dAa9p");
        mqttoptions.set_keep_alive(Duration::from_secs(10));
        
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 5);
        if let Some(local_ip) = ip {
            println!("Test MQTT: Routing connection via active network interface IP: {}", local_ip);
        }
        
        let client_clone = client.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("Test MQTT: Publishing validation message...");
            let res = client_clone.publish(
                "modem/test",
                QoS::AtLeastOnce,
                false,
                "Hello from modem-cat integration test!",
            ).await;
            println!("Test MQTT: Publish result = {:?}", res);
        });
        
        let mut success = false;
        let start = std::time::Instant::now();
        while start.elapsed() < Duration::from_secs(10) {
            match eventloop.poll().await {
                Ok(notification) => {
                    println!("Test MQTT: Notification = {:?}", notification);
                    if let rumqttc::Event::Incoming(rumqttc::Incoming::PubAck(_)) = notification {
                        println!("Test MQTT: Successfully received PubAck!");
                        success = true;
                        break;
                    }
                }
                Err(e) => {
                    println!("Test MQTT: Error = {:?}", e);
                    break;
                }
            }
        }
        assert!(success, "Failed to connect and publish message successfully within timeout");
    }
}
