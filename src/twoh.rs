use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub struct TwoH {
    pub gamename: String,
    pub turnnumber: i32,
    pub cdkey: u64,
    pub nationid: i32,
    pub file_contents: Vec<u8>,
}

impl TwoH {
    fn read_magic_marker(file: &mut std::fs::File) -> String {
        let mut magic: [u8; 3] = [0, 0, 0];
        file.read_exact(&mut magic).unwrap();
        let magic_string = std::str::from_utf8(&magic).unwrap();
        if magic_string != "DOM" {
            panic!("Magic string bogus, expected DOM, found {}", magic_string);
        }
        return magic_string.to_string();
    }

    pub fn read_file(filepath: &std::path::Path) -> Self {
        let mut file = std::fs::File::open(filepath).unwrap();

        let _unk = file.read_u24::<LittleEndian>().unwrap();
        Self::read_magic_marker(&mut file);
        let cdkey = file.read_u64::<LittleEndian>().unwrap();
        let turnnumber = file.read_i32::<LittleEndian>().unwrap();
        let mut unk: [u8; 8] = [0; 8];
        file.read_exact(&mut unk).unwrap();
        let nationid = file.read_i32::<LittleEndian>().unwrap();

        let mut contents = vec![];
        std::fs::File::open(filepath)
            .unwrap()
            .read_to_end(&mut contents)
            .unwrap();

        Self {
            gamename: "".to_string(),
            turnnumber,
            cdkey,
            nationid,
            file_contents: contents,
        }
    }
}
