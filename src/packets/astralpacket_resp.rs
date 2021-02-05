use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct AstralPacketResp {}

impl AstralPacketResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> AstralPacketResp {
        AstralPacketResp {}
    }
}

impl crate::packets::BodyContents for AstralPacketResp {
    const ID: u8 = 0x12;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_all(&[0xff,0xff]).unwrap();
    }
}
