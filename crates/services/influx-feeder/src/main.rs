use chrono::{DateTime, Utc};
use clap::Parser;
use futures::StreamExt;
use influxdb::{InfluxDbWriteable, Timestamp, WriteQuery};
use lapin::{options::BasicConsumeOptions, types::FieldTable, ConnectionProperties};
use telemetry::Telemetry;
use tracing::{error, info};

pub mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv::dotenv();

    let config = config::Configuration::parse();

    setup_logging();

    info!("Starting");

    let influx_client = influxdb::Client::new(config.influx_addr, "easyrobotics").with_token(
        "d9z-8VjZBaY8zB-JhoIXl9FbrMh_JlNSTq743BBdGG4FQRZOUInk9eKJ2lhlP2iVm44v5vcRW8CqZHbI8ORqVw==",
    );

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    queues::telemetry(&channel).await?;

    let mut consumer = channel
        .basic_consume(
            queues::telemetry::NAME,
            "influx-feeder",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Ok");

    while let Some(delivery) = consumer.next().await {
        let Ok(delivery) = delivery else {
            error!("Error receiving delivery: {}", delivery.unwrap_err());
            continue;
        };

        let telemetry: Telemetry = match bincode::deserialize(&delivery.data) {
            Ok(x) => x,
            Err(err) => {
                error!("Error deserializing delivery: {err}");
                continue;
            }
        };

        info!("Received telemetry, pushing to influxdb");

        let mut query = WriteQuery::new(
            Timestamp::Milliseconds(Utc::now().timestamp_millis() as u128),
            match &telemetry {
                Telemetry::Environmental(environmental) => match environmental {
                    telemetry::EnvironmentalTelemetry::Temperature { tag: _, value: _ } => {
                        "temperature"
                    }
                    telemetry::EnvironmentalTelemetry::Humidity { tag: _, value: _ } => "humidity",
                },
                _ => "todo",
            },
        );

        match &telemetry {
            Telemetry::Environmental(environmental) => match environmental {
                telemetry::EnvironmentalTelemetry::Temperature { tag, value } => {
                    info!("Tag: {tag}");
                    query = query.add_field("name", tag.to_owned());
                    query = query.add_field("value", *value);
                }
                _ => (),
            },
            _ => {
                query = query.add_field("todo", "todo");
            }
        }

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
