
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct PAResp {
}

impl PAResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> PAResp {
        PAResp {}
    }
}

impl crate::packets::BodyContents for PAResp {
    const ID: u8 = 0x2;
    fn write<W: std::io::Write>(&self, w: &mut W) {}
}
