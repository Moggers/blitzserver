use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct AstralPacketReq {}

impl AstralPacketReq {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> AstralPacketReq {
        AstralPacketReq {}
    }
}

impl crate::packets::BodyContents for AstralPacketReq {
    const ID: u8 = 0x11;
    fn write<W: std::io::Write>(&self, w: &mut W) {}
}
