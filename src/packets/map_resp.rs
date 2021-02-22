use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct MapResp {
    pub map: Option<(String, Vec<u8>)>,
    pub image: Option<(String, Vec<u8>)>,
    pub winter_image: Option<(String, Vec<u8>)>,
}

impl MapResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> MapResp {
        Self {
            map: None,
            image: None,
            winter_image: None,
        }
    }
}

impl crate::packets::BodyContents for MapResp {
    const ID: u8 = 0x14;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        let (
            (map_name, map_contents),
            (image_name, image_contents),
            (winter_name, winter_contents),
        ) = (
            self.map.as_ref().unwrap(),
            self.image.as_ref().unwrap(),
            self.winter_image.as_ref().unwrap(),
        );
        w.write_all(map_name.as_bytes()).unwrap();
        w.write_u8(0).unwrap();
        w.write_u32::<LittleEndian>(map_contents.len() as u32).unwrap();
        w.write_u32::<LittleEndian>(crate::util::calculate_crc(&map_contents)).unwrap();

        w.write_all(image_name.as_bytes()).unwrap();
        w.write_u8(0).unwrap();
        w.write_u32::<LittleEndian>(image_contents.len() as u32).unwrap();
        w.write_u32::<LittleEndian>(crate::util::calculate_crc(&image_contents)).unwrap();

        w.write_all(winter_name.as_bytes()).unwrap();
        w.write_u8(0).unwrap();
        w.write_u32::<LittleEndian>(winter_contents.len() as u32).unwrap();
        w.write_u32::<LittleEndian>(crate::util::calculate_crc(&winter_contents)).unwrap();
    }
}
