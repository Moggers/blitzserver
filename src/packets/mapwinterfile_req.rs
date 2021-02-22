

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct MapWinterFileReq {}

impl MapWinterFileReq {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> MapWinterFileReq {
        Self {}
    }
}

impl crate::packets::BodyContents for MapWinterFileReq {
    const ID: u8 = 0x21;
    fn write<W: std::io::Write>(&self, w: &mut W) {}
}
