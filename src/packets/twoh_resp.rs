

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct TwoHResp {
    pub nation_id: u16,
    pub twoh_contents: Vec<u8>,
}

impl TwoHResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> TwoHResp {
        let mut twoh_contents = vec![];
        let nation_id = r.read_u16::<LittleEndian>().unwrap();
        let _len = r.read_u32::<LittleEndian>().unwrap();
        r.read_u16::<LittleEndian>().unwrap();
        r.read_to_end(&mut twoh_contents).unwrap();
        TwoHResp { nation_id, twoh_contents }
    }
}

impl crate::packets::BodyContents for TwoHResp {
    const ID: u8 = 0x18;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_u16::<LittleEndian>(self.nation_id).unwrap();
        w.write_u16::<LittleEndian>(0x2c01).unwrap();
        w.write_u32::<LittleEndian>(self.twoh_contents.len() as u32)
            .unwrap();
        w.write_all(&self.twoh_contents).unwrap();
    }
}
