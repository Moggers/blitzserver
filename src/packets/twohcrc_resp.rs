use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct TwoHCrcResp {
}

impl TwoHCrcResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> TwoHCrcResp {
        TwoHCrcResp {}
    }
}

impl crate::packets::BodyContents for TwoHCrcResp {
    const ID: u8 = 0x16;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_all(&[0xff, 0xff, 0x38, 0x5b, 0x1, 0x0]).unwrap();
    }
}
