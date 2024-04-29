use clap::Parser;
use lapin::{
    options::{BasicPublishOptions, QueueBindOptions},
    types::FieldTable,
    BasicProperties, ConnectionProperties,
};
use radio::packets::SlavePacket;
use telemetry::{ElectricalTelemetry, EnvironmentalTelemetry, SpatialTelemetry, Telemetry};
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

    let (mut rx, _tx) = radio::open(config.tty.into(), config.baud).await?;

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    channel
        .queue_declare(
            queues::telemetry::NAME,
            queues::telemetry::options(),
            queues::telemetry::arguments(),
        )
        .await?;

    channel
        .exchange_declare(
            queues::telemetry::exhange::NAME,
            queues::telemetry::exhange::KIND,
            queues::telemetry::exhange::options(),
            queues::telemetry::exhange::arguments(),
        )
        .await?;

    channel
        .queue_bind(
            queues::telemetry::NAME,
            queues::telemetry::exhange::NAME,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Ok(packet) = rx.recv::<SlavePacket>().await {
        let telemetry = match packet {
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
