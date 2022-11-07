use std::sync::Arc;

use rumqttc::{AsyncClient, Event, LastWill, MqttOptions, Outgoing, Packet, QoS};
use tokio::{sync::broadcast, time::Duration};

static LWT_TOPIC: &str = "tele/dssBridgeV2/LWT";

pub fn run(
    _context: Arc<crate::context::Context>,
    cargo_pkg_name: &String,
    mqttserver: &String,
    tx: &broadcast::Sender<bool>,
) -> tokio::task::JoinHandle<()> {
    let mut rx2 = tx.subscribe();

    let mut mqttoptions = MqttOptions::new(cargo_pkg_name, mqttserver, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    mqttoptions.set_last_will(LastWill::new(
        String::from(LWT_TOPIC),
        "Offline".as_bytes().to_vec(),
        QoS::AtMostOnce,
        true,
    ));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

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
                                client.publish(
                                    String::from(LWT_TOPIC),
                                    QoS::AtMostOnce,
                                    true,
                                    "Online".as_bytes().to_vec(),
                                ).await.unwrap();
                                client.subscribe("cmnd/dssBridge/#", QoS::AtLeastOnce).await.unwrap();
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
                }
            }
        }
    });
    jh
}
