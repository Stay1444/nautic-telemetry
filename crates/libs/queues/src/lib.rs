use lapin::{options::QueueBindOptions, types::FieldTable};

pub mod telemetry;

pub async fn telemetry(channel: &lapin::Channel) -> anyhow::Result<()> {
    channel
        .queue_declare(
            telemetry::NAME,
            telemetry::options(),
            telemetry::arguments(),
        )
        .await?;

    channel
        .exchange_declare(
            telemetry::exhange::NAME,
            telemetry::exhange::KIND,
            telemetry::exhange::options(),
            telemetry::exhange::arguments(),
        )
        .await?;

    channel
        .queue_bind(
            telemetry::NAME,
            telemetry::exhange::NAME,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(())
}
