use std::sync::Arc;

use dss_openapi::apis::configuration::ApiKey;
use slog::b;
use tokio::sync::broadcast;

enum DssConnectionState {
    Reconnecting,
    Connecting,
    Authenticating,
    Listening,
}

pub fn run(
    context: Arc<crate::context::Context>,
    application_token: &String,
    tx: &broadcast::Sender<bool>,
) -> tokio::task::JoinHandle<()> {
    let _rx2 = tx.subscribe();
    let at = application_token.clone();
    let mut local_configuration = context.configuration().clone();
    let jh = tokio::spawn(async move {
        let mut _shutdown_in_progress = false;
        let mut connection_state = DssConnectionState::Connecting;
        while !_shutdown_in_progress {
            match connection_state {
                DssConnectionState::Reconnecting => {
                    tokio::time::sleep(std::time::Duration::from_millis(1000 * 60)).await;
                    connection_state = DssConnectionState::Connecting;
                }
                DssConnectionState::Connecting => {
                    let result =
                        dss_openapi::apis::system_api::get_dsid(&context.configuration()).await;
                    if result.is_err() {
                        context.logevent(
                            crate::context::Messages::E1001,
                            b!(
                                "api" => "get_dsid",
                                "message" => format!("{}",result.unwrap_err())
                            ),
                        );
                        connection_state = DssConnectionState::Reconnecting;
                    } else {
                        if result.is_ok() {
                            println!("Response: {:?}", result.unwrap().result.unwrap());
                            connection_state = DssConnectionState::Authenticating;
                        }
                    }
                }
                DssConnectionState::Authenticating => {
                    let result = dss_openapi::apis::authentication_api::login_application(
                        &context.configuration(),
                        &at,
                    )
                    .await;
                    if result.is_err() {
                        context.logevent(
                            crate::context::Messages::E1001,
                            b!(
                                "api" => "login_application",
                                "message" => format!("{}",result.unwrap_err())
                            ),
                        );
                        connection_state = DssConnectionState::Reconnecting;
                    } else {
                        if result.is_ok() {
                            let r = result.unwrap().result.unwrap();
                            println!("Response: {:?}", r);
                            connection_state = DssConnectionState::Listening;
                            local_configuration.api_key = Some(ApiKey {
                                prefix: None,
                                key: r.token,
                            });
                        }
                    }
                }
                DssConnectionState::Listening => {
                    tokio::time::sleep(std::time::Duration::from_millis(1000 * 60 * 5)).await;
                }
            }
        }
        //while !shutdown_in_progress {
        //    tokio::select! {
        //        _ = rx2.recv() => {
        //            shutdown_in_progress=true;
        //            println!("shutdown received in spawn");
        //        }
        //    }
        //}
    });
    jh
}
