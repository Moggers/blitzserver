use super::schema::files;
use super::schema::games;
use super::schema::maps;

pub struct Era;
impl Era {
    pub const EARLY: i32 = 1;
    pub const MIDDLE: i32 = 2;
    pub const LATE: i32 = 3;
}

#[derive(Debug, Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub era: i32,
    pub map_id: i32,
    pub port: Option<i32>
}

impl Game {
    pub fn era_name(&self) -> String {
        match self.era {
            Era::EARLY => "Early".to_string(),
            Era::MIDDLE => "Middle".to_string(),
            Era::LATE => "Late".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame<'a> {
    pub name: &'a str,
    pub era: i32,
    pub map_id: i32
}

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub filebinary: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub filename: &'a str,
    pub filebinary: &'a [u8],
}

// YUCK! Newtype hack to allow multiple relations from map to filew without colliding
pub struct MapFile(pub File);
pub struct TgaFile(pub File);
pub struct WinterFile(pub File);

#[derive(Debug, Queryable, Associations)]
#[belongs_to(parent = "MapFile", foreign_key = "mapfile_id")]
#[belongs_to(parent = "TgaFile", foreign_key = "tgafile_id")]
#[belongs_to(parent = "WinterFile", foreign_key = "winterfile_id")]
#[table_name = "maps"]
pub struct Map {
    pub id: i32,
    pub name: String,
    pub mapfile_id: i32,
    pub tgafile_id: i32,
    pub winterfile_id: i32,
}

#[derive(Insertable)]
#[table_name = "maps"]
pub struct NewMap {
    pub name: String,
    pub mapfile_id: i32,
    pub tgafile_id: i32,
    pub winterfile_id: i32,
}
