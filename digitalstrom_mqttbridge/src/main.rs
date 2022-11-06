mod commandline;

extern crate dss_interface;
extern crate dss_openapi;
extern crate reqwest;

extern crate phf;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use phf::phf_map;
use slog::{b, error, info, o, Drain};

use crate::commandline::{Cli, Messageformat};
use clap::Parser;

struct Context {
    log: slog::Logger,
    configuration: dss_openapi::apis::configuration::Configuration,
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
    fn logevent(&self, messageid: i32, kv: slog::BorrowedKV) {
        let z = match messageid / 1000 {
            1 => (slog::Level::Error, 'E'),
            2 => (slog::Level::Warning, 'I'),
            3 => (slog::Level::Info, 'I'),
            4 => (slog::Level::Debug, 'N'),
            _ => (slog::Level::Critical, 'C'),
        };
        let recs = slog::record_static!(z.0, "");
        self.log.log(&slog::Record::new(
            &recs,
            &format_args!("{}{}: {}", z.1, messageid, ERRORMESSAGES[&messageid]),
            b!(CombinedBorrowedKV {
                k1: &kv,
                k2: &b!("messageid"=>messageid),
            }),
        ));
    }
}
//enum MyEnum {
//    /// Startupmessage
//    I3000 = 3000,
//    B = 456,
//}
static ERRORMESSAGES: phf::Map<i32, &'static str> = phf_map! {
    3000i32 => concat!(env!("CARGO_PKG_NAME")," ", env!("CARGO_PKG_VERSION"), " - startup"),
    2i32 => "apple",
    3i32 => "apple",
    4i32 => "request succeeded, but outcome was not ok",
};
#[tokio::main]
async fn main() {
    let cargo_pkg_name = env!("CARGO_PKG_NAME");
    let cargo_pkg_version_major = env!("CARGO_PKG_VERSION_MAJOR");
    let cargo_pkg_version_minor = env!("CARGO_PKG_VERSION_MINOR");
    let cargo_pkg_version_patch = env!("CARGO_PKG_VERSION_PATCH");
    let cargo_pkg_version_pre = option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("");

    let cli = Cli::parse();

    let d: Option<slog::Logger>;
    match cli.message_format {
        Messageformat::Json => {
            let drain = slog_json::Json::default(std::io::stderr()).fuse();
            let drain = slog_async::Async::new(drain).build().fuse();
            let _log = slog::Logger::root(drain, o!("application" => cargo_pkg_name));
            d = Some(_log);
        }
        Messageformat::Simple => {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator)
                .use_file_location()
                .build()
                .fuse();
            //let drain = slog_json::Json::default(std::io::stderr()).fuse();
            let drain = slog_async::Async::new(drain).build().fuse();
            let _log = slog::Logger::root(drain, o!("application" => cargo_pkg_name));
            d = Some(_log);
        }
    }

    //let time_now = chrono::Utc::now();

    let dss_client = crate::reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build();
    let dss_client = dss_client.unwrap();
    //let dss_client = dss_client.unwraps(&d.unwrap());

    let context = Context {
        log: d.unwrap(),
        configuration: dss_openapi::apis::configuration::Configuration {
            client: dss_client,
            base_path: cli.url,
            ..dss_openapi::apis::configuration::Configuration::default()
        },
    };

    context.logevent(
        3000,
        b!(
            "version_major"=>cargo_pkg_version_major,
            "version_minor"=>cargo_pkg_version_minor,
            "version_patch"=>cargo_pkg_version_patch,
            "version_pre"=>cargo_pkg_version_pre,
        ),
    );

    match &cli.command {
        crate::commandline::Commands::Run {
            mqttserver: _,
            application_token: _,
        } => {}
        crate::commandline::Commands::RequestApplicationToken { application_name } => {
            let result = dss_openapi::apis::authentication_api::request_application_token(
                &context.configuration,
                application_name,
            )
            .await;
            match result {
                Ok(response) => {
                    if !response.ok {
                        context.logevent(
                            4,
                            b!(
                                "api" => "request_application_token",
                                "message" => response.message.unwrap_or(String::from("<unknown>")
                            )
                        ));
                        //let recs = slog::record_static!(ERRORMESSAGES[&4].0, "");
                        //context.log.log(&slog::Record::new(
                        //    &recs,
                        //    &format_args!("{}", ERRORMESSAGES[&4].1),
                        //    b!("api" => "request_application_token","message" => response.message.unwrap_or(String::from("<unknown>")),"eventid"=>4),
                        //));
                    } else {
                        let application_token = response.result.unwrap().application_token;
                        info!(
                            context.log,
                            "retrieved token '{application_token}'",;
                            "api" => "request_application_token",
                            "eventid" => 3,"application_token"=>application_token.clone()
                        );
                    }
                }
                Err(error) => {
                    error!(
                        context.log,
                        "request_application_token request failed"
                        ;
                        "eventid" => 2,
                        "error"=>%error
                    );
                }
            }
        }
    }

    //let result = dss_openapi::apis::system_api::get_dsid(&context.configuration)
    //    .await
    //    .unwrap();
    //println!("Response: {:?}", result.ok.ok_or(false));
}
