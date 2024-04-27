use clap::Parser;
use radio::packets::{MasterPacket, SlavePacket};

pub mod config;
pub mod radio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();

    let (mut receiver, mut sender) = radio::open(config.tty.into(), config.baud).await?;

    sender.send(MasterPacket::Ping).await?;

    let result: Option<SlavePacket> = receiver.recv().await?;

    dbg!(result);

    Ok(())
}
