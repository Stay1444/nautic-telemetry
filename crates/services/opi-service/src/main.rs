use std::time::Duration;

use chrono::{TimeDelta, Utc};
use clap::Parser;
use lapin::{
    options::BasicPublishOptions, protocol::basic::AMQPProperties, Channel, ConnectionProperties,
};
use radio::packets::{MasterPacket, SlavePacket};
use telemetry::{DatedTelemetry, Telemetry};
use tracing::info;

use crate::tagger::Tagger;

pub mod config;
pub mod tagger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv::dotenv();

    setup_logging();

    let config = config::Configuration::parse();

    info!("Starting");

    let mut radio = radio::open(config.tty, config.baud, config.gpio_chip, config.gpio_pin).await?;
    let mut tagger = Tagger::new();

    let connection =
        lapin::Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?;

    let channel = connection.create_channel().await?;

    queues::telemetry::exhange::declare(&channel).await?;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        info!("Starting send window");
        radio.write(MasterPacket::StartSendWindow).await?;

        let mut packets = vec![];
        let mut time = 0;

        loop {
            let packet = tokio::select! {
                packet = radio.read() => {
                    packet?
                }
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    break
                }
            };

            if let SlavePacket::EndSendWindow(millis) = packet {
                time = millis;
                info!("Send window ended");
                break;
            }

            packets.push(packet);
        }

        if time == 0 || packets.is_empty() {
            continue;
        }

        for packet in packets {
            if let Some(telemetry) = to_telemetry(packet, &mut tagger, time) {
                send_telemetry(&channel, telemetry).await?;
            }
        }
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

fn to_telemetry(packet: SlavePacket, tagger: &mut Tagger, time: u32) -> Option<DatedTelemetry> {
    let mut now = Utc::now();
    let telemetry = match packet {
        SlavePacket::GPS(x) => {
            now -= TimeDelta::milliseconds((time - x.timestamp) as i64);
            Some(Telemetry::Spatial(telemetry::SpatialTelemetry {
                latitude: x.lat,
                longitude: x.lon,
                velocity: x.mps,
                satellites: x.satellites as i32,
            }))
        }
        SlavePacket::Temperature(x) => {
            now -= TimeDelta::milliseconds((time - x.timestamp) as i64);
            Some(Telemetry::Environmental(
                telemetry::EnvironmentalTelemetry::Temperature {
                    tag: tagger.thermometer(x.tag),
                    value: x.value,
                },
            ))
        }
        SlavePacket::Voltage(x) => {
            now -= TimeDelta::milliseconds((time - x.timestamp) as i64);
            Some(Telemetry::Electrical(
                telemetry::ElectricalTelemetry::Voltage {
                    tag: tagger.voltimeter(x.tag),
                    value: x.value,
                },
            ))
        }
        SlavePacket::RadioReport(x) => {
            now -= TimeDelta::milliseconds((time - x.timestamp) as i64);
            Some(Telemetry::System(telemetry::SystemTelemetry::Radio {
                channel: x.channel,
                rx: x.rx,
                tx: x.tx,
            }))
        }
        SlavePacket::EndSendWindow(_) => None,
    }?;

    Some(DatedTelemetry {
        date: now,
        telemetry,
    })
}

async fn send_telemetry(channel: &Channel, telemetry: DatedTelemetry) -> anyhow::Result<()> {
    let payload = bincode::serialize(&telemetry)?;
    channel
        .basic_publish(
            queues::telemetry::exhange::NAME,
            "",
            BasicPublishOptions::default(),
            &payload,
            AMQPProperties::default(),
        )
        .await?;
    Ok(())
}
