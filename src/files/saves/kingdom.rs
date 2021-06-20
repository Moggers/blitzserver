use super::utils::ReadDom5Ext;
use super::DomSaveReadError;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Kingdom {
    nation_id: u16,
    is_ai: u8,
    name: String,
}

impl Kingdom {
    pub fn read_contents<R: std::io::Read>(mut file: R) -> Result<Option<Self>, DomSaveReadError> {
        let nation_id = file.read_u16::<LittleEndian>()?;
        if nation_id == std::u16::MAX {
            return Ok(None);
        }
        file.read_u16::<LittleEndian>()?;
        let magic = file.read_u16::<LittleEndian>()?;
        if magic != 12546 {
            // return Err(DomSaveReadError::BadMagic((magic.into(), 12546)));
        }
        let mut unk = [0u8; 16];
        file.read_exact(&mut unk)?;
        let is_ai  =file.read_u8()?;
        let mut unk = [0u8; 367];
        file.read_exact(&mut unk)?;
        let name = file.read_domstring()?;
        let mut unk = [0u8; 1693];
        file.read_exact(&mut unk)?;

        Ok(Some(Self { nation_id, is_ai, name }))
    }
}
