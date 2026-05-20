use crate::transport::AtTransport;
use crate::types::*;

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
    fn query_hardware_info(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<HardwareInfo, String>;

    /// Query temperature information
    fn query_temperature(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<TemperatureInfo, String>;

    // ==================== Network Information ====================

    /// Query serving cell information
    fn query_serving_cell(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<ServingCellInfo, String>;

    /// Query neighbor cells
    fn query_neighbor_cells(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<NeighborCells, String>;

    /// Query signal strength (RSRP, RSRQ, SINR)
    fn query_signal_strength(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<SignalInfo, String>;

    /// Query operator name
    fn query_operator(&mut self, transport: &mut dyn AtTransport) -> Result<String, String>;

    /// Query network registration status
    fn query_registration_status(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<String, String>;

    /// Query connection status (PDP context active)
    fn query_connection_status(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<String, String>;

    // ==================== APN and Data ====================

    /// Query APN list with active status
    fn query_apn_list(&mut self, transport: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String>;

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
    fn delete_apn(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String>;

    /// Activate or deactivate APN context (AT+CGACT=<state>,<cid>)
    fn set_apn_active(&mut self, transport: &mut dyn AtTransport, cid: i32, active: bool) -> Result<(), String>;

    /// Query 5GLAN status (AT+QCFG="5glan")
    fn query_5glan(&mut self, transport: &mut dyn AtTransport) -> Result<Vec<L5GanEntry>, String>;

    /// Set 5GLAN state for a CID (AT+QCFG="5glan",<cid>,<state>)
    fn set_5glan(&mut self, transport: &mut dyn AtTransport, cid: i32, enabled: bool) -> Result<(), String>;

    /// Activate data connection
    fn connect_data(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String>;

    /// Deactivate data connection
    fn disconnect_data(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<(), String>;

    /// Query IP information for a CID
    fn query_ip_info(
        &mut self,
        transport: &mut dyn AtTransport,
        cid: i32,
    ) -> Result<IpInfo, String>;

    // ==================== Band Configuration ====================

    /// Query supported and locked bands
    fn query_band_config(&mut self, transport: &mut dyn AtTransport) -> Result<BandConfig, String>;

    /// Set LTE bands
    fn set_lte_bands(&mut self, transport: &mut dyn AtTransport, bands: &str)
        -> Result<(), String>;

    /// Set 5G SA bands
    fn set_nr5g_bands(
        &mut self,
        transport: &mut dyn AtTransport,
        bands: &str,
    ) -> Result<(), String>;

    /// Set 5G NSA bands (if supported)
    fn set_nsa_nr5g_bands(
        &mut self,
        _transport: &mut dyn AtTransport,
        _bands: &str,
    ) -> Result<(), String> {
        Err("NSA NR5G bands not supported".to_string())
    }

    /// Set network mode preference (e.g., "LTE", "NR5G", "LTE:NR5G")
    fn set_network_mode(
        &mut self,
        transport: &mut dyn AtTransport,
        mode: &str,
    ) -> Result<(), String>;

    // ==================== Traffic Statistics ====================

    /// Query data usage statistics
    fn query_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<TrafficInfo, String>;

    /// Reset traffic counters
    fn reset_traffic(&mut self, transport: &mut dyn AtTransport) -> Result<(), String>;

    // ==================== Feature Toggles ====================

    /// Query feature toggles
    fn query_feature_toggles(
        &mut self,
        transport: &mut dyn AtTransport,
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
    fn set_cfun(&mut self, transport: &mut dyn AtTransport, mode: i32) -> Result<(), String>;

    // ==================== QoS ====================

    /// Query QoS information
    fn query_qos(&mut self, transport: &mut dyn AtTransport, cid: i32) -> Result<QosInfo, String>;

    // ==================== Baseline ====================

    /// Query AP/CP baseline version
    fn query_baseline(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<BaselineInfo, String> {
        Err("Not implemented".to_string())
    }

    // ==================== Antenna ====================

    /// Query antenna RSSI values (AT+QANTRSSI?)
    fn query_ant_rssi(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    // ==================== Network Mode ====================

    /// Query preferred network mode (e.g., AT+QNWPREFCFG="mode_pref")
    fn query_network_mode(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<String, String> {
        Err("Not implemented".to_string())
    }

    // ==================== Band Configuration (extended) ====================

    /// Query full band configuration with supported, locked, and spec bands
    fn query_bands_with_spec(
        &mut self,
        transport: &mut dyn AtTransport,
    ) -> Result<BandConfig, String> {
        self.query_band_config(transport)
    }

    /// Set LTE and NR bands (combined operation)
    fn set_bands(
        &mut self,
        transport: &mut dyn AtTransport,
        lte: &str,
        nr: &str,
    ) -> Result<(), String> {
        if !lte.is_empty() {
            self.set_lte_bands(transport, lte)?;
        }
        if !nr.is_empty() {
            self.set_nr5g_bands(transport, nr)?;
        }
        Ok(())
    }

    /// Reset all bands to factory defaults
    fn reset_all_bands(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<(), String> {
        Err("Not implemented".to_string())
    }

    // ==================== USB Configuration ====================

    /// Query USB network mode
    fn query_usbnet_mode(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<i32, String> {
        Err("Not implemented".to_string())
    }

    /// Set USB network mode
/// Set USB network mode
    fn set_usbnet_mode(
        &mut self,
        _transport: &mut dyn AtTransport,
        _mode: i32,
    ) -> Result<(), String> {
        Err("Not implemented".to_string())
    }

    // ==================== Factory Reset ====================

    /// Factory reset the modem
    fn factory_reset(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<(), String> {
        Err("Not implemented".to_string())
    }

    // ==================== SIM Slot ====================

    /// Query current SIM slot number (1-based)
    fn query_sim_slot(
        &mut self,
        _transport: &mut dyn AtTransport,
    ) -> Result<i32, String> {
        Ok(1)
    }

    /// Switch to a different SIM slot
    fn switch_sim_slot(
        &mut self,
        _transport: &mut dyn AtTransport,
        _slot: i32,
    ) -> Result<(), String> {
        Err("SIM slot switching not supported".to_string())
    }

    // ==================== Raw AT ====================

    /// Send a raw AT command and return the response
    fn send_raw_at(
        &mut self,
        transport: &mut dyn AtTransport,
        command: &str,
    ) -> Result<String, String> {
        transport.send_at(command)
    }

    // ==================== Combined Operations ====================

    /// Query full modem status (combines multiple queries)
    fn query_modem_status(
        &mut self,
        transport: &mut dyn AtTransport,
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
            rx_level: serving_cell.rx_level,
            ant_values: signal.ant_values,
            scs: serving_cell.scs,
        })
    }
}
