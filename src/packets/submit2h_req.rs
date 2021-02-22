use byteorder::{ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct Submit2hReq {
    pub nation_id: u8,
    pub unk: [u8; 7],
    pub twoh_contents: Vec<u8>,
}

impl Submit2hReq {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> Submit2hReq {
        let mut unk: [u8; 7] = [0; 7];
        let mut twoh_contents = vec![];
        let nation_id = r.read_u8().unwrap();
        r.read_exact(&mut unk).unwrap();
        r.read_to_end(&mut twoh_contents).unwrap();
        Submit2hReq {
            nation_id,
            unk,
            twoh_contents,
        }
    }
}

impl crate::packets::BodyContents for Submit2hReq {
    const ID: u8 = 0x9;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_u8(self.nation_id).unwrap();
        w.write_all(&[0x00, 0x00, 0x02c, 0xe6, 0x14, 0x00, 0x00])
            .unwrap();
        w.write_all(&self.twoh_contents).unwrap();
    }
}
