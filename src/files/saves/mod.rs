pub mod header;
pub mod land;
pub mod kingdom;
pub mod trn_contents;
pub mod twoh_contents;
mod utils;

use std::io::{Read, Seek};
use thiserror::Error;
use utils::ReadDom5Ext;

pub use header::Header;
pub use trn_contents::TrnContents;
pub use twoh_contents::TwoHContents;

#[derive(Error, Debug)]
pub enum DomSaveReadError {
    #[error("IO Error")]
    ReadError(#[from] std::io::Error),
    #[error("Bad magic")]
    BadMagic((u64, u64)),
    #[error("Invalid filetype")]
    BadFileType(u32),
}

#[derive(Debug)]
pub enum SaveBody {
    TwoHContents(TwoHContents),
    TrnContents(TrnContents),
}

#[derive(Debug)]
pub struct SaveFile {
    pub header: Header,
    pub body: SaveBody,
}

impl SaveFile {
    pub fn read_contents<R: Seek + Read>(mut file: R) -> Result<Self, DomSaveReadError> {
        let header = Header::read_contents(&mut file)?;
        Ok(Self {
            body: match header.file_type {
                0 => SaveBody::TwoHContents(TwoHContents::read_contents(&mut file)?),
                1 => SaveBody::TrnContents(TrnContents::read_contents(&mut file)?),
                _ => return Err(DomSaveReadError::BadFileType(header.file_type)),
            },
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_game_1() {
        use crate::files::saves::SaveFile;
        for entry in std::fs::read_dir("./test_data/game_2").unwrap() {
            let entry = entry.unwrap();
            for entry in std::fs::read_dir(entry.path()).unwrap() {
                let entry = entry.unwrap();
                if entry.file_name() == "ftherlnd" {
                    let f = std::fs::File::open(entry.path()).unwrap();
                    let ftherlnd = SaveFile::read_contents(f).unwrap();
                    println!("{:?}", entry.path());
                    println!("{:#?}", ftherlnd);
                }
            }
        }
    }
}
