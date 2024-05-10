use anyhow::anyhow;
use byteorder::ReadBytesExt;
use bytes::{Buf, BufMut, BytesMut};

use crate::Endianness;

use super::{Deserializable, PacketFrame, PacketGroup, Serializable};

pub mod master;
pub mod slave;

#[derive(Clone, Debug)]
pub enum MasterPacket {
    StartSendWindow,
    ChangeChannel(master::ChangeChannel),
}

#[derive(Clone, Debug)]
pub enum SlavePacket {
    EndSendWindow(u32),
    GPS(slave::GPS),
    Temperature(slave::Temperature),
    Voltage(slave::Voltage),
    RadioReport(slave::RadioReport),
    Amps(slave::Amps),
}

impl PacketGroup for MasterPacket {
    fn id(&self) -> u8 {
        match self {
            Self::StartSendWindow => 0,
            Self::ChangeChannel(_) => 1,
        }
    }
}

impl Serializable for MasterPacket {
    fn serialize(&self) -> anyhow::Result<super::PacketFrame> {
        let mut writer = BytesMut::new().writer();

        match self {
            Self::StartSendWindow => (),
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
            Self::EndSendWindow(_) => 0,
            Self::GPS(_) => 1,
            Self::Temperature(_) => 2,
            Self::Voltage(_) => 3,
            Self::RadioReport(_) => 4,
            Self::Amps(_) => 5,
        }
    }
}

impl Deserializable for SlavePacket {
    fn deserialize(frame: super::PacketFrame) -> anyhow::Result<Self> {
        Ok(match frame.id {
            0 => Self::EndSendWindow(frame.data.reader().read_u32::<Endianness>()?),
            1 => Self::GPS(slave::GPS::deserialize(frame.data)?),
            2 => Self::Temperature(slave::Temperature::deserialize(frame.data)?),
            3 => Self::Voltage(slave::Voltage::deserialize(frame.data)?),
            4 => Self::RadioReport(slave::RadioReport::deserialize(frame.data)?),
            5 => Self::Amps(slave::Amps::deserialize(frame.data)?),
            _ => return Err(anyhow!("Unknown Packet for id {}", frame.id)),
        })
    }
}
