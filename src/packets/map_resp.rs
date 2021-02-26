use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Debug, Clone)]
pub struct MapResp {
    pub map: (String, Vec<u8>),
    pub image: (String, Vec<u8>),
    pub winter_image: Option<(String, Vec<u8>)>,
    map_crc: u32,
    image_crc: u32,
    winter_crc: Option<u32>,
}

impl MapResp {
    pub fn new(
        map_name: String,
        map_contents: Vec<u8>,
        image_name: String,
        image_contents: Vec<u8>,
        winter_name: Option<String>,
        winter_contents: Option<Vec<u8>>,
    ) -> Self {
        Self {
            map_crc: crate::util::calculate_crc(&map_contents[..]),
            image_crc: crate::util::calculate_crc(&image_contents[..]),
            winter_crc: match &winter_contents {
                Some(wc) => Some(crate::util::calculate_crc(&wc[..])),
                None => None,
            },
            map: (map_name, map_contents),
            image: (image_name, image_contents),
            winter_image: match (winter_name, winter_contents) {
                (Some(winter_name), Some(winter_contents)) => Some((winter_name, winter_contents)),
                _ => None,
            },
        }
    }
    pub fn from_reader<R: std::io::BufRead>(r: &mut R) -> MapResp {
        let mut mapname_bytes = vec![];
        r.read_until(0x0, &mut mapname_bytes).unwrap();
        let mapname = String::from_utf8_lossy(&mapname_bytes).to_string();
        let len = r.read_u32::<LittleEndian>().unwrap();
        let mut map_contents: Vec<u8> = vec![0; len as usize];
        r.read_exact(&mut map_contents).unwrap();
        let mut imagename_bytes = vec![];
        r.read_until(0x0, &mut imagename_bytes).unwrap();
        let imagename = String::from_utf8_lossy(&imagename_bytes).to_string();
        let len = r.read_u32::<LittleEndian>().unwrap();
        let mut image_contents: Vec<u8> = vec![0; len as usize];
        r.read_exact(&mut image_contents).unwrap();
        let mut wintername_bytes = vec![];
        match r.read_until(0x0, &mut wintername_bytes) {
            Ok(t) if t > 0 => {
                let wintername = String::from_utf8_lossy(&wintername_bytes).to_string();
                let len = r.read_u32::<LittleEndian>().unwrap();
                let mut winter_contents: Vec<u8> = vec![0; len as usize];
                r.read_exact(&mut winter_contents).unwrap();
                MapResp::new(
                    mapname,
                    map_contents,
                    imagename,
                    image_contents,
                    Some(wintername),
                    Some(winter_contents),
                )
            }
            _ => MapResp::new(mapname, map_contents, imagename, image_contents, None, None),
        }
    }
}

impl crate::packets::BodyContents for MapResp {
    const ID: u8 = 0x14;
    fn write<W: std::io::Write>(&self, w: &mut W) {
        let ((map_name, map_contents), (image_name, image_contents)) = (&self.map, &self.image);
        w.write_all(map_name.as_bytes()).unwrap();
        w.write_u8(0).unwrap();
        w.write_u32::<LittleEndian>(map_contents.len() as u32)
            .unwrap();
        w.write_u32::<LittleEndian>(self.map_crc).unwrap();

        w.write_all(image_name.as_bytes()).unwrap();
        w.write_u8(0).unwrap();
        w.write_u32::<LittleEndian>(image_contents.len() as u32)
            .unwrap();
        w.write_u32::<LittleEndian>(self.image_crc).unwrap();

        if let (Some((winter_name, winter_contents)), Some(crc)) =
            (&self.winter_image, &self.winter_crc)
        {
            w.write_all(winter_name.as_bytes()).unwrap();
            w.write_u8(0).unwrap();
            w.write_u32::<LittleEndian>(winter_contents.len() as u32)
                .unwrap();
            w.write_u32::<LittleEndian>(*crc).unwrap();
        }
    }
}
