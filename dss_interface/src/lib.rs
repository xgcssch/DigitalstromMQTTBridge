use rumqttc::{AsyncClient, Event, MqttOptions, Outgoing, Packet, QoS};
use tokio::{sync::broadcast, time::Duration};

pub fn run_mqtt_subscriptions(
    cargo_pkg_name: &String,
    mqttserver: &String,
    tx: &broadcast::Sender<bool>,
) -> tokio::task::JoinHandle<()> {
    let mut rx2 = tx.subscribe();

    let mut mqttoptions = MqttOptions::new(cargo_pkg_name, mqttserver, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

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
                }
            }
        }
    });
    jh
}
