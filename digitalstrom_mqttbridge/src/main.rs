mod commandline;

extern crate dss_interface;
extern crate dss_openapi;
extern crate reqwest;

extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::{error, info, o, Drain};

use crate::commandline::{Cli, Messageformat};
use clap::Parser;

struct Context {
    log: slog::Logger,
    configuration: dss_openapi::apis::configuration::Configuration,
}

//enum Messages {
//    E0001,
//}

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

    info!(
        context.log,
        "{} - {version_major}.{version_minor}.{version_patch}{version_pre} {event}",
        cargo_pkg_name,
        event="start",
        version_major=cargo_pkg_version_major,
        version_minor=cargo_pkg_version_minor,
        version_patch=cargo_pkg_version_patch,
        version_pre=cargo_pkg_version_pre,
        ;
        "eventid" => 1
    );

    match &cli.command {
        crate::commandline::Commands::Run {
            mqttserver,
            application_token,
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
                        error!(
                            context.log,
                            "outcome was not ok";
                            "api" => "request_application_token",
                            "message" => response.message.unwrap_or(String::from("<unknown>")),
                            "eventid" => 4
                        );
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
