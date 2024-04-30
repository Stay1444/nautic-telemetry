use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use clap::Parser;
use lapin::{options::BasicPublishOptions, BasicProperties, ConnectionProperties};
use radio::{
    packets::{MasterPacket, SlavePacket},
    RadioSender,
};
use telemetry::{ElectricalTelemetry, EnvironmentalTelemetry, SpatialTelemetry, Telemetry};
use tokio::sync::Mutex;
use tracing::info;

use crate::tagger::Tagger;

pub mod config;
pub mod tagger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv::dotenv();

    setup_logging();

    let config = config::Configuration::parse();
    let mut tagger = Tagger::new();

    info!("Starting");

    let (mut rx, tx) = radio::open(config.tty.into(), config.baud).await?;
    let pending_pings = Arc::new(Mutex::new(Vec::<(u8, DateTime<Utc>)>::new()));

    tokio::spawn(pinger(tx, pending_pings.clone()));

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    queues::telemetry(&channel).await?;

    while let Ok(packet) = rx.recv::<SlavePacket>().await {
        let telemetry = match packet {
            SlavePacket::Pong(pong) => {
                let mut lock = pending_pings.lock().await;
                let Some(index) = lock.iter().position(|x| x.0 == pong.id) else {
                    continue;
                };

                let (_, time) = lock.remove(index);

                let now = Utc::now();

                let diff = now - time;

                let milliseconds = diff.num_milliseconds();

                Some(Telemetry::System(telemetry::SystemTelemetry::Ping {
                    milliseconds: milliseconds as u64,
                }))
            }
            SlavePacket::GPS(gps) => Some(Telemetry::Spatial(SpatialTelemetry {
                latitude: gps.lat,
                longitude: gps.lon,
                velocity: gps.mps,
                satellites: gps.satellites as i32,
            })),
            SlavePacket::Voltage(voltage) => {
                Some(Telemetry::Electrical(ElectricalTelemetry::Voltage {
                    tag: tagger.voltimeter(voltage.tag),
                    value: voltage.value,
                }))
            }
            SlavePacket::Temperature(temperature) => Some(Telemetry::Environmental(
                EnvironmentalTelemetry::Temperature {
                    tag: tagger.thermometer(temperature.tag),
                    value: temperature.value,
                },
            )),
            SlavePacket::RadioReport(radio) => {
                Some(Telemetry::System(telemetry::SystemTelemetry::Radio {
                    channel: radio.channel,
                    rx: radio.rx,
                    tx: radio.tx,
                }))
            }
            _ => None,
        };

        let Some(telemetry) = telemetry else {
            continue;
        };

        let payload = bincode::serialize(&telemetry)?;

        channel
            .basic_publish(
                "telemetry-exange",
                queues::telemetry::NAME,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?;

        println!("Pushed telemetry");
    }

    Ok(())
}

fn setup_logging() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

async fn pinger(
    mut tx: RadioSender,
    pings: Arc<Mutex<Vec<(u8, DateTime<Utc>)>>>,
) -> anyhow::Result<()> {
    let mut id = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;

        tx.send(MasterPacket::Ping(radio::packets::master::Ping { id }))
            .await?;

        pings.lock().await.push((id, Utc::now()));

        id += 1;

        if id >= u8::MAX {
            id = 0;
        }
    }
}
