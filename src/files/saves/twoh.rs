use super::ReadDom5Ext;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Seek, SeekFrom};

pub struct TwoH {
    pub gamename: String,
    pub turnnumber: i32,
    pub cdkey: u64,
    pub nationid: i32,
    pub turnkey: u32,
    pub password: String,
    pub master_password: String,
    pub body: FileBody,
    pub file_contents: Vec<u8>,
    pub status: i32,
}

impl std::fmt::Debug for TwoH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwoH")
            .field("gamename", &self.gamename)
            .field("turnnumber", &self.turnnumber)
            .field("cdkey", &self.cdkey)
            .field("nationid", &self.nationid)
            .field("turnkey", &self.turnkey)
            .field("password", &self.password)
            .field("master_password", &self.master_password)
            .field("body", &self.body)
            .field("status", &self.status)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_game_1() {
        use crate::files::TwoH;
        for entry in std::fs::read_dir("./test_data/game_1").unwrap() {
            let entry = entry.unwrap();
            let f = std::fs::File::open(entry.path()).unwrap();
            let ftherlnd =
                TwoH::read_contents(f).unwrap();
            println!("{:?}", entry.path());
            println!("{:#?}", ftherlnd);
        }
    }
}

#[derive(Debug)]
pub struct OrdersContents {
    pub pretender_id: u16,
    pub pretender_name: String,
}
#[derive(Debug)]
pub struct TrnContents {}

#[derive(Debug)]
pub enum FileBody {
    TurnFile(TrnContents),
    OrdersFile(OrdersContents),
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
        let file_type = file.read_u32::<LittleEndian>().unwrap();
        let _unk = file.read_u32::<LittleEndian>().unwrap();
        let nationid = file.read_i32::<LittleEndian>().unwrap();
        let status = file.read_i32::<LittleEndian>().unwrap();
        file.read_i32::<LittleEndian>().unwrap();
        let gamename = file.read_domstring().unwrap();
        let password = file.read_domsecret().unwrap();
        let masterpass = file.read_domsecret().unwrap();
        let turnkey = file.read_u32::<LittleEndian>().unwrap();
        let body = match file_type {
            1 => FileBody::TurnFile(TrnContents {}),
            0 => {
                let mut unk: [u8; 40] = [0; 40];
                file.read_exact(&mut unk).unwrap();
                let pretender_id = file.read_u16::<LittleEndian>().unwrap();
                let mut unk: [u8; 45] = [0; 45];
                file.read_exact(&mut unk).unwrap();
                let mut name = vec![];
                while let c @ 1..=255 = file.read_u8().unwrap() ^ 79 {
                    name.push(c);
                }
                let mut remainder = vec![];
                file.read_to_end(&mut remainder).unwrap();
                FileBody::OrdersFile(OrdersContents {
                    pretender_name: String::from_utf8(name).unwrap_or("".to_string()),
                    pretender_id,
                })
            }
            unk @ _ => panic!("Unknown filetype {:?}", unk),
        };

        let mut contents = vec![];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_to_end(&mut contents).unwrap();

        Some(Self {
            gamename,
            turnnumber,
            cdkey,
            nationid,
            turnkey,
            status,
            file_contents: contents,
            password,
            master_password: masterpass,
            body,
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
