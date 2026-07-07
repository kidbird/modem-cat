use modem_hal::types::*;
use modem_hal::{validate_at_string, validate_cid, validate_raw_at_command};
use std::net::Ipv4Addr;
use std::sync::atomic::Ordering;

use crate::{mqtt, AppState};

fn current_data_cid(
    data_cid: &std::sync::Arc<std::sync::atomic::AtomicI32>,
) -> Result<i32, String> {
    let cid = data_cid.load(Ordering::Relaxed);
    validate_cid(cid)?;
    Ok(cid)
}

fn parse_ipv4_field(value: &str, field: &str) -> Result<String, String> {
    validate_at_string(value)?;
    value
        .parse::<Ipv4Addr>()
        .map(|addr| addr.to_string())
        .map_err(|_| format!("Invalid {} IPv4 address: {}", field, value))
}

// ── High-level modem queries ──

#[tauri::command]
pub(crate) async fn get_modem_status(
    state: tauri::State<'_, AppState>,
) -> Result<ModemStatus, String> {
    with_vendor!(state, |t, v| v.query_modem_status(t))
}

#[tauri::command]
pub(crate) async fn get_hardware_info(
    state: tauri::State<'_, AppState>,
) -> Result<HardwareInfo, String> {
    let mut info = with_vendor!(state, |t, v| v.query_hardware_info(t))?;
    let stored_ids = *state
        .connected_usb_ids
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;

    let resolved_ids = stored_ids.or(info.usb_vid.zip(info.usb_pid));
    if let Some((vid, pid)) = resolved_ids {
        info.usb_vid = Some(vid);
        info.usb_pid = Some(pid);
    }
    Ok(info)
}

#[tauri::command]
pub(crate) async fn get_ip_info(state: tauri::State<'_, AppState>) -> Result<IpInfo, String> {
    with_vendor_cid!(state, |t, v, cid| {
        validate_cid(cid)?;
        v.query_ip_info(t, cid)
    })
}

#[tauri::command]
pub(crate) async fn get_apn_list(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ApnEntry>, String> {
    with_vendor!(state, |t, v| v.query_apn_list(t))
}

#[tauri::command]
pub(crate) async fn get_neighbor_cells(
    state: tauri::State<'_, AppState>,
) -> Result<NeighborCells, String> {
    with_vendor!(state, |t, v| v.query_neighbor_cells(t))
}

#[tauri::command]
pub(crate) async fn get_qos_info(state: tauri::State<'_, AppState>) -> Result<QosInfo, String> {
    with_vendor_cid!(state, |t, v, cid| {
        validate_cid(cid)?;
        v.query_qos(t, cid)
    })
}

#[tauri::command]
pub(crate) async fn get_network_mode(state: tauri::State<'_, AppState>) -> Result<String, String> {
    with_vendor!(state, |t, v| v.query_network_mode(t))
}

#[tauri::command]
pub(crate) async fn get_ims_enabled(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    with_vendor!(state, |t, v| v.query_ims_enabled(t))
}

#[tauri::command]
pub(crate) async fn set_ims_enabled(
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_ims_enabled(t, enabled))
}

#[tauri::command]
pub(crate) async fn get_cfun_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_cfun_mode(t))
}

#[tauri::command]
pub(crate) async fn set_mtu(value: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    if !(576..=9000).contains(&value) {
        return Err(format!("Invalid MTU {}: must be between 576 and 9000", value));
    }
    with_vendor!(state, |t, v| v.set_mtu(t, value))
}

#[tauri::command]
pub(crate) async fn get_lan_config(
    state: tauri::State<'_, AppState>,
) -> Result<LanConfig, String> {
    with_vendor!(state, |t, v| v.query_lan_config(t))
}

#[tauri::command]
pub(crate) async fn set_lan_config(
    gateway: String,
    netmask: Option<String>,
    dhcp_start: String,
    dhcp_end: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let gateway = parse_ipv4_field(&gateway, "gateway")?;
    let dhcp_start = parse_ipv4_field(&dhcp_start, "DHCP start")?;
    let dhcp_end = parse_ipv4_field(&dhcp_end, "DHCP end")?;
    let netmask = match netmask {
        Some(mask) if !mask.trim().is_empty() => Some(parse_ipv4_field(mask.trim(), "netmask")?),
        _ => None,
    };
    with_vendor!(state, |t, v| v.set_lan_config(
        t,
        &LanConfig {
            gateway,
            netmask,
            dhcp_start,
            dhcp_end,
        }
    ))
}

#[tauri::command]
pub(crate) async fn set_dmz(
    ip: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let ip = parse_ipv4_field(&ip, "DMZ host")?;
    with_vendor!(state, |t, v| v.set_dmz(t, &ip))
}

#[tauri::command]
pub(crate) async fn clear_dmz(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.clear_dmz(t))
}

// ── Write operations ──

#[tauri::command]
pub(crate) async fn set_apn_config(
    cid: i32,
    context_type: i32,
    apn: String,
    username: String,
    password: String,
    auth_type: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    validate_at_string(&apn)?;
    validate_at_string(&username)?;
    validate_at_string(&password)?;
    with_vendor!(state, |t, v| v.set_apn(
        t,
        cid,
        context_type,
        &apn,
        &username,
        &password,
        auth_type
    ))
}

#[tauri::command]
pub(crate) async fn delete_apn_config(
    cid: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    with_vendor!(state, |t, v| v.delete_apn(t, cid))
}

#[tauri::command]
pub(crate) async fn set_apn_active(
    cid: i32,
    active: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    with_vendor!(state, |t, v| v.set_apn_active(t, cid, active))
}

#[tauri::command]
pub(crate) async fn connect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let transport = state.transport.clone();
    let vendor = state.vendor.clone();
    let data_cid = state.data_cid.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        let mut vguard = vendor.lock().map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        let v = vguard.as_deref_mut().ok_or("No vendor detected")?;
        let cid = current_data_cid(&data_cid)?;
        v.connect_data(t, cid)?;
        data_cid.store(cid, Ordering::Relaxed);
        Ok(())
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub(crate) async fn get_5glan(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<L5GanEntry>, String> {
    with_vendor!(state, |t, v| v.query_5glan(t))
}

#[tauri::command]
pub(crate) async fn set_5glan(
    cid: i32,
    enabled: bool,
    vlan_id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    with_vendor!(state, |t, v| v.set_5glan(t, cid, enabled, vlan_id))
}

#[tauri::command]
pub(crate) fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub(crate) async fn configure_qualcomm_5glan(
    cid: i32,
    apn: String,
    snssai: String,
    profile_id: i32,
    vlan_start: i32,
    vlan_end: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_cid(cid)?;
    validate_at_string(&apn)?;
    validate_at_string(&snssai)?;
    with_vendor!(state, |t, v| v.configure_qualcomm_5glan(
        t, cid, &apn, &snssai, profile_id, vlan_start, vlan_end
    ))
}

#[tauri::command]
pub(crate) async fn enable_eth_pdu(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.enable_eth_pdu(t))
}

#[tauri::command]
pub(crate) async fn connect_qualcomm_5glan(
    rule_id: i32,
    cid: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.connect_qualcomm_5glan(t, rule_id, cid))
}

#[tauri::command]
pub(crate) async fn query_qualcomm_5glan_status(
    state: tauri::State<'_, AppState>,
) -> Result<Qualcomm5GlanStatus, String> {
    with_vendor!(state, |t, v| v.query_qualcomm_5glan_status(t))
}

#[tauri::command]
pub(crate) async fn get_vlan(state: tauri::State<'_, AppState>) -> Result<Vec<i32>, String> {
    with_vendor!(state, |t, v| v.query_vlan(t))
}

#[tauri::command]
pub(crate) async fn set_vlan(
    vlan_id: i32,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_vlan(t, vlan_id, enabled))
}

#[tauri::command]
pub(crate) async fn disconnect_data(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor_cid!(state, |t, v, cid| {
        validate_cid(cid)?;
        v.disconnect_data(t, cid)
    })
}

#[tauri::command]
pub(crate) async fn set_network_mode_cmd(
    mode: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&mode)?;
    with_vendor!(state, |t, v| v.set_network_mode(t, &mode))
}

#[tauri::command]
pub(crate) async fn set_nr5g_band_cmd(
    band: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&band)?;
    with_vendor!(state, |t, v| v.set_nr5g_bands(t, &band))
}

#[tauri::command]
pub(crate) async fn reboot_modem(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.reboot(t))
}

#[tauri::command]
pub(crate) async fn set_cfun(mode: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_cfun(t, mode))
}

#[tauri::command]
pub(crate) async fn factory_reset(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.factory_reset(t))
}

#[tauri::command]
pub(crate) async fn get_sim_slot(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_sim_slot(t))
}

#[tauri::command]
pub(crate) async fn set_sim_slot(
    slot: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.switch_sim_slot(t, slot))
}

#[tauri::command]
pub(crate) async fn send_raw_at(
    command: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    validate_raw_at_command(&command)?;
    let transport = state.transport.clone();
    tokio::task::spawn_blocking(move || {
        let mut tguard = transport
            .lock()
            .map_err(|e| format!("Lock poisoned: {}", e))?;
        let t = tguard.as_deref_mut().ok_or("Not connected")?;
        t.send_at(&command)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub(crate) async fn query_cell_lock(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<CellLockEntry>, String> {
    with_vendor!(state, |t, v| v.query_cell_lock(t))
}

#[tauri::command]
pub(crate) async fn set_cell_lock(
    arfcn: String,
    pci: String,
    scs: String,
    band: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&arfcn)?;
    validate_at_string(&pci)?;
    validate_at_string(&scs)?;
    validate_at_string(&band)?;
    with_vendor!(state, |t, v| v.set_cell_lock(t, &arfcn, &pci, &scs, &band))
}

#[tauri::command]
pub(crate) async fn clear_cell_lock(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.clear_cell_lock(t))
}

#[tauri::command]
pub(crate) async fn set_plmn_lock(
    plmn: String,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&plmn)?;
    let pw = password.ok_or_else(|| {
        "PLMN lock requires a password (the device-specific unlock code, NOT the public default)"
            .to_string()
    })?;
    validate_at_string(&pw)?;
    with_vendor!(state, |t, v| v.set_plmn_lock(t, &plmn, &pw))
}

#[tauri::command]
pub(crate) async fn clear_plmn_lock(
    password: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let pw = password.ok_or_else(|| "PLMN unlock requires a password".to_string())?;
    validate_at_string(&pw)?;
    with_vendor!(state, |t, v| v.clear_plmn_lock(t, &pw))
}

#[tauri::command]
pub(crate) fn pop_at_commands(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut log = state
        .at_command_log
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;
    Ok(log.drain(..).collect())
}

/// Export the AT terminal text to a user-chosen file via a native save dialog.
///
/// `content` is the full terminal text assembled by the frontend from the
/// on-screen DOM (the user's "what you see is what you export" view). This does
/// NOT touch the backend AT ring buffer, which only holds redacted command
/// echoes without their full responses. Default file name is `at_log.txt`,
/// default directory is the program's own directory (`current_exe` parent).
/// Returns `Ok(None)` when the user cancels the dialog.
#[tauri::command]
pub(crate) async fn export_at_log(
    app: tauri::AppHandle,
    content: String,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let (tx, rx) = tokio::sync::oneshot::channel();
    let mut builder = app
        .dialog()
        .file()
        .set_file_name("at_log.txt")
        .add_filter("文本文件", &["txt"]);
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            builder = builder.set_directory(dir);
        }
    }
    builder.save_file(move |file_path| {
        let _ = tx.send(file_path);
    });

    let Some(file_path) = rx.await.map_err(|e| e.to_string())? else {
        return Ok(None);
    };
    let path = file_path
        .into_path()
        .map_err(|e| format!("解析路径失败: {e}"))?;
    std::fs::write(&path, &content).map_err(|e| format!("写入文件失败: {e}"))?;
    Ok(Some(path.to_string_lossy().into_owned()))
}

#[tauri::command]
pub(crate) async fn get_bands(state: tauri::State<'_, AppState>) -> Result<BandConfig, String> {
    with_vendor!(state, |t, v| v.query_bands_with_spec(t))
}

#[tauri::command]
pub(crate) async fn set_bands(
    lte: String,
    nr: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&lte)?;
    validate_at_string(&nr)?;
    with_vendor!(state, |t, v| v.set_bands(t, &lte, &nr))
}

#[tauri::command]
pub(crate) async fn reset_all_bands(state: tauri::State<'_, AppState>) -> Result<(), String> {
    with_vendor!(state, |t, v| v.reset_all_bands(t))
}

#[tauri::command]
pub(crate) async fn get_feature_toggles(
    state: tauri::State<'_, AppState>,
) -> Result<FeatureToggles, String> {
    with_vendor!(state, |t, v| v.query_feature_toggles(t))
}

#[tauri::command]
pub(crate) async fn set_feature_toggle(
    feature: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&feature)?;
    with_vendor!(state, |t, v| v.set_feature_toggle(t, &feature, enabled))
}

#[tauri::command]
pub(crate) async fn get_qualcomm_config(
    state: tauri::State<'_, AppState>,
) -> Result<QualcommConfig, String> {
    with_vendor!(state, |t, v| v.query_qualcomm_config(t))
}

#[tauri::command]
pub(crate) async fn set_qualcomm_config(
    param: String,
    value: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    validate_at_string(&param)?;
    validate_at_string(&value)?;
    with_vendor!(state, |t, v| v.set_qualcomm_config(t, &param, &value))
}

#[tauri::command]
pub(crate) async fn get_traffic(state: tauri::State<'_, AppState>) -> Result<TrafficInfo, String> {
    with_vendor!(state, |t, v| v.query_traffic(t))
}

#[tauri::command]
pub(crate) async fn get_usbnet_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_usbnet_mode(t))
}

#[tauri::command]
pub(crate) async fn set_usbnet_mode(
    mode: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_usbnet_mode(t, mode))
}

#[tauri::command]
pub(crate) async fn get_nat_mode(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    with_vendor!(state, |t, v| v.query_nat_mode(t))
}

#[tauri::command]
pub(crate) async fn set_nat_mode(
    mode: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    with_vendor!(state, |t, v| v.set_nat_mode(t, mode))
}

// ── MQTT ──

#[tauri::command]
pub(crate) async fn set_mqtt_enabled(
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut task_guard = state
        .mqtt_task
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;
    if enabled {
        if task_guard.is_none() {
            let config = mqtt::MqttConfig::from_env()?;
            log::info!("MQTT: Enabling remote connection...");
            let transport = state.transport.clone();
            let vendor = state.vendor.clone();
            let handle = tokio::spawn(async move {
                mqtt::run_mqtt_loop(transport, vendor, config).await;
            });
            *task_guard = Some(handle);
        }
    } else if let Some(handle) = task_guard.take() {
        log::info!("MQTT: Disabling remote connection...");
        handle.abort();
    }
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_mqtt_enabled(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let task_guard = state
        .mqtt_task
        .lock()
        .map_err(|e| format!("Lock poisoned: {}", e))?;
    Ok(task_guard.is_some())
}
