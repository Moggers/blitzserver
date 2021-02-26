use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct DmFileResp {
    pub contents: Vec<u8>,
}

impl DmFileResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> DmFileResp {
        let len = r.read_u32::<LittleEndian>().unwrap();
        let mut contents = vec![];
        r.read_to_end(&mut contents).unwrap();
        DmFileResp { contents }
    }
}

impl crate::packets::BodyContents for DmFileResp {
    const ID: u8 = 0x10;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_u32::<LittleEndian>(self.contents.len() as u32).unwrap();
        w.write_all(&self.contents).unwrap();
    }
}
