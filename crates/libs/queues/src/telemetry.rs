use lapin::{options::QueueDeclareOptions, types::FieldTable};

pub fn options() -> QueueDeclareOptions {
    QueueDeclareOptions::default()
}

pub fn arguments() -> FieldTable {
    let mut table = FieldTable::default();

    table.insert("x-auto-delete".into(), true.into());
    table
}

pub mod exhange {
    use lapin::{options::ExchangeDeclareOptions, types::FieldTable, Channel, ExchangeKind};

    pub const NAME: &str = "telemetry-exange";
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
