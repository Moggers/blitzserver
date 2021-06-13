pub mod twoh;
pub mod header;
mod utils;

pub use header::Header;
pub use twoh::TwoH;

use utils::ReadDom5Ext;
use std::io::{Read, Seek};
use std::io;

pub struct SaveFile {
    header: Header
}


impl SaveFile {
    pub fn read_contents<R: Seek + Read>(mut file: R) -> Result<Self, io::Error> {
        let header = Header::read_contents(file).unwrap();
        Ok(Self {
            header
        })
    }
}
