use byteorder::{ReadBytesExt, WriteBytesExt};
use bytes::{buf::Writer, Buf, Bytes, BytesMut};

use crate::Endianness;

#[derive(Clone, Debug)]
pub struct GPS {
    pub satellites: u8,
    pub mps: f32,
    pub lat: f32,
    pub lon: f32,
}

impl GPS {
    pub fn serialize(&self, writer: &mut Writer<BytesMut>) -> anyhow::Result<()> {
        writer.write_u8(self.satellites)?;
        writer.write_f32::<Endianness>(self.mps)?;
        writer.write_f32::<Endianness>(self.lat)?;
        writer.write_f32::<Endianness>(self.lon)?;
        Ok(())
    }

    pub fn deserialize(data: Bytes) -> anyhow::Result<Self> {
        let mut reader = data.reader();

        Ok(Self {
            satellites: reader.read_u8()?,
            mps: reader.read_f32::<Endianness>()?,
            lat: reader.read_f32::<Endianness>()?,
            lon: reader.read_f32::<Endianness>()?,
        })
    }
}
