extern crate phf;
extern crate slog;

use dss_openapi::apis::configuration::ApiKey;
use phf::phf_map;
use slog::b;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard};

pub struct Context {
    pub log: slog::Logger,
    configuration: Arc<RwLock<dss_openapi::apis::configuration::Configuration>>,
}

impl Context {
    pub fn new(
        log: slog::Logger,
        configuration: dss_openapi::apis::configuration::Configuration,
    ) -> Context {
        Context {
            log,
            configuration: Arc::new(RwLock::new(configuration)),
        }
    }

    pub fn logevent(&self, messageid: Messages, kv: slog::BorrowedKV) {
        let numeric_messageid = messageid as i32;
        let z = match numeric_messageid / 1000 {
            1 => (slog::Level::Error, 'E'),
            2 => (slog::Level::Warning, 'I'),
            3 => (slog::Level::Info, 'I'),
            4 => (slog::Level::Debug, 'N'),
            _ => (slog::Level::Critical, 'C'),
        };
        let recs = slog::record_static!(z.0, "");
        self.log.log(&slog::Record::new(
            &recs,
            &format_args!(
                "{}{}: {}",
                z.1, numeric_messageid, ERRORMESSAGES[&numeric_messageid]
            ),
            b!(CombinedBorrowedKV {
                k1: &kv,
                k2: &b!("messageid"=>numeric_messageid ),
            }),
        ));
    }

    pub fn configuration(
        &self,
    ) -> LockResult<RwLockReadGuard<'_, dss_openapi::apis::configuration::Configuration>> {
        self.configuration.read()
    }
    pub fn set_api_key(&self, v: Option<ApiKey>) {
        let cguard = self.configuration.write();
        cguard.unwrap().api_key = v;
    }

    pub fn log(&self) -> &slog::Logger {
        &self.log
    }
}
struct CombinedBorrowedKV<'a> {
    k1: &'a slog::BorrowedKV<'a>,
    k2: &'a slog::BorrowedKV<'a>,
}

impl slog::KV for CombinedBorrowedKV<'_> {
    fn serialize(
        &self,
        record: &slog::Record,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        self.k1.serialize(record, serializer)?;
        self.k2.serialize(record, serializer)?;
        Ok(())
    }
}

pub enum Messages {
    /// Request to dSS Server returned HTTP Status 200, but 'ok' indicator was 'false'
    E1000 = 1000,
    /// Request to dSS Server failed
    E1001 = 1001,
    /// Startupmessage
    I3000 = 3000,
    /// Applicaton token successfully retrieved
    I3001 = 3001,
}
static ERRORMESSAGES: phf::Map<i32, &'static str> = phf_map! {
    1000i32 => "Request to dSS Server returned HTTP Status 200, but 'ok' indicator was 'false'",
    1001i32 => "Request to dSS Server failed",
    3000i32 => concat!(env!("CARGO_PKG_NAME")," ", env!("CARGO_PKG_VERSION"), " - startup"),
    3001i32 => "Applicaton token successfully retrieved",
};
