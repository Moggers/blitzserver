pub struct MapFile {
    pub map_name: String,
    pub tga_filename: String,
    pub winter_filename: Option<String>,
    pub prov_count: i32,
    pub uwprov_count: i32,
}

#[derive(Debug)]
enum MapFileReadErrorType {
    NoImageFile,
    NoName,
}
#[derive(Debug)]
struct MapFileReadError {
    err_type: MapFileReadErrorType,
}

impl std::fmt::Display for MapFileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "No valid {} in map file",
            match self.err_type {
                MapFileReadErrorType::NoImageFile => "#imagefile",
                MapFileReadErrorType::NoName => "#dom2title",
            }
        )
    }
}

impl std::error::Error for MapFileReadError {}

impl std::convert::TryFrom<&[u8]> for MapFile {
    type Error = Box<dyn std::error::Error>;
    fn try_from(contents: &[u8]) -> Result<Self, Self::Error> {
        let tga_name = regex::bytes::Regex::new(r#"#imagefile ("?[^"\n]+"?)"#)
            .ok()
            .and_then(|c| c.captures(&contents))
            .and_then(|m| m.get(1))
            .and_then(|d| String::from_utf8(d.as_bytes().to_vec()).ok())
            .ok_or(MapFileReadError {
                err_type: MapFileReadErrorType::NoImageFile,
            })?;
        let name = regex::bytes::Regex::new(r#"#dom2title ("?[^"\n]+"?)"#)
            .ok()
            .and_then(|c| c.captures(&contents))
            .and_then(|m| m.get(1))
            .and_then(|d| String::from_utf8(d.as_bytes().to_vec()).ok())
            .ok_or(MapFileReadError {
                err_type: MapFileReadErrorType::NoName,
            })?;
        let winter_filename = regex::bytes::Regex::new(r#"#winterimagefile ("?[^"\n]+"?)"#)
            .ok()
            .and_then(|c| c.captures(&contents))
            .and_then(|m| m.get(1))
            .and_then(|d| String::from_utf8(d.as_bytes().to_vec()).ok());

        Ok(MapFile {
            map_name: name,
            tga_filename: tga_name,
            winter_filename: winter_filename,
            prov_count: 0,
            uwprov_count: 0,
        })
    }
}
