use std::io::Write;

use anyhow::anyhow;
use byteorder::WriteBytesExt;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use gpio_cdev::{Line, LineHandle, LineRequestFlags};
use packets::{MasterPacket, SlavePacket};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio_serial::{SerialPort, SerialPortBuilderExt, SerialStream};

pub mod packets;

pub const PACKET_HEAD: u8 = 0xAA;
pub type Endianness = byteorder::BigEndian;

pub struct PacketFrame {
    pub id: u8,
    pub data: Bytes,
}

pub trait PacketGroup: Sized {
    fn id(&self) -> u8;
}

pub trait Deserializable: PacketGroup {
    fn deserialize(frame: PacketFrame) -> anyhow::Result<Self>;
}

pub trait Serializable: PacketGroup {
    fn serialize(&self) -> anyhow::Result<PacketFrame>;
}

pub async fn open(
    tty: String,
    baud: u32,
    gpio_chip: String,
    gpio_pin: u32,
) -> anyhow::Result<Radio> {
    let port = tokio_serial::new(tty, baud).open_native_async()?;

    let mut chip = gpio_cdev::Chip::new(gpio_chip)?;
    let line = chip
        .get_line(gpio_pin)?
        .request(LineRequestFlags::OUTPUT, 1, "opi-radio-set")?;

    Ok(Radio { port, line })
}

pub struct Radio {
    port: SerialStream,
    line: LineHandle,
}

impl Radio {
    pub async fn write(&mut self, packet: MasterPacket) -> anyhow::Result<()> {
        let frame = packet.serialize()?;

        let mut packet = BytesMut::new().writer();

        packet.write_u8(PACKET_HEAD)?;
        packet.write_u8(frame.id)?;
        packet.write_u32::<Endianness>(frame.data.len() as u32)?;
        packet.write_all(&frame.data)?;

        let mut packet = packet.into_inner();

        self.port.write_all_buf(&mut packet).await?;
        Write::flush(&mut self.port)?;

        Ok(())
    }

    pub async fn read(&mut self) -> anyhow::Result<SlavePacket> {
        while self.port.read_u8().await? != PACKET_HEAD {
            continue;
        }

        let id = self.port.read_u8().await?;
        let length = self.port.read_u32().await?; // read length in big endian.

        let mut data = vec![];

        while data.len() < length as usize {
            data.push(self.port.read_u8().await?);
        }

        let frame = PacketFrame {
            id,
            data: Bytes::from(data),
        };

        let packet = SlavePacket::deserialize(frame)?;

        Ok(packet)
    }
}
