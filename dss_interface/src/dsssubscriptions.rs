use std::sync::Arc;

use dss_openapi::apis::configuration::{ApiKey, Configuration};
use slog::b;
use tokio::sync::broadcast::{self, Receiver};

use crate::messages::Messages;

enum DssConnectionState {
    Reconnecting,
    Connecting,
    Authenticating,
    Listening,
}
//use futures::future::{BoxFuture, FutureExt};

async fn callit0<'a, F, R1, R2>(
    rcv: &'a mut Receiver<bool>,
    context: Arc<crate::context::Context>,
    configuration: &'a Configuration,
    api_name: String,
    f: F,
) -> Result<R1, bool>
where
    F: async_fn_traits::AsyncFn1<
        &'a dss_openapi::apis::configuration::Configuration,
        Output = Result<R1, R2>,
    >,
    R1: dss_openapi::status_additions::Status,
    R2: std::fmt::Display + std::fmt::Debug,
{
    tokio::select! {
        _ = rcv.recv() => {
            Err(true)
        }
        ws_call_result = f(configuration) => {
            match &ws_call_result {
                Ok(r) => {
                    let status=r.get_status();
                    let result:Result<R1,bool>;
                    // Status ok?
                    if !status.0 {
                        let message=status.1;
                        if message.is_some() {
                            context.logevent(
                                Messages::E1000,
                                b!(
                                    "api" => api_name,
                                    "message" => format!("{}",message.as_ref().unwrap())
                                ),
                            );
                        } else {
                            context.logevent(
                                Messages::E1000,
                                b!(
                                    "api" => api_name,
                                    "message" => "<unknown>"
                                ),
                            );
                        }
                        result=Err(false);
                    }
                    else {
                        result=Ok(ws_call_result.unwrap());
                    }
                    result
                }
                Err(error) => {
                    context.logevent(
                        Messages::E1001,
                        b!(
                            "api" => api_name,
                            "message" => format!("{}",error)
                        ),
                    );
                    Err(false)
                }
            }
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
                    match callit0(
                        &mut rx3,
                        context.clone(),
                        &real_local_configuration,
                        String::from("get_dsid"),
                        dss_openapi::apis::system_api::get_dsid,
                    )
                    .await
                    {
                        Ok(a) => {
                            context.logevent(
                                Messages::I3003,
                                b!(
                                    "message" => format!("{}",a.result.unwrap().d_sid.unwrap())
                                ),
                            );
                        }
                        Err(e) => {}
                    }

                    // ~~~~~~~~~~~~~~~~~
                    let result =
                        dss_openapi::apis::system_api::get_dsid(&real_local_configuration).await;
                    if result.is_err() {
                        context.logevent(
                            Messages::E1001,
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
                            Messages::E1001,
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
