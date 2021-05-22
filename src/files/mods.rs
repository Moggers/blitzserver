use crate::models::{ModDefReadError, ModDefinition, NewFileOwned, NewModOwned};
use std::io::Read;
use thiserror::Error;
use zip::{result::ZipError, ZipArchive};

struct ModPackage {
    moddef: NewModOwned,
    banner: Option<NewFileOwned>,
    archive: NewFileOwned,
}

#[derive(Error, Debug)]
pub enum ModExtractError {
    #[error("Not a valid zip")]
    InvalidArchive(#[from] ZipError),
    #[error("No dm file")]
    MissingDm,
    #[error("Dm file malformed")]
    InvalidDm(#[from] ModDefReadError),
}
pub fn extract_zip<R>(filename: &str, file: R) -> Result<ModPackage, ModExtractError>
where
    R: std::io::Read + std::io::Seek,
{
    let mut archive = ZipArchive::new(file)?;
    let mut moddef = None;
    for i in 0..archive.len() {
        let mut archive_file = archive.by_index(i).unwrap();
        if archive_file.is_file()
            && std::path::PathBuf::from(archive_file.name()).extension()
                == Some(std::ffi::OsStr::new("dm"))
        {
            let mut contents: String = String::new();
            archive_file.read_to_string(&mut contents).unwrap();
            moddef = Some(ModDefinition::from_str(archive_file.name(), &contents)?);
            break;
        }
    }
    let moddef = if let Some(moddef) = moddef {
        moddef
    } else {
        return Err(ModExtractError::MissingDm);
    };

    Ok(ModPackage {
        moddef: NewModOwned {
            dm_filename: moddef.dm_filename,
            name: moddef.name,
            file_id: 0,
            icon_file_id: moddef.icon_filename.map(|_| 0),
        },
        banner: None,
        archive: ,
    })
}
