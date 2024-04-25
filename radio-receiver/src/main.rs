use clap::Parser;
use tokio::io::AsyncWriteExt;
use tokio_serial::SerialPortBuilderExt;

pub mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();
    let packet_index: u32 = 100;
    let data: Vec<u8> = vec![82];
    let data_length = data.len() as u32;

    const CRC: crc::Crc<u8> = crc::Crc::<u8>::new(&crc::CRC_8_SMBUS);
    let mut digest = CRC.digest();
    digest.update(&data);
    let crc_val = digest.finalize();

    println!("CRC value for data is {:#02X} ({})", crc_val, crc_val);

    let mut packet = vec![];
    packet.push(0xAA); // head
    packet.push(64); // id
    packet.extend_from_slice(&data_length.to_be_bytes()); // data length
    packet.extend_from_slice(&data); // data
    packet.extend_from_slice(&packet_index.to_be_bytes()); // packet index
    packet.push(crc_val);

    let mut port = tokio_serial::new(config.tty, config.baud).open_native_async()?;
    port.write_all(&mut packet).await?;
    port.flush().await?;

    println!("---");

    for b in &packet {
        print!("{} ", b);
    }

    println!();
    println!("---");

    println!("Ok! Exiting");

    Ok(())
}
