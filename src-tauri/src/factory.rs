//! Factory mode: SN generation, device communication, product CRUD, CSV records.
//!
//! Communicates with the device via HTTP REST API (not AT commands).
//! Data persisted in the Tauri app data directory.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Datelike;
use tauri::{AppHandle, Manager};

/// Lock a FactoryState Mutex field, converting poison into a String error.
/// AGENTS.md: "运行时锁路径禁止 panic；必须返回错误或记录明确日志。"
macro_rules! lock_field {
    ($state:expr, $field:ident) => {
        $state.$field.lock().map_err(|e| format!("Lock poisoned: {e}"))
    };
}

// ─── Types ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "Brand")]
    pub brand: String,
    #[serde(rename = "Type")]
    pub product_type: String,
    #[serde(rename = "Fac")]
    pub fac: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub index: i32,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseData {
    #[serde(default)]
    pub brands: Vec<Item>,
    #[serde(rename = "types", default)]
    pub product_types: Vec<Item>,
    #[serde(default)]
    pub factories: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSet {
    pub brand_code: String,
    pub type_code: String,
    pub fac_code: String,
    pub year_code: String,
    pub mon_code: String,
    pub seq_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecuteData {
    pub date_str: String,
    #[serde(rename = "Type")]
    pub product_type: String,
    pub prefix_str: String,
    #[serde(rename = "CurretSeqNo")]
    pub current_seq_no: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecuteDataList {
    pub exe_data_list: Vec<ExecuteData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub imei: String,
    pub iccid: String,
    pub sn: String,
    pub sw_version: String,
    pub device_name: String,
    pub timestamp: String,
    pub activated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceActivateInfo {
    pub firmware_version: String,
    pub imei: String,
    pub iccid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceNameData {
    pub device_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialNumberData {
    pub serial_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationStatus {
    pub valid: bool,
    pub level: i32,
    pub activate_time: i64,
    pub expires_time: i64,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyData {}

// ─── Factory AppState ───────────────────────────────────────────────────────

pub struct FactoryState {
    pub device_client: Mutex<Option<DeviceClient>>,
    pub data_manager: Mutex<Option<DataManager>>,
    pub current_product: Mutex<Product>,
    pub current_code_set: Mutex<CodeSet>,
    pub execute_data: Mutex<Option<ExecuteDataList>>,
    pub base_data: Mutex<BaseData>,
}

impl FactoryState {
    pub fn new() -> Self {
        Self {
            device_client: Mutex::new(None),
            data_manager: Mutex::new(None),
            current_product: Mutex::new(Product {
                brand: String::new(),
                product_type: String::new(),
                fac: String::new(),
            }),
            current_code_set: Mutex::new(CodeSet {
                brand_code: String::new(),
                type_code: String::new(),
                fac_code: String::new(),
                year_code: String::new(),
                mon_code: String::new(),
                seq_code: "00001".to_string(),
            }),
            execute_data: Mutex::new(None),
            base_data: Mutex::new(BaseData::default()),
        }
    }
}

// ─── SN Generator ───────────────────────────────────────────────────────────

fn get_year_code() -> String {
    let year = chrono::Local::now().format("%Y").to_string();
    year.chars().last().unwrap_or('0').to_string()
}

fn get_mon_code() -> String {
    let month = chrono::Local::now().month() as u32;
    format!("{:X}", month).to_uppercase()
}

fn increment_seq(seq: &str) -> Result<String, String> {
    let number: u32 = seq.parse().map_err(|_| "Invalid sequence number")?;
    let incremented = number + 1;
    if incremented > 99999 {
        return Err("序列号超过最大值 (99999)".to_string());
    }
    Ok(format!("{:05}", incremented))
}

fn generate_sn(code_set: &CodeSet) -> String {
    format!(
        "{}{}{}{}{}{}",
        code_set.brand_code,
        code_set.type_code,
        code_set.fac_code,
        code_set.year_code,
        code_set.mon_code,
        code_set.seq_code
    )
}

fn update_code_set(code_set: &mut CodeSet) {
    code_set.year_code = get_year_code();
    code_set.mon_code = get_mon_code();
}

fn resolve_codes(base_data: &BaseData, product: &Product, code_set: &mut CodeSet) {
    for brand in &base_data.brands {
        if brand.name == product.brand {
            code_set.brand_code = brand.code.clone();
            break;
        }
    }
    for type_item in &base_data.product_types {
        if type_item.name == product.product_type {
            code_set.type_code = type_item.code.clone();
            break;
        }
    }
    for factory in &base_data.factories {
        if factory.name == product.fac {
            code_set.fac_code = factory.code.clone();
            break;
        }
    }
}

fn next_index(items: &[Item]) -> i32 {
    items.iter().map(|i| i.index).max().unwrap_or(0) + 1
}

// ─── HTTP Device Client ─────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DeviceClient {
    client: Client,
    ip: String,
}

impl DeviceClient {
    pub fn new(ip: &str) -> Result<Self, String> {
        // 校验 IP/主机名格式，避免静默生成无效 URL。
        let _ = ip.parse::<std::net::IpAddr>()
            .map_err(|_| format!("无效的设备 IP 地址: '{ip}'（IPv4/IPv6 格式错误）"))?;

        // 工厂设备使用 HTTP 内网通信，证书校验在受控内网环境下关闭；
        // 但构建失败必须报错，不能静默回退到无超时的 Client::new()。
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {e}"))?;
        Ok(Self {
            client,
            ip: ip.to_string(),
        })
    }

    pub fn update_ip(&mut self, ip: &str) {
        self.ip = ip.to_string();
    }

    fn base_url(&self) -> String {
        format!("http://{}/api", self.ip)
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, action: &str) -> Result<T, String> {
        let url = format!("{}/{}", self.base_url(), action);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        if !response.status().is_success() {
            return Err(format!("HTTP 错误: {}", response.status()));
        }
        response
            .json()
            .await
            .map_err(|e| format!("响应解析失败: {}", e))
    }

    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        action: &str,
        body: serde_json::Value,
    ) -> Result<T, String> {
        let url = format!("{}/{}", self.base_url(), action);
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        if !response.status().is_success() {
            return Err(format!("HTTP 错误: {}", response.status()));
        }
        response
            .json()
            .await
            .map_err(|e| format!("响应解析失败: {}", e))
    }

    pub async fn get_device_info(&self) -> Result<DeviceInfo, String> {
        let activate_info: ApiResponse<DeviceActivateInfo> =
            self.get("device_activate_info").await?;
        if activate_info.code != 200 {
            return Err(format!("API 错误: {}", activate_info.message));
        }
        let name_resp: ApiResponse<DeviceNameData> = self.get("device_name_get").await?;
        let device_name = if name_resp.code == 200 {
            name_resp.data.device_name
        } else {
            String::new()
        };
        let sn_resp: ApiResponse<SerialNumberData> = self.get("device_sn_get").await?;
        let sn = if sn_resp.code == 200 {
            sn_resp.data.serial_number
        } else {
            String::new()
        };
        let activated = self
            .get::<ApiResponse<ActivationStatus>>("license_get")
            .await
            .map(|r| r.data.valid)
            .unwrap_or(false);

        Ok(DeviceInfo {
            imei: activate_info.data.imei,
            iccid: activate_info.data.iccid,
            sn,
            sw_version: activate_info.data.firmware_version,
            device_name,
            activated,
            timestamp: chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        })
    }

    pub async fn set_device_sn(&self, sn: &str) -> Result<bool, String> {
        let body = json!({ "serial_number": sn });
        let resp: ApiResponse<EmptyData> = self.post("device_sn_set", body).await?;
        if resp.code != 200 {
            return Err(format!("API 错误: {}", resp.message));
        }
        let readback: ApiResponse<SerialNumberData> = self.get("device_sn_get").await?;
        Ok(readback.data.serial_number == sn)
    }
}

// ─── Data Manager ───────────────────────────────────────────────────────────

pub struct DataManager {
    data_dir: PathBuf,
}

impl DataManager {
    pub fn new(data_dir: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建数据目录失败 '{}': {}", data_dir.display(), e))?;
        Ok(Self { data_dir })
    }

    fn read_json<T: serde::de::DeserializeOwned>(&self, filename: &str) -> Result<T, String> {
        let path = self.data_dir.join(filename);
        let content =
            fs::read_to_string(&path).map_err(|e| format!("读取 {} 失败: {}", filename, e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("解析 {} 失败: {}", filename, e))
    }

    fn write_json<T: Serialize>(&self, filename: &str, data: &T) -> Result<(), String> {
        let path = self.data_dir.join(filename);
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| format!("序列化 {} 失败: {}", filename, e))?;
        fs::write(&path, content).map_err(|e| format!("写入 {} 失败: {}", filename, e))
    }

    pub fn load_base_data(&self) -> BaseData {
        match self.read_json("factory_basecfg.json") {
            Ok(data) => data,
            Err(_) => {
                let data = Self::embedded_base_data();
                let _ = self.write_json("factory_basecfg.json", &data);
                data
            }
        }
    }

    pub fn save_base_data(&self, data: &BaseData) -> Result<(), String> {
        self.write_json("factory_basecfg.json", data)
    }

    fn embedded_base_data() -> BaseData {
        const JSON: &str = include_str!("../../factory_portable/factory_basecfg.json");
        serde_json::from_str(JSON).unwrap_or_default()
    }

    pub fn load_product_selection(&self, _base_data: &BaseData) -> Result<Product, String> {
        match self.read_json("factory_select.json") {
            Ok(product) => Ok(product),
            Err(_) => {
                // factory_select.json 损坏或缺失时静默回退到第一个品牌——
                // 这会在生产写入 SN 时导致错误的产品关联。
                // 改为返回错误，让前端提示用户重新选择产品。
                Err("factory_select.json 不存在或损坏，请重新选择产品品牌/类型/工厂".to_string())
            }
        }
    }

    pub fn save_product_selection(&self, product: &Product) -> Result<(), String> {
        self.write_json("factory_select.json", product)
    }

    pub fn load_execute_data(&self) -> Result<ExecuteDataList, String> {
        self.read_json("factory_execute.json")
    }

    pub fn save_execute_data(&self, data: &ExecuteDataList) -> Result<(), String> {
        self.write_json("factory_execute.json", data)
    }

    fn csv_escape(s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }

    pub fn append_csv_record(&self, record: &DeviceInfo) -> Result<(), String> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let filename = format!("factory_{}.csv", date);
        let path = self.data_dir.join(&filename);

        let need_header =
            !path.exists() || fs::metadata(&path).map(|m| m.len() == 0).unwrap_or(true);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("打开 {} 失败: {}", filename, e))?;

        if need_header {
            writeln!(file, "日期,IMEI,SN,软件版本,设备名称,激活状态")
                .map_err(|e| format!("写入表头失败 {}: {}", filename, e))?;
        }

        let status = if record.activated { "已激活" } else { "未激活" };

        writeln!(
            file,
            "{},{},{},{},{},{}",
            record.timestamp,
            Self::csv_escape(&record.imei),
            Self::csv_escape(&record.sn),
            Self::csv_escape(&record.sw_version),
            Self::csv_escape(&record.device_name),
            status,
        )
        .map_err(|e| format!("写入记录失败 {}: {}", filename, e))?;

        Ok(())
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn get_factory_state(app: &AppHandle) -> tauri::State<'_, FactoryState> {
    app.state::<FactoryState>()
}

fn save_base_data_and_notify(state: &tauri::State<'_, FactoryState>) -> Result<(), String> {
    let dm = lock_field!(state, data_manager)?;
    let dm = dm.as_ref().ok_or("数据管理器未初始化")?;
    let base = lock_field!(state, base_data)?;
    dm.save_base_data(&base)
}

// ─── Initialization ─────────────────────────────────────────────────────────

#[tauri::command]
pub fn init_factory(app: AppHandle) -> Result<bool, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;

    let data_manager = DataManager::new(data_dir)?;
    let base_data = data_manager.load_base_data();
    let product = data_manager.load_product_selection(&base_data)?;
    let execute_data = data_manager.load_execute_data().ok();

    let mut code_set = CodeSet {
        brand_code: String::new(),
        type_code: String::new(),
        fac_code: String::new(),
        year_code: String::new(),
        mon_code: String::new(),
        seq_code: "00001".to_string(),
    };

    resolve_codes(&base_data, &product, &mut code_set);
    update_code_set(&mut code_set);

    if let Some(ref ex_data) = execute_data {
        let prefix = format!(
            "{}{}{}{}{}",
            code_set.brand_code,
            code_set.type_code,
            code_set.fac_code,
            code_set.year_code,
            code_set.mon_code
        );
        for ex in &ex_data.exe_data_list {
            if ex.prefix_str == prefix {
                // SN 序列解析失败时不再静默重置为 "00001"（危害 SN 唯一性）；
                // 传播错误让前端看到具体原因。
                code_set.seq_code = increment_seq(&ex.current_seq_no)
                    .map_err(|e| format!("递增序列号失败 (prefix={prefix}): {e}"))?;
                break;
            }
        }
    }

    let state = get_factory_state(&app);
    *state.data_manager.lock()
        .map_err(|e| format!("Lock poisoned: {e}"))? = Some(data_manager);
    *state.current_product.lock()
        .map_err(|e| format!("Lock poisoned: {e}"))? = product;
    *state.current_code_set.lock()
        .map_err(|e| format!("Lock poisoned: {e}"))? = code_set;
    *state.execute_data.lock()
        .map_err(|e| format!("Lock poisoned: {e}"))? = execute_data;
    *state.base_data.lock()
        .map_err(|e| format!("Lock poisoned: {e}"))? = base_data;

    Ok(true)
}

// ─── Product & SN commands ─────────────────────────────────────────────────

#[tauri::command]
pub fn factory_get_base_data(state: tauri::State<'_, FactoryState>) -> Result<BaseData, String> {
    Ok(lock_field!(state, base_data)?.clone())
}

#[tauri::command]
pub fn factory_get_current_product(
    state: tauri::State<'_, FactoryState>,
) -> Result<Product, String> {
    Ok(lock_field!(state, current_product)?.clone())
}

#[tauri::command]
pub fn factory_set_product(
    brand: String,
    product_type: String,
    fac: String,
    state: tauri::State<'_, FactoryState>,
) -> Result<String, String> {
    let mut product = lock_field!(state, current_product)?;
    product.brand = brand.clone();
    product.product_type = product_type.clone();
    product.fac = fac.clone();

    let mut code_set = lock_field!(state, current_code_set)?;
    let base = lock_field!(state, base_data)?;

    resolve_codes(&base, &product, &mut code_set);
    update_code_set(&mut code_set);

    if let Some(ref dm) = *lock_field!(state, data_manager)? {
        dm.save_product_selection(&product)?;
    }

    Ok(generate_sn(&code_set))
}

#[tauri::command]
pub fn factory_get_current_sn(state: tauri::State<'_, FactoryState>) -> Result<String, String> {
    Ok(generate_sn(&*lock_field!(state, current_code_set)?))
}

#[tauri::command]
pub fn factory_get_code_set(state: tauri::State<'_, FactoryState>) -> Result<CodeSet, String> {
    Ok(lock_field!(state, current_code_set)?.clone())
}

#[tauri::command]
pub fn factory_increment_sequence(
    state: tauri::State<'_, FactoryState>,
) -> Result<String, String> {
    let mut code_set = lock_field!(state, current_code_set)?;
    code_set.seq_code = increment_seq(&code_set.seq_code)?;
    Ok(generate_sn(&code_set))
}

// ─── Brand / Type / Factory CRUD ────────────────────────────────────────────

macro_rules! crud_commands {
    ($add_name:ident, $remove_name:ident, $field:ident, $label:literal) => {
        #[tauri::command]
        pub fn $add_name(
            name: String,
            code: String,
            state: tauri::State<'_, FactoryState>,
        ) -> Result<BaseData, String> {
            if name.trim().is_empty() || code.trim().is_empty() {
                return Err(format!("{}名称和编码不能为空", $label));
            }
            {
                let mut base = lock_field!(state, base_data)?;
                if base.$field.iter().any(|i| i.name == name) {
                    return Err(format!("{}名称已存在: {}", $label, name));
                }
                if base.$field.iter().any(|i| i.code == code) {
                    return Err(format!("{}编码已存在: {}", $label, code));
                }
                let idx = next_index(&base.$field);
                base.$field.push(Item {
                    name: name.trim().to_string(),
                    index: idx,
                    code: code.trim().to_string(),
                });
            }
            save_base_data_and_notify(&state)?;
            Ok(lock_field!(state, base_data)?.clone())
        }

        #[tauri::command]
        pub fn $remove_name(
            name: String,
            state: tauri::State<'_, FactoryState>,
        ) -> Result<BaseData, String> {
            {
                let mut base = lock_field!(state, base_data)?;
                let before = base.$field.len();
                base.$field.retain(|i| i.name != name);
                if base.$field.len() == before {
                    return Err(format!("{}不存在: {}", $label, name));
                }
            }
            save_base_data_and_notify(&state)?;
            Ok(lock_field!(state, base_data)?.clone())
        }
    };
}

crud_commands!(factory_add_brand, factory_remove_brand, brands, "品牌");
crud_commands!(
    factory_add_product_type,
    factory_remove_product_type,
    product_types,
    "产品类型"
);
crud_commands!(
    factory_add_factory,
    factory_remove_factory,
    factories,
    "工厂"
);

// ─── Device communication ──────────────────────────────────────────────────

#[tauri::command]
pub fn factory_set_device_ip(
    ip: String,
    state: tauri::State<'_, FactoryState>,
) -> Result<(), String> {
    // AGENTS.md: "AT / 认证输入必须显式校验" — 设备 IP 在设置时校验格式。
    // DeviceClient::new 现在会校验 IP 格式并返回 Result，无效 IP 直接报错。
    let mut client_opt = state.device_client.lock()
        .map_err(|e| format!("Lock poisoned: {e}"))?;
    match client_opt.as_mut() {
        Some(client) => client.update_ip(&ip),
        None => *client_opt = Some(DeviceClient::new(&ip)?),
    }
    Ok(())
}

#[tauri::command]
pub async fn factory_write_sn_to_device(
    sn: String,
    state: tauri::State<'_, FactoryState>,
) -> Result<bool, String> {
    let client = {
        let guard = lock_field!(state, device_client)?;
        guard
            .as_ref()
            .ok_or("设备客户端未初始化")?
            .clone()
    };
    client.set_device_sn(&sn).await
}

#[tauri::command]
pub async fn factory_get_device_info(
    state: tauri::State<'_, FactoryState>,
) -> Result<DeviceInfo, String> {
    let client = {
        let guard = lock_field!(state, device_client)?;
        guard
            .as_ref()
            .ok_or("设备客户端未初始化")?
            .clone()
    };
    client.get_device_info().await
}

// ─── Data persistence ──────────────────────────────────────────────────────

#[tauri::command]
pub fn factory_save_execute_data(state: tauri::State<'_, FactoryState>) -> Result<(), String> {
    let mut execute_data = lock_field!(state, execute_data)?;
    let code_set = lock_field!(state, current_code_set)?;
    let product = lock_field!(state, current_product)?;

    let prefix = format!(
        "{}{}{}{}{}",
        code_set.brand_code,
        code_set.type_code,
        code_set.fac_code,
        code_set.year_code,
        code_set.mon_code
    );

    if execute_data.is_none() {
        *execute_data = Some(ExecuteDataList {
            exe_data_list: Vec::new(),
        });
    }

    if let Some(ref mut ex_data) = *execute_data {
        let mut found = false;
        for ex in &mut ex_data.exe_data_list {
            if ex.prefix_str == prefix {
                ex.date_str = chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string();
                ex.current_seq_no = code_set.seq_code.clone();
                ex.product_type = product.product_type.clone();
                found = true;
                break;
            }
        }
        if !found {
            ex_data.exe_data_list.push(ExecuteData {
                date_str: chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                product_type: product.product_type.clone(),
                prefix_str: prefix,
                current_seq_no: code_set.seq_code.clone(),
            });
        }
    }

    if let Some(ref dm) = *lock_field!(state, data_manager)? {
        dm.save_execute_data(
            execute_data
                .as_ref()
                .ok_or("execute data 未初始化")?,
        )?;
    }

    Ok(())
}

#[tauri::command]
pub fn factory_save_device_record(
    device_info: DeviceInfo,
    state: tauri::State<'_, FactoryState>,
) -> Result<(), String> {
    let dm = lock_field!(state, data_manager)?;
    let dm = dm.as_ref().ok_or("数据管理器未初始化")?;
    dm.append_csv_record(&device_info)
}
