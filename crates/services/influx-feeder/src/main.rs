use chrono::Utc;
use clap::Parser;
use futures::StreamExt;
use influxdb::{Timestamp, WriteQuery};
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    ConnectionProperties,
};
use telemetry::Telemetry;
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

        delivery.ack(BasicAckOptions::default()).await?;

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
                Telemetry::System(sys) => match sys {
                    telemetry::SystemTelemetry::Radio {
                        channel: _,
                        rx: _,
                        tx: _,
                    } => "radio",
                },
                _ => "todo",
            },
        );

        match &telemetry {
            Telemetry::Environmental(environmental) => match environmental {
                telemetry::EnvironmentalTelemetry::Temperature { tag, value } => {
                    query = query.add_tag("name", tag.to_owned());
                    query = query.add_field("value", *value);
                }
                _ => (),
            },
            Telemetry::System(sys) => match sys {
                telemetry::SystemTelemetry::Radio { channel, rx, tx } => {
                    query = query.add_field("channel", channel);
                    query = query.add_field("rx", rx);
                    query = query.add_field("tx", tx);
                }
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
