use std::io::BufRead;
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

enum TerrainFlags {
    SmallProvince = 1,
    LargeProvince = 2,
    Sea = 4,
    Freshwater = 8,
    Highlands = 16,
    Swamp = 32,
    Waste = 64,
    Forest = 128,
    Farm = 256,
    Nostart = 512,
    ManySites = 1024,
    DeepSea = 2048,
    Cave = 4096,
    Mountains = 4194304,
    GoodThroneLocation = 16777216,
    GoodStartLocation = 33554432,
    BadThroneLocation = 67108864,
    Warmer = 536870912,
    Colder = 1073741824,
}

impl std::convert::TryFrom<&[u8]> for MapFile {
    type Error = Box<dyn std::error::Error>;
    fn try_from(contents: &[u8]) -> Result<Self, Self::Error> {
        let commented_regex = regex::Regex::new(r#"^ *--"#).unwrap();
        let nocomment_lines = contents.lines().filter_map(Result::ok).filter(|l| !commented_regex.is_match(l)).collect::<Vec<String>>().join("\n");
        let tga_name = regex::Regex::new(r#"#imagefile "?([^"\n]+)"?"#)
            .ok()
            .and_then(|c| c.captures(&nocomment_lines))
            .and_then(|m| m.get(1))
            .and_then(|d| Some(d.as_str().to_owned()))
            .ok_or(MapFileReadError {
                err_type: MapFileReadErrorType::NoImageFile,
            })?;
        let name = regex::Regex::new(r#"#dom2title "?([^"\n]+)"?"#)
            .ok()
            .and_then(|c| c.captures(&nocomment_lines))
            .and_then(|m| m.get(1))
            .and_then(|d| Some(d.as_str().to_owned()))
            .ok_or(MapFileReadError {
                err_type: MapFileReadErrorType::NoName,
            })?;
        let winter_filename = regex::Regex::new(r#"#winterimagefile "?([^"\n]+)"?"#)
            .ok()
            .and_then(|c| c.captures(&nocomment_lines))
            .and_then(|m| m.get(1))
            .and_then(|d| Some(d.as_str().to_owned()));

        let mut province_count = 0;
        let mut uw_count = 0;
        let capture_terrain = regex::Regex::new("#terrain ([0-9]+) ([0-9]+)").unwrap();
        for (_, terrain_type) in contents.lines().filter_map(Result::ok).filter_map(|l| {
            capture_terrain.captures(&l).and_then(|c| {
                Some((
                    c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ))
            })
        }) {
            if terrain_type & TerrainFlags::Sea as i32 == TerrainFlags::Sea as i32 {
                uw_count = uw_count + 1;
            } else {
                province_count = province_count + 1;
            }
        }

        Ok(MapFile {
            map_name: name,
            tga_filename: tga_name,
            winter_filename: winter_filename,
            prov_count: province_count,
            uwprov_count: uw_count,
        })
    }
}
