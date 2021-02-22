use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Seek, SeekFrom};

#[derive(Debug)]
pub struct TwoH {
    pub gamename: String,
    pub turnnumber: i32,
    pub cdkey: u64,
    pub nationid: i32,
    pub turnkey: u32,
    pub password: String,
    pub master_password: String,
    pub file_contents: Vec<u8>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_ctis() {
        use crate::twoh::TwoH;
        let trn_np = TwoH::read_contents(std::io::Cursor::new(&include_bytes!(
            "../test_data/ctis_no_pass.2h"
        )))
        .unwrap();
        assert_eq!(trn_np.password, "");
        let trn_p = TwoH::read_contents(std::io::Cursor::new(&include_bytes!(
            "../test_data/ctis_pass.2h"
        )))
        .unwrap();
        assert_eq!(trn_p.password, "password");
    }
    #[test]
    fn deserialize_ftherlnd() {
        use crate::twoh::TwoH;
        let fthrlnd = TwoH::read_contents(std::io::Cursor::new(&include_bytes!(
            "../test_data/tstgame20_ftherlnd"
        )))
        .unwrap();
        assert_eq!(fthrlnd.password, "father");
    }
    #[test]
    fn deserialize_2h() {
        use crate::twoh::TwoH;
        let twoh = TwoH::read_contents(std::io::Cursor::new(&include_bytes!(
            "../test_data/submit_turn.2h"
        )))
        .unwrap();
        assert_eq!(twoh.nationid, 0xf);
        assert_eq!(twoh.gamename, "tstgame20");
        assert_eq!(twoh.password, "");
    }
}

impl TwoH {
    fn read_magic_marker<R: std::io::Read>(mut file: R) -> String {
        let mut magic: [u8; 3] = [0, 0, 0];
        file.read_exact(&mut magic).unwrap();
        let magic_string = std::str::from_utf8(&magic).unwrap();
        if magic_string != "DOM" {
            panic!("Magic string bogus, expected DOM, found {}", magic_string);
        }
        return magic_string.to_string();
    }

    pub fn read_contents<R: Seek + std::io::Read>(mut file: R) -> Option<Self> {
        let _unk = file.read_u24::<LittleEndian>().unwrap();
        Self::read_magic_marker(&mut file);
        let cdkey = file.read_u64::<LittleEndian>().unwrap();
        let turnnumber = file.read_i32::<LittleEndian>().unwrap();
        let mut unk: [u8; 8] = [0; 8];
        file.read_exact(&mut unk).unwrap();
        let nationid = file.read_i32::<LittleEndian>().unwrap();
        let mut unk1: [u8; 8] = [0; 8];
        file.read_exact(&mut unk1).unwrap();
        let mut game_name_bytes: Vec<u8> = vec![];
        loop {
            let c: u8 = file.read_u8().unwrap();
            if c == 0x4f {
                break;
            }
            game_name_bytes.push(c ^ 0x4f);
        }
        let mut password_bytes: Vec<u8> = vec![];
        let mut master_password_bytes: Vec<u8> = vec![];
        let mut magic = 0x78;
        loop {
            let c: u8 = file.read_u8().unwrap() ^ magic;
            if c == 0 {
                break;
            }
            magic = (std::num::Wrapping(c) + std::num::Wrapping(magic)).0;
            password_bytes.push(c);
        }
        let mut magic = 0x78;
        loop {
            let c: u8 = file.read_u8().unwrap() ^ magic;
            if c == 0 {
                break;
            }
            magic = (std::num::Wrapping(c) + std::num::Wrapping(magic)).0;
            master_password_bytes.push(c);
        }
        let turnkey = file.read_u32::<LittleEndian>().unwrap();

        let mut contents = vec![];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_to_end(&mut contents).unwrap();

        Some(Self {
            gamename: String::from_utf8_lossy(&game_name_bytes).to_string(),
            turnnumber,
            cdkey,
            nationid,
            turnkey,
            file_contents: contents,
            password: String::from_utf8_lossy(&password_bytes).to_string(),
            master_password: String::from_utf8_lossy(&master_password_bytes).to_string(),
        })
    }

    pub fn read_file(filepath: &std::path::Path) -> Option<Self> {
        let file = if let Ok(file) = std::fs::File::open(filepath) {
            file
        } else {
            return None;
        };
        Self::read_contents(file)
    }
}
