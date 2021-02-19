use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct MapFileReq {}

impl MapFileReq {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> MapFileReq {
        Self {}
    }
}

impl crate::packets::BodyContents for MapFileReq {
    const ID: u8 = 0x1d;
    fn write<W: std::io::Write>(&self, w: &mut W) {}
}
