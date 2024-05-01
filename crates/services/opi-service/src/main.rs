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
use tracing::{info, warn};

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

    let mut radio = radio::open(config.tty, config.baud, config.gpio_chip, config.gpio_pin).await?;

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    queues::telemetry(&channel).await?;

    loop {
        let packet = rx.recv::<SlavePacket>().await.unwrap();
        let telemetry = match packet {
            SlavePacket::Pong(pong) => {
                let mut lock = pending_pings.lock().await;
                let Some(index) = lock.iter().position(|x| x.0 == pong.id) else {
                    warn!("Got response to unsolicited ping, {}, corruption?", pong.id);
                    lock.clear();
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

        info!("Pushed telemetry");
    }
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
    let mut total = 0;
    loop {
        info!("{total}");

        tx.send(MasterPacket::Ping(radio::packets::master::Ping { id: 0 }))
            .await?;

        total += 1;

        if total >= 1000 {
            break;
        }
    }

    Ok(())
}
