use std::sync::Arc;

use dss_openapi::apis::configuration::{ApiKey, Configuration};
use slog::b;
use tokio::sync::broadcast::{self, Receiver};

enum DssConnectionState {
    Reconnecting,
    Connecting,
    Authenticating,
    Listening,
}
//use futures::future::{BoxFuture, FutureExt};

async fn callit0<'a, F, R>(
    rcv: &'a mut Receiver<bool>,
    context: &'a Configuration,
    f: F,
) -> Result<R, ()>
where
    F: async_fn_traits::AsyncFn1<&'a dss_openapi::apis::configuration::Configuration, Output = R>,
{
    tokio::select! {
        _ = rcv.recv() => {
            println!("shutdown received in spawn");
            Err(())
        }
        zz = f(context) => {
            println!("Function executed");
            Ok(zz)
        }
    }
}
pub fn run(
    context: Arc<crate::context::Context>,
    application_token: &String,
    tx: &broadcast::Sender<bool>,
) -> tokio::task::JoinHandle<()> {
    let at = application_token.clone();
    let local_context = context.clone();
    let mut rx3 = tx.subscribe();
    let jh = tokio::spawn(async move {
        let mut _shutdown_in_progress = false;
        let mut connection_state = DssConnectionState::Connecting;
        while !_shutdown_in_progress {
            match connection_state {
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                DssConnectionState::Reconnecting => {
                    tokio::time::sleep(std::time::Duration::from_millis(1000 * 60)).await;
                    connection_state = DssConnectionState::Connecting;
                }
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                DssConnectionState::Connecting => {
                    // Kopie der Konfiguration für den Zugriff
                    let real_local_configuration: Configuration;
                    {
                        let locked_configuration = local_context.configuration().unwrap();
                        real_local_configuration = locked_configuration.clone();
                    }
                    // ~~~~~~~~~~~~~~~~~
                    let _result = callit0(
                        &mut rx3,
                        &real_local_configuration,
                        dss_openapi::apis::system_api::get_dsid,
                    )
                    .await;
                    // ~~~~~~~~~~~~~~~~~
                    let result =
                        dss_openapi::apis::system_api::get_dsid(&real_local_configuration).await;
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
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                DssConnectionState::Authenticating => {
                    // Kopie der Konfiguration für den Zugriff
                    let real_local_configuration: Configuration;
                    {
                        let locked_configuration = local_context.configuration().unwrap();
                        real_local_configuration = locked_configuration.clone();
                    }
                    let result = dss_openapi::apis::authentication_api::login_application(
                        &real_local_configuration,
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
                            local_context.set_api_key(Some(ApiKey {
                                prefix: None,
                                key: r.token,
                            }));
                        }
                    }
                }
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                DssConnectionState::Listening => {
                    tokio::time::sleep(std::time::Duration::from_millis(1000 * 60 * 5)).await;
                }
            }
        }
    });
    jh
}
