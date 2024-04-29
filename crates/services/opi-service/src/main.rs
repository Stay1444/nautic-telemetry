use std::time::Duration;

use clap::Parser;
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, ConnectionProperties,
};
use radio::packets::SlavePacket;
use tracing::info;

pub mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv::dotenv();

    setup_logging();

    let config = config::Configuration::parse();

    info!("Starting");

    let (mut rx, _tx) = radio::open(config.tty.into(), config.baud).await?;

    while let Ok(packet) = rx.recv::<SlavePacket>().await {
        dbg!(packet);
    }

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    channel
        .exchange_declare(
            queues::telemetry::exange::NAME,
            queues::telemetry::exange::KIND,
            queues::telemetry::exange::options(),
            queues::telemetry::exange::arguments(),
        )
        .await?;

    channel
        .queue_declare(
            queues::telemetry::NAME,
            queues::telemetry::options(),
            queues::telemetry::arguments(),
        )
        .await?;

    tokio::spawn(publish(connection.create_channel().await?));

    tokio::time::sleep(Duration::from_secs(999)).await;
    Ok(())
}

async fn publish(channel: Channel) -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_millis(750)).await;

        let data = telemetry::Telemetry::Electrical(telemetry::ElectricalTelemetry::Voltage {
            tag: "motor-0".into(),
            value: 11.8,
        });

        let payload = serde_json::to_string(&data)?;

        channel
            .basic_publish(
                "telemetry-exange",
                queues::telemetry::NAME,
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
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
