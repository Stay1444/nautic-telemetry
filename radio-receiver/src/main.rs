use std::f32::consts::PI;

use clap::Parser;
use radio::packets::{MasterPacket, SlavePacket};

pub mod config;
pub mod radio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();

    let (mut receiver, mut sender) = radio::open(config.tty.into(), config.baud).await?;

    for _ in 0..2 {
        println!("Sending ping");
        sender.send(MasterPacket::Ping).await?;

        let result: Option<SlavePacket> = receiver.recv().await?;

        dbg!(result);
    }

    sender
        .send(MasterPacket::ProtocolTest {
            a: PI,
            b: 144,
            c: 441,
        })
        .await?;

    Ok(())
}
