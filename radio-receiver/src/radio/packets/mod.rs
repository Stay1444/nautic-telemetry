use anyhow::anyhow;
use bytes::{BufMut, BytesMut};

use super::{PacketFrame, PacketGroup};

pub mod master;
pub mod slave;

#[derive(Clone, Debug)]
pub enum MasterPacket {
    Ping,
}

#[derive(Clone, Debug)]
pub enum SlavePacket {
    Pong,
    GPS(slave::GPS),
}

impl PacketGroup for MasterPacket {
    fn serialize(&self) -> anyhow::Result<super::PacketFrame> {
        let writer = BytesMut::new().writer();

        match self {
            Self::Ping => (),
        }

        let frame = PacketFrame {
            id: self.id(),
            data: writer.into_inner().into(),
        };

        Ok(frame)
    }

    fn deserialize(frame: super::PacketFrame) -> anyhow::Result<Self> {
        Ok(match frame.id {
            0 => Self::Ping,
            _ => return Err(anyhow!("Unknown Packet for id {}", frame.id)),
        })
    }

    fn id(&self) -> u8 {
        match self {
            Self::Ping => 0,
        }
    }
}

impl PacketGroup for SlavePacket {
    fn serialize(&self) -> anyhow::Result<PacketFrame> {
        let mut writer = BytesMut::new().writer();

        match self {
            Self::Pong => (),
            Self::GPS(gps) => gps.serialize(&mut writer)?,
        }

        let frame = PacketFrame {
            id: self.id(),
            data: writer.into_inner().into(),
        };

        Ok(frame)
    }

    fn deserialize(frame: super::PacketFrame) -> anyhow::Result<Self> {
        Ok(match frame.id {
            0 => Self::Pong,
            1 => Self::GPS(slave::GPS::deserialize(frame.data)?),
            _ => return Err(anyhow!("Unknown Packet for id {}", frame.id)),
        })
    }

    fn id(&self) -> u8 {
        match self {
            Self::Pong => 0,
            Self::GPS(_) => 1,
        }
    }
}
