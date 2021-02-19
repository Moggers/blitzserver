use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct UnknownReq {
    remaining: Vec<u8>,
}

impl UnknownReq {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> UnknownReq {
        let mut remaining = vec![];
        r.read_to_end(&mut remaining).unwrap();
        UnknownReq { remaining }
    }
}

impl crate::packets::BodyContents for UnknownReq {
    const ID: u8 = 0x19;
    fn write<W: std::io::Write>(&self, w: &mut W) {}
}
