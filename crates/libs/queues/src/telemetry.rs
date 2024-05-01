use lapin::{options::QueueDeclareOptions, types::FieldTable};

pub fn options() -> QueueDeclareOptions {
    QueueDeclareOptions {
        auto_delete: true,
        ..Default::default()
    }
}

pub fn arguments() -> FieldTable {
    FieldTable::default()
}

pub mod exchange {
    use lapin::{options::ExchangeDeclareOptions, types::FieldTable, Channel, ExchangeKind};

    pub const NAME: &str = "telemetry-exchange";
    pub const KIND: lapin::ExchangeKind = ExchangeKind::Fanout;

    pub fn options() -> ExchangeDeclareOptions {
        ExchangeDeclareOptions::default()
    }

    pub fn arguments() -> FieldTable {
        FieldTable::default()
    }

    pub async fn declare(channel: &Channel) -> lapin::Result<()> {
        channel
            .exchange_declare(NAME, KIND, options(), arguments())
            .await?;

        Ok(())
    }
}
