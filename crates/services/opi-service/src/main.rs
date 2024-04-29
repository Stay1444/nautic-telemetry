use std::{
    io::{Read, Write},
    time::Duration,
};

use clap::Parser;
use gpio_cdev::{Chip, LineRequestFlags};
use lapin::{
    options::{BasicPublishOptions, QueueBindOptions},
    types::FieldTable,
    BasicProperties, ConnectionProperties,
};
use radio::packets::SlavePacket;
use telemetry::{ElectricalTelemetry, EnvironmentalTelemetry, SpatialTelemetry, Telemetry};
use tokio_serial::SerialPortBuilderExt;
use tracing::info;

pub mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv::dotenv();

    setup_logging();

    let config = config::Configuration::parse();

    info!("Starting");

    let mut chip = Chip::new("/dev/gpiochip1")?;
    let handle = chip
        .get_line(6)?
        .request(LineRequestFlags::OUTPUT, 1, "radio-set-pin")?;

    println!("Setting to low");
    handle.set_value(0)?;
    let mut port = tokio_serial::new(config.tty.clone(), config.baud).open_native_async()?;

    port.write_all(b"AT\r\n")?;

    let mut buffer = vec![0u8; 8];

    loop {
        let read = port.read(&mut buffer)?;
        if read > 0 {
            println!("{}", String::from_utf8_lossy(&buffer));
        }
    }

    return Ok(());

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
                    tag: String::from("Untagged"),
                    value: voltage.value,
                }))
            }
            SlavePacket::Temperature(temperature) => Some(Telemetry::Environmental(
                EnvironmentalTelemetry::Temperature {
                    tag: String::from("Untagged"),
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
