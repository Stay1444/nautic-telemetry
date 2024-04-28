use lapin::{
    options::QueueDeclareOptions,
    types::{AMQPValue, FieldTable},
};

pub const NAME: &str = "telemetry";

pub fn options() -> QueueDeclareOptions {
    QueueDeclareOptions::default()
}

pub fn arguments() -> FieldTable {
    let mut table = FieldTable::default();

    table.insert("x-message-ttl".into(), 0.into()); // Causes messages to be expired upon reaching
                                                    // a queue unless they can be delivered to
                                                    // a consumer immediately

    table
}
