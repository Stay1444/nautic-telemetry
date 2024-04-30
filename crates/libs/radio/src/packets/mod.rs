use anyhow::anyhow;
use bytes::{BufMut, BytesMut};

use super::{Deserializable, PacketFrame, PacketGroup, Serializable};

pub mod master;
pub mod slave;

#[derive(Clone, Debug)]
pub enum MasterPacket {
    Ping(master::Ping),
    ChangeChannel(master::ChangeChannel),
}

#[derive(Clone, Debug)]
pub enum SlavePacket {
    Pong(slave::Pong),
    GPS(slave::GPS),
    Temperature(slave::Temperature),
    Voltage(slave::Voltage),
    RadioReport(slave::RadioReport),
}

impl PacketGroup for MasterPacket {
    fn id(&self) -> u8 {
        match self {
            Self::Ping(_) => 0,
            Self::ChangeChannel(_) => 1,
        }
    }
}

impl Serializable for MasterPacket {
    fn serialize(&self) -> anyhow::Result<super::PacketFrame> {
        let mut writer = BytesMut::new().writer();

        match self {
            Self::Ping(p) => p.serialize(&mut writer)?,
            Self::ChangeChannel(p) => p.serialize(&mut writer)?,
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
            Self::Pong(_) => 0,
            Self::GPS(_) => 1,
            Self::Temperature(_) => 2,
            Self::Voltage(_) => 3,
            Self::RadioReport(_) => 4,
        }
    }
}

impl Deserializable for SlavePacket {
    fn deserialize(frame: super::PacketFrame) -> anyhow::Result<Self> {
        Ok(match frame.id {
            0 => Self::Pong(slave::Pong::deserialize(frame.data)?),
            1 => Self::GPS(slave::GPS::deserialize(frame.data)?),
            2 => Self::Temperature(slave::Temperature::deserialize(frame.data)?),
            3 => Self::Voltage(slave::Voltage::deserialize(frame.data)?),
            4 => Self::RadioReport(slave::RadioReport::deserialize(frame.data)?),
            _ => return Err(anyhow!("Unknown Packet for id {}", frame.id)),
        })
    }
}
