use crate::types::*;
use crate::transport::AtTransport;

/// Trait for modem vendor adapters
/// 
/// This trait abstracts the differences between different modem chipsets/vendors.
/// Each vendor implementation provides its own AT command sequences and parsers
/// while exposing a unified interface for common operations.
pub trait ModemVendor: Send {
    /// Get the vendor/chipset type
    fn vendor(&self) -> ChipsetVendor;
    
    /// Get the model name (e.g., "RG200U", "RM520N")
    fn model(&self) -> &str;
    
    // ==================== Basic Information ====================
    
    /// Query SIM card status
    fn query_sim_status(&mut self, transport: &mut dyn AtTransport) -> Result<String, String>;
    
    /// Query IMEI
    fn query_imei(&mut self, transport: &mut dyn AtTransport) -> Result<String, String>;
    
    /// Query ICCID
    fn query_iccid(&mut self, transport: &mut dyn AtTransport) -> Result<String, String>;
    
    /// Query hardware information (model, manufacturer, firmware)
    fn query_hardware_info(&mut self, transport: &mut dyn AtTransport
    ) -> Result<HardwareInfo, String>;
    
    /// Query temperature information
    fn query_temperature(&mut self, transport: &mut dyn AtTransport
    ) -> Result<TemperatureInfo, String>;
    
    // ==================== Network Information ====================
    
    /// Query serving cell information
    fn query_serving_cell(&mut self, transport: &mut dyn AtTransport
    ) -> Result<ServingCellInfo, String>;
    
    /// Query neighbor cells
    fn query_neighbor_cells(&mut self, transport: &mut dyn AtTransport
    ) -> Result<NeighborCells, String>;
    
    /// Query signal strength (RSRP, RSRQ, SINR)
    fn query_signal_strength(&mut self, transport: &mut dyn AtTransport
    ) -> Result<SignalInfo, String>;
    
    /// Query operator name
    fn query_operator(&mut self, transport: &mut dyn AtTransport) -> Result<String, String>;
    
    /// Query network registration status
    fn query_registration_status(&mut self, transport: &mut dyn AtTransport
    ) -> Result<String, String>;
    
    /// Query connection status (PDP context active)
    fn query_connection_status(&mut self, transport: &mut dyn AtTransport
    ) -> Result<String, String>;
    
    // ==================== APN and Data ====================
    
    /// Query APN list with active status
    fn query_apn_list(&mut self, transport: &mut dyn AtTransport
    ) -> Result<Vec<ApnEntry>, String>;
    
    /// Set APN configuration
    fn set_apn(
        &mut self,
        transport: &mut dyn AtTransport,
        cid: i32,
        context_type: i32,
        apn: &str,
        username: &str,
        password: &str,
        auth_type: i32,
    ) -> Result<(), String>;
    
    /// Delete APN
    fn delete_apn(&mut self, transport: &mut dyn AtTransport, cid: i32
    ) -> Result<(), String>;
    
    /// Activate data connection
    fn connect_data(
        &mut self, 
        transport: &mut dyn AtTransport, 
        cid: i32
    ) -> Result<(), String>;
    
    /// Deactivate data connection
    fn disconnect_data(
        &mut self, 
        transport: &mut dyn AtTransport, 
        cid: i32
    ) -> Result<(), String>;
    
    /// Query IP information for a CID
    fn query_ip_info(
        &mut self, 
        transport: &mut dyn AtTransport, 
        cid: i32
    ) -> Result<IpInfo, String>;
    
    // ==================== Band Configuration ====================
    
    /// Query supported and locked bands
    fn query_band_config(&mut self, transport: &mut dyn AtTransport
    ) -> Result<BandConfig, String>;
    
    /// Set LTE bands
    fn set_lte_bands(
        &mut self, 
        transport: &mut dyn AtTransport, 
        bands: &str
    ) -> Result<(), String>;
    
    /// Set 5G SA bands
    fn set_nr5g_bands(
        &mut self, 
        transport: &mut dyn AtTransport, 
        bands: &str
    ) -> Result<(), String>;
    
    /// Set 5G NSA bands (if supported)
    fn set_nsa_nr5g_bands(
        &mut self, 
        _transport: &mut dyn AtTransport, 
        _bands: &str
    ) -> Result<(), String> {
        Err("NSA NR5G bands not supported".to_string())
    }
    
    /// Set network mode preference (e.g., "LTE", "NR5G", "LTE:NR5G")
    fn set_network_mode(
        &mut self, 
        transport: &mut dyn AtTransport, 
        mode: &str
    ) -> Result<(), String>;
    
    // ==================== Traffic Statistics ====================
    
    /// Query data usage statistics
    fn query_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<TrafficInfo, String>;
    
    /// Reset traffic counters
    fn reset_traffic(&mut self, transport: &mut dyn AtTransport
    ) -> Result<(), String>;
    
    // ==================== Feature Toggles ====================
    
    /// Query feature toggles
    fn query_feature_toggles(
        &mut self, 
        transport: &mut dyn AtTransport
    ) -> Result<FeatureToggles, String>;
    
    /// Set feature toggle
    fn set_feature_toggle(
        &mut self,
        transport: &mut dyn AtTransport,
        feature: &str,
        enabled: bool,
    ) -> Result<(), String>;
    
    // ==================== Power Management ====================
    
    /// Reboot the module via AT+CFUN=1,1
    fn reboot(&mut self, transport: &mut dyn AtTransport) -> Result<(), String>;
    
    /// Set functionality mode (AT+CFUN)
    fn set_cfun(
        &mut self, 
        transport: &mut dyn AtTransport, 
        mode: i32
    ) -> Result<(), String>;
    
    // ==================== QoS ====================
    
    /// Query QoS information
    fn query_qos(
        &mut self, 
        transport: &mut dyn AtTransport, 
        cid: i32
    ) -> Result<QosInfo, String>;
    
    // ==================== Combined Operations ====================
    
    /// Query full modem status (combines multiple queries)
    fn query_modem_status(
        &mut self, 
        transport: &mut dyn AtTransport
    ) -> Result<ModemStatus, String> {
        // Default implementation that combines individual queries
        let sim_status = self.query_sim_status(transport)?;
        let imei = self.query_imei(transport)?;
        let iccid = self.query_iccid(transport).unwrap_or_default();
        let operator = self.query_operator(transport).unwrap_or_default();
        let reg_status = self.query_registration_status(transport)?;
        let conn_status = self.query_connection_status(transport)?;
        let serving_cell = self.query_serving_cell(transport).unwrap_or_default();
        let signal = self.query_signal_strength(transport).unwrap_or_default();
        
        Ok(ModemStatus {
            sim_status,
            reg_status,
            conn_status,
            imei,
            iccid,
            operator,
            network_type: serving_cell.tech,
            pci: serving_cell.pci,
            cell_id: serving_cell.cell_id,
            arfcn: serving_cell.arfcn,
            bandwidth: serving_cell.bandwidth,
            rsrp: signal.rsrp,
            rsrq: signal.rsrq,
            sinr: signal.sinr,
            tx_power: serving_cell.tx_power,
            ant_values: signal.ant_values,
            scs: serving_cell.scs,
        })
    }
}

/// Helper trait for implementing common delay between AT commands
pub trait AtCommandHelper {
    fn cmd_delay();
}

impl AtCommandHelper for dyn ModemVendor {
    fn cmd_delay() {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
