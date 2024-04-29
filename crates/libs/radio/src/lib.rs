use std::{io::Write, path::PathBuf};

use byteorder::WriteBytesExt;
use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

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

pub async fn open(tty: PathBuf, baud: u32) -> anyhow::Result<(RadioReceiver, RadioSender)> {
    let port = tokio_serial::new(tty.to_string_lossy(), baud).open_native_async()?;

    let (read, write) = tokio::io::split(port);

    Ok((RadioReceiver(read), RadioSender(write)))
}

pub struct RadioSender(WriteHalf<SerialStream>);
impl RadioSender {
    pub async fn send(&mut self, packet: impl Serializable) -> anyhow::Result<()> {
        let frame = packet.serialize()?;

        let mut packet = BytesMut::new().writer();

        packet.write_u8(PACKET_HEAD)?;
        packet.write_u8(frame.id)?;
        packet.write_u32::<Endianness>(frame.data.len() as u32)?;
        packet.write_all(&frame.data)?;

        let packet = packet.into_inner();

        self.0.write_all(&packet).await?;

        Ok(())
    }
}

pub struct RadioReceiver(ReadHalf<SerialStream>);
impl RadioReceiver {
    pub async fn recv<T>(&mut self) -> anyhow::Result<T>
    where
        T: Deserializable,
    {
        while self.0.read_u8().await? != PACKET_HEAD {
            continue;
        }

        let id = self.0.read_u8().await?;
        let length = self.0.read_u32().await?; // read_u32 reads in big endian. Use read_u32_le if
                                               // little-endianness is needed.
        let mut data = vec![];

        while data.len() < length as usize {
            data.push(self.0.read_u8().await?);
        }

        let frame = PacketFrame {
            id,
            data: Bytes::from(data),
        };

        let packet = T::deserialize(frame)?;

        Ok(packet)
    }
}
