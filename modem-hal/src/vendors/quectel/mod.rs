use crate::types::*;
use crate::transport::AtTransport;
use crate::modem_vendor::ModemVendor;

pub mod parser;
pub mod qualcomm;
pub mod unisoc;

pub enum QuectelChip { Qualcomm, UniSoc }

pub struct QuectelModem {
    pub chip: QuectelChip,
    pub model: String,
}

impl QuectelModem {
    pub fn qualcomm(model: String) -> Self { Self { chip: QuectelChip::Qualcomm, model } }
    pub fn unisoc(model: String) -> Self { Self { chip: QuectelChip::UniSoc, model } }
}

impl ModemVendor for QuectelModem {
    fn vendor(&self) -> ChipsetVendor {
        match self.chip {
            QuectelChip::Qualcomm => ChipsetVendor::Qualcomm,
            QuectelChip::UniSoc => ChipsetVendor::UniSoc,
        }
    }
    fn model(&self) -> &str { &self.model }
    fn query_sim_status(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_imei(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_iccid(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_hardware_info(&mut self, _t: &mut dyn AtTransport) -> Result<HardwareInfo, String> { todo!() }
    fn query_temperature(&mut self, _t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> { todo!() }
    fn query_serving_cell(&mut self, _t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> { todo!() }
    fn query_neighbor_cells(&mut self, _t: &mut dyn AtTransport) -> Result<NeighborCells, String> { todo!() }
    fn query_signal_strength(&mut self, _t: &mut dyn AtTransport) -> Result<SignalInfo, String> { todo!() }
    fn query_operator(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_registration_status(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_connection_status(&mut self, _t: &mut dyn AtTransport) -> Result<String, String> { todo!() }
    fn query_apn_list(&mut self, _t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> { todo!() }
    fn set_apn(&mut self, _t: &mut dyn AtTransport, _cid: i32, _ctx: i32, _apn: &str, _user: &str, _pass: &str, _auth: i32) -> Result<(), String> { todo!() }
    fn delete_apn(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<(), String> { todo!() }
    fn connect_data(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<(), String> { todo!() }
    fn disconnect_data(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<(), String> { todo!() }
    fn query_ip_info(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<IpInfo, String> { todo!() }
    fn query_band_config(&mut self, _t: &mut dyn AtTransport) -> Result<BandConfig, String> { todo!() }
    fn set_lte_bands(&mut self, _t: &mut dyn AtTransport, _bands: &str) -> Result<(), String> { todo!() }
    fn set_nr5g_bands(&mut self, _t: &mut dyn AtTransport, _bands: &str) -> Result<(), String> { todo!() }
    fn set_network_mode(&mut self, _t: &mut dyn AtTransport, _mode: &str) -> Result<(), String> { todo!() }
    fn query_traffic(&mut self, _t: &mut dyn AtTransport) -> Result<TrafficInfo, String> { todo!() }
    fn reset_traffic(&mut self, _t: &mut dyn AtTransport) -> Result<(), String> { todo!() }
    fn query_feature_toggles(&mut self, _t: &mut dyn AtTransport) -> Result<FeatureToggles, String> { todo!() }
    fn set_feature_toggle(&mut self, _t: &mut dyn AtTransport, _feat: &str, _on: bool) -> Result<(), String> { todo!() }
    fn reboot(&mut self, _t: &mut dyn AtTransport) -> Result<(), String> { todo!() }
    fn set_cfun(&mut self, _t: &mut dyn AtTransport, _mode: i32) -> Result<(), String> { todo!() }
    fn query_qos(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<QosInfo, String> { todo!() }
}
