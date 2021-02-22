
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct MapWinterFileResp {
    pub winter_contents: Vec<u8>,
}

impl MapWinterFileResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> MapWinterFileResp {
        let mut winter_contents= vec![];
        r.read_u16::<LittleEndian>().unwrap();
        r.read_u32::<LittleEndian>().unwrap();
        r.read_to_end(&mut winter_contents).unwrap();
        MapWinterFileResp { winter_contents}
    }
}

impl crate::packets::BodyContents for MapWinterFileResp {
    const ID: u8 = 0x22;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_u32::<LittleEndian>(self.winter_contents.len() as u32).unwrap();
        w.write_all(&self.winter_contents).unwrap();
    }
}


