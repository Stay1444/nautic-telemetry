use anyhow::anyhow;
use byteorder::WriteBytesExt;
use bytes::{BufMut, BytesMut};

use super::{Deserializable, Endianness, PacketFrame, PacketGroup, Serializable};

pub mod master;
pub mod slave;

#[derive(Clone, Debug)]
pub enum MasterPacket {
    Ping,
    ProtocolTest { a: f32, b: u32, c: i32 },
}

#[derive(Clone, Debug)]
pub enum SlavePacket {
    Pong,
    GPS(slave::GPS),
}

impl PacketGroup for MasterPacket {
    fn id(&self) -> u8 {
        match self {
            Self::Ping => 0,
            Self::ProtocolTest { a, b, c } => 1,
        }
    }
}

impl Serializable for MasterPacket {
    fn serialize(&self) -> anyhow::Result<super::PacketFrame> {
        let mut writer = BytesMut::new().writer();

        match self {
            Self::Ping => (),
            Self::ProtocolTest { a, b, c } => {
                writer.write_f32::<Endianness>(*a)?;
                writer.write_u32::<Endianness>(*b)?;
                writer.write_i32::<Endianness>(*c)?;
            }
        }

        let frame = PacketFrame {
            id: self.id(),
            data: writer.into_inner().into(),
        };

        Ok(frame)
    }
}

impl PacketGroup for SlavePacket {
    fn id(&self) -> u8 {
        match self {
            Self::Pong => 0,
            Self::GPS(_) => 1,
        }
    }
}

impl Deserializable for SlavePacket {
    fn deserialize(frame: super::PacketFrame) -> anyhow::Result<Self> {
        Ok(match frame.id {
            0 => Self::Pong,
            1 => Self::GPS(slave::GPS::deserialize(frame.data)?),
            _ => return Err(anyhow!("Unknown Packet for id {}", frame.id)),
        })
    }
}
