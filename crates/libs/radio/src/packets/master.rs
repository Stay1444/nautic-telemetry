use byteorder::WriteBytesExt;
use bytes::{buf::Writer, BytesMut};

#[derive(Clone, Debug)]
pub struct ChangeChannel {
    pub value: u8,
}

impl ChangeChannel {
    pub fn serialize(&self, writer: &mut Writer<BytesMut>) -> anyhow::Result<()> {
        writer.write_u8(self.value)?;
        Ok(())
    }
}
