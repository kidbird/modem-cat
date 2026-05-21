use modem_hal::types::*;

use crate::AppState;

// ── Read commands ──

#[tauri::command]
pub async fn get_modem_status(state: tauri::State<'_, AppState>) -> Result<ModemStatus, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_modem_status(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_hardware_info(state: tauri::State<'_, AppState>) -> Result<HardwareInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_hardware_info(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_ip_info(state: tauri::State<'_, AppState>) -> Result<IpInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().unwrap();
        v.query_ip_info(t, if cid > 0 { cid } else { 1 })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_apn_list(state: tauri::State<'_, AppState>) -> Result<Vec<ApnEntry>, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_apn_list(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_neighbor_cells(
    state: tauri::State<'_, AppState>,
) -> Result<NeighborCells, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_neighbor_cells(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_qos_info(state: tauri::State<'_, AppState>) -> Result<QosInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().unwrap();
        v.query_qos(t, if cid > 0 { cid } else { 1 })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_network_mode(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_network_mode(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_bands(state: tauri::State<'_, AppState>) -> Result<BandConfig, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_bands_with_spec(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_feature_toggles(
    state: tauri::State<'_, AppState>,
) -> Result<FeatureToggles, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_feature_toggles(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_usbnet_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_usbnet_mode(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_traffic(state: tauri::State<'_, AppState>) -> Result<TrafficInfo, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_traffic(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_5glan(state: tauri::State<'_, AppState>) -> Result<Vec<L5GanEntry>, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_5glan(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn get_sim_slot(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.query_sim_slot(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

// ── Write commands ──

#[tauri::command]
pub async fn set_apn_config(
    cid: i32,
    context_type: i32,
    apn: String,
    username: String,
    password: String,
    auth_type: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_apn(t, cid, context_type, &apn, &username, &password, auth_type)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn delete_apn_config(
    cid: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.delete_apn(t, cid)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_apn_active(
    cid: i32,
    active: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_apn_active(t, cid, active)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_5glan(
    cid: i32,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_5glan(t, cid, enabled)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn connect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().unwrap();
        let cid = if cid > 0 { cid } else { 1 };
        v.connect_data(t, cid)?;
        *data_cid.lock().unwrap() = cid;
        Ok(())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn disconnect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = *data_cid.lock().unwrap();
        let cid = if cid > 0 { cid } else { 1 };
        v.disconnect_data(t, cid)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_network_mode_cmd(
    mode: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_network_mode(t, &mode)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_nr5g_band_cmd(
    band: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_nr5g_bands(t, &band)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_bands(
    lte: String,
    nr: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_bands(t, &lte, &nr)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn reset_all_bands(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.reset_all_bands(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_feature_toggle(
    feature: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_feature_toggle(t, &feature, enabled)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_usbnet_mode(
    mode: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.set_usbnet_mode(t, mode)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn reboot_modem(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.reboot(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn factory_reset(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.factory_reset(t)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn set_sim_slot(slot: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.switch_sim_slot(t, slot)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn send_raw_at(
    command: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport.lock().unwrap();
        let mut vguard = vendor.lock().unwrap();
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        v.send_raw_at(t, &command)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

/// Return all AT commands logged by internal operations since last call, then clear the log.
#[tauri::command]
pub fn pop_at_commands(state: tauri::State<'_, AppState>) -> Vec<String> {
    let mut log = state.at_command_log.lock().unwrap();
    std::mem::take(&mut *log)
}
