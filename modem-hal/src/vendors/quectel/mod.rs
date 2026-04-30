use crate::modem_vendor::ModemVendor;
use crate::transport::AtTransport;
use crate::types::*;
use parser::*;

pub mod parser;
pub mod qualcomm;
pub mod unisoc;

fn cmd_delay() {
    std::thread::sleep(std::time::Duration::from_millis(5));
}

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

    fn query_sim_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CPIN?")?;
        Ok(parse_cpin(&resp))
    }

    fn query_imei(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CGSN")?;
        Ok(parse_cgsn(&resp))
    }

    fn query_iccid(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let cmd = match self.chip {
            QuectelChip::Qualcomm => "AT+ICCID",
            QuectelChip::UniSoc => "AT+CCID",
        };
        let resp = t.send_at(cmd)?;
        let iccid = parse_iccid(&resp);
        if !iccid.is_empty() { return Ok(iccid); }
        cmd_delay();
        let resp2 = t.send_at("AT+QCCID")?;
        Ok(parse_iccid(&resp2))
    }

    fn query_operator(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+COPS?")?;
        Ok(parse_cops(&resp))
    }

    fn query_registration_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CEREG?")?;
        for line in extract_data_lines(&resp) {
            if let Some(rest) = line.strip_prefix("+CEREG:") {
                let parts: Vec<&str> = rest.trim().split(',').collect();
                let stat = parts.get(1).unwrap_or(&parts.get(0).unwrap_or(&"0")).trim();
                return Ok(stat.to_string());
            }
        }
        Ok("0".to_string())
    }

    fn query_connection_status(&mut self, t: &mut dyn AtTransport) -> Result<String, String> {
        let resp = t.send_at("AT+CGACT?")?;
        for line in extract_data_lines(&resp) {
            if let Some(rest) = line.strip_prefix("+CGACT:") {
                let parts: Vec<&str> = rest.trim().split(',').collect();
                if parts.get(1).map(|s| s.trim()) == Some("1") {
                    return Ok("1".to_string());
                }
            }
        }
        Ok("0".to_string())
    }

    fn query_serving_cell(&mut self, t: &mut dyn AtTransport) -> Result<ServingCellInfo, String> {
        let resp = t.send_at("AT+QENG=\"servingcell\"")?;
        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        Ok(parse_qeng_serving_cell(&resp, qualcomm_bw))
    }

    fn query_signal_strength(&mut self, t: &mut dyn AtTransport) -> Result<SignalInfo, String> {
        let resp = t.send_at("AT+QENG=\"servingcell\"")?;
        let qualcomm_bw = matches!(self.chip, QuectelChip::Qualcomm);
        let cell = parse_qeng_serving_cell(&resp, qualcomm_bw);
        Ok(SignalInfo {
            rsrp: cell.rsrp,
            rsrq: cell.rsrq,
            sinr: cell.sinr,
            ant_values: Default::default(),
        })
    }

    fn query_neighbor_cells(&mut self, t: &mut dyn AtTransport) -> Result<NeighborCells, String> {
        let resp = t.send_at("AT+QENG=\"neighbourcell\"")?;
        Ok(parse_qeng_neighbour_cells(&resp))
    }

    fn reboot(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT+CFUN=1,1")?;
        Ok(())
    }

    fn set_cfun(&mut self, t: &mut dyn AtTransport, mode: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CFUN={}", mode))?;
        Ok(())
    }

    fn set_lte_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"lte_band\",{}", bands))?;
        Ok(())
    }

    fn set_nr5g_bands(&mut self, t: &mut dyn AtTransport, bands: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"nr5g_band\",{}", bands))?;
        Ok(())
    }

    fn set_network_mode(&mut self, t: &mut dyn AtTransport, mode: &str) -> Result<(), String> {
        t.send_at(&format!("AT+QNWPREFCFG=\"mode_pref\",{}", mode))?;
        Ok(())
    }

    fn query_hardware_info(&mut self, t: &mut dyn AtTransport) -> Result<HardwareInfo, String> {
        let model = parse_cgmm(&t.send_at("AT+CGMM")?);
        cmd_delay();
        let manufacturer = { let r = t.send_at("AT+CGMI")?; parse_cgmm(&r) };
        cmd_delay();
        let firmware = { let r = t.send_at("AT+GMR")?; parse_cgmm(&r) };
        Ok(HardwareInfo {
            model, manufacturer, firmware,
            ap_baseline: String::new(),
            cp_baseline: String::new(),
            soc_temp: String::new(),
            pa_temp: String::new(),
        })
    }

    fn query_temperature(&mut self, t: &mut dyn AtTransport) -> Result<TemperatureInfo, String> {
        match self.chip {
            QuectelChip::UniSoc => {
                let resp = t.send_at("AT+QTEMP?")?;
                Ok(parse_qtemp(&resp))
            }
            QuectelChip::Qualcomm => Ok(TemperatureInfo::default()),
        }
    }

    fn query_apn_list(&mut self, t: &mut dyn AtTransport) -> Result<Vec<ApnEntry>, String> {
        let cgdcont = t.send_at("AT+CGDCONT?")?;
        cmd_delay();
        let cgact = t.send_at("AT+CGACT?")?;
        let active_cids = parse_cgact_cids(&cgact);
        Ok(parse_cgdcont_apn(&cgdcont, &active_cids))
    }

    fn set_apn(&mut self, t: &mut dyn AtTransport, cid: i32, ctx: i32, apn: &str, user: &str, pass: &str, auth: i32) -> Result<(), String> {
        let pdp_type = match ctx { 2 => "IPV6", 3 => "IPV4V6", _ => "IP" };
        t.send_at(&format!("AT+CGDCONT={},\"{}\",\"{}\"", cid, pdp_type, apn))?;
        if !user.is_empty() {
            t.send_at(&format!("AT+QICSGP={},1,\"{}\",\"{}\",\"{}\",{}", cid, apn, user, pass, auth))?;
        }
        Ok(())
    }

    fn delete_apn(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        t.send_at(&format!("AT+CGDCONT={}", cid))?;
        Ok(())
    }

    fn connect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::connect_data(t, cid),
            QuectelChip::UniSoc => unisoc::connect_data(t, cid),
        }
    }

    fn disconnect_data(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<(), String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::disconnect_data(t, cid),
            QuectelChip::UniSoc => unisoc::disconnect_data(t, cid),
        }
    }

    fn query_ip_info(&mut self, t: &mut dyn AtTransport, cid: i32) -> Result<IpInfo, String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::query_ip_info(t, cid),
            QuectelChip::UniSoc => unisoc::query_ip_info(t, cid),
        }
    }

    fn query_band_config(&mut self, t: &mut dyn AtTransport) -> Result<BandConfig, String> {
        let lte_resp = t.send_at("AT+QNWPREFCFG=\"lte_band\"")?;
        cmd_delay();
        let nr_resp = t.send_at("AT+QNWPREFCFG=\"nr5g_band\"")?;
        Ok(BandConfig {
            lte_supported: parse_band_list(&lte_resp),
            nr_supported: parse_band_list(&nr_resp),
            lte_locked: vec![],
            nr_locked: vec![],
        })
    }

    fn query_traffic(&mut self, t: &mut dyn AtTransport) -> Result<TrafficInfo, String> {
        match self.chip {
            QuectelChip::Qualcomm => qualcomm::query_traffic(t),
            QuectelChip::UniSoc => unisoc::query_traffic(t),
        }
    }

    fn reset_traffic(&mut self, t: &mut dyn AtTransport) -> Result<(), String> {
        t.send_at("AT+QGDCNT=0")?;
        Ok(())
    }

    fn query_feature_toggles(&mut self, _t: &mut dyn AtTransport) -> Result<FeatureToggles, String> {
        Ok(FeatureToggles::default())
    }

    fn set_feature_toggle(&mut self, _t: &mut dyn AtTransport, _feat: &str, _on: bool) -> Result<(), String> {
        Ok(())
    }

    fn query_qos(&mut self, _t: &mut dyn AtTransport, _cid: i32) -> Result<QosInfo, String> {
        Ok(QosInfo { cqi: String::new(), ul_bandwidth: String::new(), dl_bandwidth: String::new() })
    }
}
