use super::ReadDom5Ext;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Seek;

pub struct Header {
    pub gamename: String,
    pub turnnumber: i32,
    pub cdkey: u64,
    pub nationid: i32,
    pub turnkey: u32,
    pub password: String,
    pub master_password: String,
    pub status: i32,
    pub file_type: u32,
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwoH")
            .field("gamename", &self.gamename)
            .field("turnnumber", &self.turnnumber)
            .field("cdkey", &self.cdkey)
            .field("nationid", &self.nationid)
            .field("turnkey", &self.turnkey)
            .field("password", &self.password)
            .field("master_password", &self.master_password)
            .field("status", &self.status)
            .field("file_type", &self.file_type)
            .finish()
    }
}

impl Header {
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
        let file_type = file.read_u32::<LittleEndian>().unwrap();
        let _unk = file.read_u32::<LittleEndian>().unwrap();
        let nationid = file.read_i32::<LittleEndian>().unwrap();
        let status = file.read_i32::<LittleEndian>().unwrap();
        file.read_i32::<LittleEndian>().unwrap();
        let gamename = file.read_domstring().unwrap();
        let password = file.read_domsecret().unwrap();
        let masterpass = file.read_domsecret().unwrap();
        let turnkey = file.read_u32::<LittleEndian>().unwrap();
        Some(Self {
            gamename,
            turnnumber,
            cdkey,
            nationid,
            turnkey,
            status,
            password,
            master_password: masterpass,
            file_type,
        })
    }
}
