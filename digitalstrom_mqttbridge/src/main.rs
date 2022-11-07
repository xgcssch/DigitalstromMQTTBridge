mod commandline;
mod shutdown;

extern crate dss_interface;
extern crate dss_openapi;
extern crate reqwest;

extern crate phf;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use crate::commandline::{Cli, Messageformat};
use clap::Parser;
use phf::phf_map;
use rumqttc::{AsyncClient, Event, MqttOptions, Outgoing, Packet, QoS};
use slog::{b, o, Drain};
use tokio::{signal, sync::broadcast, time::Duration};

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
    fn logevent(&self, messageid: Messages, kv: slog::BorrowedKV) {
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
        Messages::I3000,
        b!(
            "version_major"=>cargo_pkg_version_major,
            "version_minor"=>cargo_pkg_version_minor,
            "version_patch"=>cargo_pkg_version_patch,
            "version_pre"=>cargo_pkg_version_pre,
        ),
    );

    match &cli.command {
        crate::commandline::Commands::Run {
            mqttserver,
            application_token: _,
        } => {
            // ****************************************
            // ****************************************

            let mut mqttoptions = MqttOptions::new(cargo_pkg_name, mqttserver, 1883);
            mqttoptions.set_keep_alive(Duration::from_secs(5));

            let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

            let (tx, mut rx) = broadcast::channel::<bool>(1);
            let mut rx2 = tx.subscribe();
            let tx2 = tx.clone();

            let jh = tokio::spawn(async move {
                let mut shutdown_in_progress = false;
                while !shutdown_in_progress {
                    tokio::select! {
                        _ = rx2.recv() => {
                            shutdown_in_progress=true;
                            println!("shutdown received in spawn");
                        },
                        notification = eventloop.poll() => {
                            match notification {
                                Ok(Event::Incoming(Packet::Publish(p))) => {
                                        println!("Incoming = {:?}, {:?}", p.topic, p.payload);
                                },
                                Ok(Event::Incoming(Packet::PingResp)) |
                                Ok(Event::Outgoing(Outgoing::PingReq)) => {},
                                Ok(Event::Incoming(Packet::ConnAck(a))) => {
                                    if a.code==rumqttc::ConnectReturnCode::Success {
                                        client.subscribe("#", QoS::AtMostOnce).await.unwrap();
                                    }
                                },
                                Ok(Event::Incoming(i)) => {
                                    println!("Incoming = {:?}", i);
                                },
                                Ok(Event::Outgoing(o)) => {
                                    println!("Outgoing = {:?}", o);
                                },
                                Err(e) => {
                                    println!("Error = {:?}", e);
                                }
                            }
                            //println!("Received = {:?}", notification);
                        }
                    }
                }
                //sleep(Duration::from_millis(1000 * 60 * 1)).await;
                println!("spawned task ending");
            });
            //
            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("ctrl_c received");
                    tx.send(true);
                },
                _ = rx.recv() => {
                    println!("Task ended");
                },
            }
            jh.await;
            println!("task joinded");
        }
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
                            Messages::E1000,
                            b!(
                                "api" => "request_application_token",
                                "message" => response.message.unwrap_or(String::from("<unknown>")
                            )
                        ));
                    } else {
                        let application_token = response.result.unwrap().application_token;
                        context.logevent(
                            Messages::I3001,
                            b!(
                                "api" => "request_application_token",
                                "application_token"=>application_token.clone()                            )
                        );
                    }
                }
                Err(error) => {
                    context.logevent(
                        Messages::E1001,
                        b!(
                            "api" => "request_application_token",
                            "message" => format!("{}",error)
                        ),
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
