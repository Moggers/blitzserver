use super::schema::{game_mods, files, games, maps, mods, nations, player_turns, players, turns};
use std::hash::{Hash, Hasher};

pub struct Era;
impl Era {
    pub const EARLY: i32 = 1;
    pub const MIDDLE: i32 = 2;
    pub const LATE: i32 = 3;
}

#[derive(Identifiable, Debug, Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub era: i32,
    pub map_id: i32,
    pub port: Option<i32>,
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
    pub map_id: i32,
}

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub filebinary: Vec<u8>,
    pub hash: i64,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub filename: &'a str,
    pub filebinary: &'a [u8],
    pub hash: i64,
}

impl<'a> NewFile<'a> {
    pub fn new(filename: &'a str, filebinary: &'a [u8]) -> NewFile<'a> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        filename.hash(&mut hasher);
        filebinary.hash(&mut hasher);
        NewFile {
            filename,
            filebinary,
            hash: hasher.finish() as i64,
        }
    }
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
    pub archive_id: i32,
}

#[derive(Insertable)]
#[table_name = "maps"]
pub struct NewMap {
    pub name: String,
    pub mapfile_id: i32,
    pub tgafile_id: i32,
    pub winterfile_id: i32,
    pub archive_id: i32
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Game)]
#[table_name = "players"]
pub struct Player {
    pub id: i32,
    pub nationid: i32,
    pub game_id: i32,
    pub file_id: i32,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub nationid: i32,
    pub game_id: i32,
    pub file_id: i32,
}

#[derive(Identifiable, Queryable)]
#[table_name = "nations"]
pub struct Nation {
    pub id: i32,
    pub game_id: i32,
    pub nation_id: i32,
    pub name: String,
    pub epithet: String,
}

#[derive(Insertable)]
#[table_name = "nations"]
pub struct NewNation {
    pub game_id: i32,
    pub name: String,
    pub nation_id: i32,
    pub epithet: String,
}

#[derive(Identifiable, Queryable)]
pub struct Turn {
    id: i32,
    pub game_id: i32,
    pub turn_number: i32,
    pub file_id: i32,
}

#[derive(Insertable)]
#[table_name = "turns"]
pub struct NewTurn {
    pub game_id: i32,
    pub turn_number: i32,
    pub file_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(parent = "File", foreign_key = "trnfile_id")]
pub struct PlayerTurn {
    id: i32,
    pub turn_number: i32,
    pub nation_id: i32,
    pub game_id: i32,
    pub trnfile_id: i32,
    pub twohfile_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "player_turns"]
pub struct NewPlayerTurn {
    pub turn_number: i32,
    pub nation_id: i32,
    pub game_id: i32,
    pub trnfile_id: i32,
}

#[derive(Insertable)]
#[table_name = "mods"]
pub struct NewMod<'a> {
    pub dm_filename: &'a str,
    pub name: &'a str,
    pub file_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
pub struct Mod {
    pub id: i32,
    pub dm_filename: String,
    pub name: String,
    pub file_id: i32,
}

#[derive(Insertable)]
#[table_name = "game_mods"]
pub struct NewGameMod {
    pub game_id: i32,
    pub mod_id: i32
}

#[derive(Identifiable, Queryable, Associations)]
pub struct GameMod {
    id: i32,
    pub game_id: i32,
    pub mod_id: i32
}
