mod commandline;

extern crate dss_interface;
extern crate dss_openapi;
extern crate reqwest;

extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::sync::Arc;

use crate::commandline::{Cli, Messageformat};
use clap::Parser;
use dss_interface::messages::Messages;
use slog::{b, o, Drain};
use tokio::{signal, sync::broadcast};

#[tokio::main]
async fn main() {
    let cargo_pkg_name = env!("CARGO_PKG_NAME");
    let cargo_pkg_version_major = env!("CARGO_PKG_VERSION_MAJOR");
    let cargo_pkg_version_minor = env!("CARGO_PKG_VERSION_MINOR");
    let cargo_pkg_version_patch = env!("CARGO_PKG_VERSION_PATCH");
    let cargo_pkg_version_pre = option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("");

    let cli = Cli::parse();

    let d: slog::Logger;
    match cli.message_format {
        Messageformat::Json => {
            let drain = slog_json::Json::default(std::io::stderr()).fuse();
            let drain = slog_async::Async::new(drain).build().fuse();
            let _log = slog::Logger::root(drain, o!("application" => cargo_pkg_name));
            d = _log;
        }
        Messageformat::Simple => {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator)
                .use_file_location()
                .build()
                .fuse();

            let drain = slog_async::Async::new(drain).build().fuse();
            let _log = slog::Logger::root(drain, o!("application" => cargo_pkg_name));
            d = _log;
        }
    }

    let dss_client = crate::reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build();
    let dss_client = dss_client.unwrap();

    let context = Arc::new(dss_interface::context::Context::new(
        d,
        dss_openapi::apis::configuration::Configuration {
            client: dss_client,
            base_path: cli.url,
            ..dss_openapi::apis::configuration::Configuration::default()
        },
    ));

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
            application_token,
        } => {
            let (tx, mut rx) = broadcast::channel::<bool>(1);

            let mqtts = dss_interface::mqttsubscriptions::run(
                context.clone(),
                &String::from(cargo_pkg_name),
                mqttserver,
                &tx,
            );
            let dsss =
                dss_interface::dsssubscriptions::run(context.clone(), application_token, &tx);

            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("ctrl_c received");
                    _=tx.send(true);
                },
                _ = rx.recv() => {
                    println!("Task ended");
                },
            }
            _ = tokio::join!(mqtts, dsss);
        }
        crate::commandline::Commands::RequestApplicationToken { application_name } => {
            let real_local_configuration: dss_openapi::apis::configuration::Configuration;
            {
                let locked_configuration = context.configuration().unwrap();
                real_local_configuration = locked_configuration.clone();
            }
            let result = dss_openapi::apis::authentication_api::request_application_token(
                &real_local_configuration,
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
    context.logevent(Messages::I3002, b!());
}
