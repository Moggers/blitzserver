use super::schema::{
    email_configs, files, game_mods, games, maps, mods, nations, player_turns, players, turns,
};
use crate::diesel::RunQueryDsl;
use serde::Deserialize;
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
    pub timer: Option<i32>,
    pub thrones_t1: i32,
    pub thrones_t2: i32,
    pub thrones_t3: i32,
    pub throne_points_required: i32,
    pub research_diff: i32,
    pub research_rand: bool,
    pub hof_size: i32,
    pub global_size: i32,
    pub indepstr: i32,
    pub magicsites: i32,
    pub eventrarity: i32,
    pub richness: i32,
    pub resources: i32,
    pub recruitment: i32,
    pub supplies: i32,
    pub startprov: i32,
    pub renaming: bool,
    pub scoregraphs: bool,
    pub nationinfo: bool,
    pub artrest: bool,
    pub teamgame: bool,
    pub clustered: bool,
    pub storyevents: i32,
    pub newailvl: i32,
    pub newai: bool,
    pub next_turn: Option<std::time::SystemTime>,
    pub password: String,
}

#[derive(QueryableByName)]
struct GameNationCount {
    #[sql_type = "diesel::sql_types::Integer"]
    pub game_id: i32,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
}

impl Game {
    pub fn next_turn_string(&self) -> String {
        match self.next_turn {
            None => "When these shazbots submit their turns".to_string(),
            Some(next_turn) => {
                if let Ok(until) = next_turn.duration_since(std::time::SystemTime::now()) {
                    format!(
                        "{}{}",
                        if until.as_secs() > (60 * 60) {
                            // More than one hour
                            format!("{} hours, ", (until.as_secs() as f32 / 60.0 / 60.0).floor())
                        } else {
                            "".to_string()
                        },
                        if until.as_secs() % (60 * 60) > 0 {
                            // More than one minute within the hour
                            format!(
                                "{} minutes",
                                (until.as_secs() as f32 % (60.0 * 60.0) / 60.0).floor(),
                            )
                        } else {
                            "".to_string()
                        }
                    )
                } else {
                    "The past".to_string()
                }
            }
        }
    }
    pub fn timer_string(&self) -> String {
        match self.timer {
            None => "".to_owned(),
            Some(t) => t.to_string(),
        }
    }
    pub fn era_name(&self) -> String {
        match self.era {
            Era::EARLY => "Early".to_string(),
            Era::MIDDLE => "Middle".to_string(),
            Era::LATE => "Late".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn get_player_count<D>(games: Vec<i32>, db: &D) -> std::collections::HashMap<i32, i32>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(
            "SELECT game_id, COUNT(*) count FROM players WHERE game_id = ANY($1) GROUP BY game_id",
        )
        .bind::<diesel::sql_types::Array<diesel::sql_types::Integer>, _>(games)
        .get_results::<GameNationCount>(db)
        .unwrap()
        .iter()
        .fold(std::collections::HashMap::new(), |mut hm, val| {
            hm.insert(val.game_id, val.count as i32);
            hm
        })
    }
}

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub name: String,
    pub era: i32,
    pub map_id: i32,
    pub thrones_t1: i32,
    pub thrones_t2: i32,
    pub thrones_t3: i32,
    pub throne_points_required: i32,
    pub research_diff: i32,
    pub research_rand: bool,
    pub hof_size: i32,
    pub global_size: i32,
    pub indepstr: i32,
    pub magicsites: i32,
    pub eventrarity: i32,
    pub richness: i32,
    pub resources: i32,
    pub recruitment: i32,
    pub supplies: i32,
    pub startprov: i32,
    pub renaming: bool,
    pub scoregraphs: bool,
    pub nationinfo: bool,
    pub artrest: bool,
    pub teamgame: bool,
    pub clustered: bool,
    pub storyevents: i32,
    pub newailvl: i32,
    pub newai: bool,
    pub password: String
}
impl Default for NewGame {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            era: 1,
            map_id: 1,
            thrones_t1: 5,
            thrones_t2: 0,
            thrones_t3: 0,
            throne_points_required: 5,
            research_diff: 2,
            research_rand: true,
            hof_size: 10,
            global_size: 5,
            indepstr: 5,
            magicsites: 55,
            eventrarity: 1,
            richness: 100,
            resources: 100,
            recruitment: 100,
            supplies: 100,
            startprov: 1,
            renaming: true,
            scoregraphs: false,
            nationinfo: true,
            artrest: false,
            teamgame: false,
            clustered: false,
            storyevents: 1,
            newailvl: 2,
            newai: true,
            password: "password".to_string()
        }
    }
}

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub filebinary: Vec<u8>,
    pub hash: i64,
}

impl<'a> NewFile<'a> {
    pub fn insert<'b>(
        self,
        db: &'b r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    ) -> File {
        use super::diesel::prelude::*;
        use crate::schema::files::dsl::*;
        match files.filter(hash.eq(self.hash)).get_result(db) {
            Ok(f) => f,
            Err(_) => diesel::insert_into(files)
                .values(self)
                .on_conflict(hash)
                .do_update()
                .set(filename.eq(filename)) // Bogus update so return row gets populated with existing stuff
                .get_result(db)
                .unwrap(),
        }
    }
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
    pub winterfile_id: Option<i32>,
    pub archive_id: i32,
}

#[derive(Insertable)]
#[table_name = "maps"]
pub struct NewMap {
    pub name: String,
    pub mapfile_id: i32,
    pub tgafile_id: i32,
    pub winterfile_id: Option<i32>,
    pub archive_id: i32,
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

#[derive(Debug, Associations, Identifiable, Queryable, QueryableByName)]
#[belongs_to(parent = "Game", foreign_key = "game_id")]
#[table_name = "turns"]
pub struct Turn {
    id: i32,
    pub game_id: i32,
    pub turn_number: i32,
    pub file_id: i32,
    pub created_at: std::time::SystemTime,
}

#[derive(Clone, Deserialize, QueryableByName)]
pub struct TurnSummary {
    #[sql_type = "diesel::sql_types::Integer"]
    pub game_id: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    pub turn_number: i32,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub submitted: i64,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub total: i64,
}

impl Turn {
    pub fn current_turn<D>(game_ids: &[i32], db: D) -> Vec<Turn>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(
            format!("SELECT * FROM turns t INNER JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE game_id IN ({}) GROUP BY game_id ) as mt ON mt.turn_number = t.turn_number AND mt.game_id = t.game_id", game_ids.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")),
        ).get_results(&db).unwrap()
    }
    pub fn turn_summary<D>(game_ids: &[i32], db: &D) -> Vec<TurnSummary>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(
            format!("\
SELECT t.game_id, t.turn_number, COALESCE(pt.submitted, 0) as submitted, COALESCE(pt.total, 0) as total
FROM turns t 
INNER JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE game_id IN ({}) GROUP BY game_id ) as mt 
    ON mt.turn_number = t.turn_number AND mt.game_id = t.game_id
LEFT JOIN (SELECT game_id,turn_number,COUNT(twohfile_id) submitted, COUNT(trnfile_id) total FROM player_turns GROUP BY game_id, turn_number) as pt
    ON pt.turn_number = t.turn_number AND pt.game_id = t.game_id\
", game_ids.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")),
        ).get_results(db).unwrap()
    }
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
    pub icon_file_id: Option<i32>,
}

#[derive(Clone, Identifiable, Queryable, Associations)]
pub struct Mod {
    pub id: i32,
    pub dm_filename: String,
    pub name: String,
    pub file_id: i32,
    pub icon_file_id: Option<i32>,
}
#[derive(Clone)]
pub struct ModDefinition {
    pub dm_filename: String,
    pub icon_filename: String,
    pub name: String
}

#[derive(Insertable)]
#[table_name = "game_mods"]
pub struct NewGameMod {
    pub game_id: i32,
    pub mod_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
pub struct GameMod {
    id: i32,
    pub game_id: i32,
    pub mod_id: i32,
}

#[derive(QueryableByName, Debug, Identifiable, Queryable)]
#[table_name = "email_configs"]
pub struct EmailConfig {
    pub id: i32,
    pub nation_id: i32,
    pub game_id: i32,
    pub hours_before_host: i32,
    pub email_address: String,
    pub last_turn_notified: Option<i32>,
    pub subject: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "email_configs"]
pub struct NewEmailConfig {
    pub nation_id: i32,
    pub game_id: i32,
    pub hours_before_host: i32,
    pub email_address: String,
    pub subject: String,
    pub body: String,
}
