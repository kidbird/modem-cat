pub mod modem_vendor;
pub mod modem_factory;
pub mod transport;
pub mod types;
pub mod vendors;

pub use modem_vendor::ModemVendor;
pub use modem_factory::ModemFactory;
pub use types::*;

// ── napi-rs surface for Bun/TS ──
#[cfg(feature = "napi-feature")]
mod napi_exports {
    use napi_derive::napi;
    use crate::transport::SerialTransport;
    use crate::ModemFactory;

    #[napi]
    pub struct ModemHandle {
        inner: Box<dyn crate::ModemVendor + Send>,
        transport: SerialTransport,
    }

    #[napi]
    impl ModemHandle {
        #[napi(factory)]
        pub fn connect(port: String, baud: u32) -> napi::Result<Self> {
            let mut transport = SerialTransport::new(&port, baud)
                .map_err(|e| napi::Error::from_reason(e))?;
            let modem = ModemFactory::create(&mut transport)
                .map_err(|e| napi::Error::from_reason(e))?;
            Ok(Self { inner: modem, transport })
        }

        #[napi]
        pub fn query_signal(&mut self) -> napi::Result<crate::types::SignalInfo> {
            self.inner.query_signal_strength(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn query_status(&mut self) -> napi::Result<crate::types::ModemStatus> {
            self.inner.query_modem_status(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn connect_data(&mut self, cid: i32) -> napi::Result<()> {
            self.inner.connect_data(&mut self.transport, cid)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn disconnect_data(&mut self, cid: i32) -> napi::Result<()> {
            self.inner.disconnect_data(&mut self.transport, cid)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn reboot(&mut self) -> napi::Result<()> {
            self.inner.reboot(&mut self.transport)
                .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn close(&mut self) {
            use crate::transport::AtTransport;
            self.transport.close();
        }
    }
}
