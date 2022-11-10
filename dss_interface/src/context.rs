extern crate phf;
extern crate slog;

use dss_openapi::apis::configuration::ApiKey;
use slog::b;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard};

use crate::messages::Messages;

pub struct Context {
    pub log: slog::Logger,
    configuration: Arc<RwLock<dss_openapi::apis::configuration::Configuration>>,
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
                z.1,
                numeric_messageid,
                crate::messages::ERRORMESSAGES[&numeric_messageid]
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
