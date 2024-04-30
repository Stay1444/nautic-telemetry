use byteorder::ReadBytesExt;
use bytes::{Buf, Bytes};

use crate::Endianness;

#[derive(Clone, Debug)]
pub struct GPS {
    pub satellites: u8,
    pub mps: f32,
    pub lat: f64,
    pub lon: f64,
}

impl GPS {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            satellites: reader.read_u8()?,
            mps: reader.read_f32::<Endianness>()?,
            lat: reader.read_f64::<Endianness>()?,
            lon: reader.read_f64::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Temperature {
    pub tag: u8,
    pub value: f32,
}

impl Temperature {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            tag: reader.read_u8()?,
            value: reader.read_f32::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Voltage {
    pub tag: u8,
    pub value: f32,
}

impl Voltage {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            tag: reader.read_u8()?,
            value: reader.read_f32::<Endianness>()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct RadioReport {
    pub channel: u8,
    pub rx: u32,
    pub tx: u32,
}

impl RadioReport {
    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            channel: reader.read_u8()?,
            rx: reader.read_u32::<Endianness>()?,
            tx: reader.read_u32::<Endianness>()?,
        })
    }
}
