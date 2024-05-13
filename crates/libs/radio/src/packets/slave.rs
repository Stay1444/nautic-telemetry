use byteorder::ReadBytesExt;
use bytes::{Buf, Bytes};

use crate::Endianness;

#[derive(Clone, Debug)]
pub struct GPS {
    pub timestamp: u32,
    pub satellites: u8,
    pub mps: f32,
    pub lat: f32,
    pub lon: f32,
    pub altitude: f32,
}

impl GPS {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            timestamp: reader.read_u32::<Endianness>()?,
            satellites: reader.read_u8()?,
            mps: reader.read_f32::<Endianness>()?,
            lat: reader.read_f32::<Endianness>()?,
            lon: reader.read_f32::<Endianness>()?,
            altitude: reader.read_f32::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Temperature {
    pub timestamp: u32,
    pub tag: u8,
    pub value: f32,
}

impl Temperature {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            timestamp: reader.read_u32::<Endianness>()?,
            tag: reader.read_u8()?,
            value: reader.read_f32::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Voltage {
    pub timestamp: u32,
    pub tag: u8,
    pub value: f32,
}

impl Voltage {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            timestamp: reader.read_u32::<Endianness>()?,
            tag: reader.read_u8()?,
            value: reader.read_f32::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Amps {
    pub timestamp: u32,
    pub tag: u8,
    pub value: f32,
}

impl Amps {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            timestamp: reader.read_u32::<Endianness>()?,
            tag: reader.read_u8()?,
            value: reader.read_f32::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct RadioReport {
    pub timestamp: u32,
    pub channel: u8,
    pub rx: u32,
    pub tx: u32,
}

impl RadioReport {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            timestamp: reader.read_u32::<Endianness>()?,
            channel: reader.read_u8()?,
            rx: reader.read_u32::<Endianness>()?,
            tx: reader.read_u32::<Endianness>()?,
        })
    }
}
