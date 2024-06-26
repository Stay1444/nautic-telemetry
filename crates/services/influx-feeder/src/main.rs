use clap::Parser;
use futures::StreamExt;
use influxdb::{Timestamp, WriteQuery};
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, ExchangeBindOptions},
    types::FieldTable,
    ConnectionProperties,
};
use telemetry::{DatedTelemetry, Telemetry};
use tracing::{error, info};

pub mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv::dotenv();

    let config = config::Configuration::parse();

    setup_logging();

    info!("Starting");

    let influx_client =
        influxdb::Client::new(config.influx_addr, "easyrobotics").with_token(config.influx_token);

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    let queue_name = format!(
        "influx-feeder,{}",
        uuid::Uuid::new_v4().simple().to_string()
    );

    info!("Declaring exchange");

    queues::telemetry::exchange::declare(&channel).await?;

    info!("Declaring queue {queue_name}");

    channel
        .queue_declare(
            &queue_name,
            queues::telemetry::options(),
            queues::telemetry::arguments(),
        )
        .await?;

    info!(
        "Binding exchange {} => {}",
        queues::telemetry::exchange::NAME,
        queue_name
    );

    channel
        .queue_bind(
            &queue_name,
            queues::telemetry::exchange::NAME,
            "",
            Default::default(),
            Default::default(),
        )
        .await?;

    info!("Creating consumer...");

    let mut consumer = channel
        .basic_consume(
            &queue_name,
            "influx-feeder",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Ready");

    while let Some(delivery) = consumer.next().await {
        let Ok(delivery) = delivery else {
            error!("Error receiving delivery: {}", delivery.unwrap_err());
            continue;
        };

        delivery.ack(BasicAckOptions::default()).await?;

        let telemetry: DatedTelemetry = match bincode::deserialize(&delivery.data) {
            Ok(x) => x,
            Err(err) => {
                error!("Error deserializing delivery: {err}");
                continue;
            }
        };

        info!("Received telemetry, pushing to influxdb");

        let query = match &telemetry.telemetry {
            Telemetry::Spatial(spatial) => WriteQuery::new(
                Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                "space",
            )
            .add_field("lat", spatial.latitude)
            .add_field("lon", spatial.longitude)
            .add_field("velocity", spatial.velocity)
            .add_field("altitude", spatial.altitude)
            .add_field("satellites", spatial.satellites),
            Telemetry::Electrical(x) => match x {
                telemetry::ElectricalTelemetry::Amps { tag, value } => WriteQuery::new(
                    Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                    "amperage",
                )
                .add_tag("name", tag.to_owned())
                .add_field("value", value),
                telemetry::ElectricalTelemetry::Voltage { tag, value } => WriteQuery::new(
                    Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                    "voltage",
                )
                .add_tag("name", tag.to_owned())
                .add_field("value", value),
            },
            Telemetry::Environmental(x) => match x {
                telemetry::EnvironmentalTelemetry::Temperature { tag, value } => WriteQuery::new(
                    Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                    "temperature",
                )
                .add_tag("name", tag.to_owned())
                .add_field("value", value),
                telemetry::EnvironmentalTelemetry::Humidity { tag, value } => WriteQuery::new(
                    Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                    "humidity",
                )
                .add_tag("name", tag.to_owned())
                .add_field("value", value),
            },
            Telemetry::Relay(x) => WriteQuery::new(
                Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                "relay",
            )
            .add_tag("name", x.tag.to_owned())
            .add_field("status", x.status),
            Telemetry::System(x) => match x {
                telemetry::SystemTelemetry::Radio { channel, rx, tx } => WriteQuery::new(
                    Timestamp::Milliseconds(telemetry.date.timestamp_millis() as u128),
                    "radio",
                )
                .add_field("channel", channel)
                .add_field("rx", rx)
                .add_field("tx", tx),
            },
        };

        influx_client.query(query).await?;
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
