use byteorder::ReadBytesExt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Dom5ReadExtErr {
    #[error("Bad read")]
    BadRead(#[from] std::io::Error),
    #[error("Bad string")]
    BadString(#[from] std::str::Utf8Error)
}

pub trait ReadDom5Ext: std::io::Read {
    fn read_domstring(&mut self) -> Result<String, Dom5ReadExtErr> {
        let mut newstr = String::new();
        while let Ok(c) = self.read_u8() {
            if c ^ 0x4f == 0 {
                break;
            }
            newstr.push((c^0x4f).into());
        }
        Ok(newstr)
    }

    fn read_domsecret(&mut self) -> Result<String, Dom5ReadExtErr>{
        let mut newstr = String::new();
        let mut magic = 0x78;
        loop {
            let c: u8 = self.read_u8()? ^ magic;
            if c == 0 {
                break;
            }
            magic = (std::num::Wrapping(c) + std::num::Wrapping(magic)).0;
            newstr.push(c.into());
        }
        Ok(newstr)
    }
}

impl<R: std::io::Read + ?Sized> ReadDom5Ext for R {}
