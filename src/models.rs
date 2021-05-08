use super::schema::{
    admin_logs, disciples, discord_configs, email_configs, files, game_logs, game_mods, games,
    maps, mods, nations, player_turns, players, turns,
};
use crate::diesel::OptionalExtension;
use crate::diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use crate::diesel::{JoinOnDsl, RunQueryDsl};
use serde::Deserialize;
use std::hash::{Hash, Hasher};

pub struct Era;
impl Era {
    pub const EARLY: i32 = 1;
    pub const MIDDLE: i32 = 2;
    pub const LATE: i32 = 3;
}

#[derive(QueryableByName, Identifiable, Debug, Queryable)]
#[table_name = "games"]
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
    pub next_turn: Option<std::time::SystemTime>,
    pub password: String,
    pub archived: bool,
    pub masterpass: Option<String>,
    pub private: bool,
}

#[derive(QueryableByName)]
struct GameNationCount {
    #[sql_type = "diesel::sql_types::Integer"]
    pub game_id: i32,
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
}

impl Game {
    pub fn get_next_wakeup<D>(
        db: &D,
    ) -> Result<Option<std::time::SystemTime>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        games_dsl::games
            .select(games_dsl::next_turn)
            .filter(
                diesel::dsl::not(games_dsl::next_turn.is_null()).and(games_dsl::archived.eq(false)),
            )
            .order(games_dsl::next_turn.asc())
            .limit(1)
            .get_result(db)
    }
    pub fn get_due_games<D>(db: &D) -> Result<Vec<Game>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(
            "SELECT g.*
            FROM games g
            WHERE g.archived = FALSE AND g.next_turn IS NOT NULL AND g.next_turn <= NOW()",
        )
        .get_results(db)
    }
    pub fn get_by_name<D>(name: &str, db: &D) -> Result<Game, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        games_dsl::games
            .filter(games_dsl::name.eq(name).and(games_dsl::archived.eq(false)))
            .get_result(db)
    }
    pub fn get_public<D>(db: &D) -> Result<Vec<Game>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        games_dsl::games
            .filter(games_dsl::private.eq(false))
            .get_results(db)
    }
    pub fn get_mods<D>(&self, db: &D) -> Result<Vec<Mod>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::game_mods::dsl as gm_dsl;
        use crate::schema::mods::dsl as mods_dsl;
        let gamemods: Vec<(GameMod, Mod)> = gm_dsl::game_mods
            .filter(gm_dsl::game_id.eq(self.id))
            .inner_join(mods_dsl::mods.on(mods_dsl::id.eq(gm_dsl::mod_id)))
            .get_results(db)?;
        Ok(gamemods.into_iter().map(|(_, m)| m).collect())
    }
    pub fn get<D>(game_id: i32, db: &D) -> Result<Game, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        games_dsl::games
            .filter(games_dsl::id.eq(game_id))
            .get_result(db)
    }

    pub fn schedule_turn<D>(
        &self,
        next_turn: std::time::SystemTime,
        db: &D,
    ) -> Result<Game, crate::diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        diesel::update(games_dsl::games.filter(games_dsl::id.eq(self.id)))
            .set(games_dsl::next_turn.eq(next_turn))
            .get_result(db)
    }
    pub fn next_turn_string(&self) -> String {
        if self.archived {
            return "Never".to_string();
        }
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

    pub fn unstart<D>(&self, db: &D) -> Result<(), diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::player_turns::dsl as pt_dsl;
        use crate::schema::turns::dsl as turns_dsl;
        diesel::update(turns_dsl::turns)
            .filter(turns_dsl::game_id.eq(self.id))
            .set(turns_dsl::archived.eq(true))
            .execute(db)?;
        diesel::update(pt_dsl::player_turns)
            .filter(pt_dsl::game_id.eq(self.id))
            .set(pt_dsl::archived.eq(true))
            .execute(db)?;
        NewAdminLog {
            game_id: self.id,
            datetime: std::time::SystemTime::now(),
            action: "Returned game to lobby".to_string(),
        }
        .insert(db)?;
        Ok(())
    }

    pub fn rollback<D>(&self, db: &D) -> Result<Turn, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::player_turns::dsl as pt_dsl;
        use crate::schema::turns::dsl as turns_dsl;
        let turn: Turn = turns_dsl::turns
            .filter(
                turns_dsl::game_id
                    .eq(self.id)
                    .and(turns_dsl::archived.eq(false)),
            )
            .order(turns_dsl::turn_number.desc())
            .limit(1)
            .get_result(db)?;
        if turn.turn_number == 1 {
            return Ok(turn);
        }
        let turn: Turn = diesel::update(turns_dsl::turns)
            .filter(
                turns_dsl::game_id
                    .eq(self.id)
                    .and(turns_dsl::turn_number.eq(turn.turn_number)),
            )
            .set(turns_dsl::archived.eq(true))
            .get_result(db)?;
        diesel::update(pt_dsl::player_turns)
            .filter(
                pt_dsl::game_id
                    .eq(self.id)
                    .and(pt_dsl::turn_number.eq(turn.turn_number)),
            )
            .set(pt_dsl::archived.eq(true))
            .execute(db)?;
        diesel::update(pt_dsl::player_turns)
            .filter(
                pt_dsl::game_id
                    .eq(self.id)
                    .and(pt_dsl::status.eq(2))
                    .and(pt_dsl::turn_number.eq(turn.turn_number - 1)),
            )
            .set(pt_dsl::status.eq(1))
            .execute(db)?;
        NewAdminLog {
            game_id: self.id,
            datetime: std::time::SystemTime::now(),
            action: format!(
                "Rolled back turn {}->{}",
                turn.turn_number,
                turn.turn_number - 1
            ),
        }
        .insert(db)?;
        Ok(turn)
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

    pub fn remove_timer<D>(self, db: &D) -> Result<Self, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        diesel::update(games_dsl::games)
            .filter(games_dsl::id.eq(self.id))
            .set(games_dsl::next_turn.eq::<Option<std::time::SystemTime>>(None))
            .get_result::<Game>(db)
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
    pub password: String,
    pub masterpass: Option<String>,
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
            artrest: true,
            teamgame: false,
            clustered: false,
            storyevents: 1,
            newailvl: 2,
            password: "password".to_string(),
            masterpass: None,
        }
    }
}

#[derive(Clone, Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub filebinary: Vec<u8>,
    pub hash: i64,
}

impl<'a> NewFile<'a> {
    pub fn insert<D>(self, db: &D) -> File
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
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

#[derive(Debug, Clone, Queryable, Associations)]
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
    pub province_count: i32,
    pub uw_count: i32,
}

impl Map {
    pub fn get<D>(id: i32, db: &D) -> Result<Self, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::maps::dsl as maps_dsl;
        maps_dsl::maps.filter(maps_dsl::id.eq(id)).get_result(db)
    }

    pub fn get_files<D>(&self, db: &D) -> Result<(File, File, Option<File>), diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        let ids = match self.winterfile_id {
            Some(wid) => vec![self.tgafile_id, self.mapfile_id, wid],
            None => vec![self.tgafile_id, self.mapfile_id],
        };
        let files: Vec<File> = files_dsl::files
            .filter(files_dsl::id.eq_any(&ids))
            .get_results(db)?;
        return Ok((
            (*files.iter().find(|f| f.id == self.mapfile_id).unwrap()).clone(),
            (*files.iter().find(|f| f.id == self.tgafile_id).unwrap()).clone(),
            match self.winterfile_id {
                None => None,
                Some(fid) => Some((*files.iter().find(|f| f.id == fid).unwrap()).clone()),
            },
        ));
    }
}

#[derive(Debug, Insertable)]
#[table_name = "maps"]
pub struct NewMap {
    pub name: String,
    pub mapfile_id: i32,
    pub tgafile_id: i32,
    pub winterfile_id: Option<i32>,
    pub province_count: i32,
    pub uw_count: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Game)]
#[table_name = "players"]
pub struct Player {
    pub id: i32,
    pub nationid: i32,
    pub game_id: i32,
    pub file_id: i32,
    pub name: String,
}

impl Player {
    pub fn get_players<D>(game_id: i32, db: &D) -> Result<Vec<Player>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::players::dsl as players_dsl;
        players_dsl::players
            .filter(players_dsl::game_id.eq(game_id))
            .get_results(db)
    }

    pub fn get_newlord<D>(&self, db: &D) -> Result<File, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        files_dsl::files
            .filter(files_dsl::id.eq(self.file_id))
            .get_result(db)
    }

    pub fn remove<D>(&self, db: &D) -> Result<(), diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::players::dsl as p_dsl;
        diesel::delete(p_dsl::players)
            .filter(p_dsl::id.eq(self.id))
            .execute(db)?;
        let nation = Nation::get(self.game_id, self.nationid, db)?;
        NewAdminLog {
            game_id: self.game_id,
            datetime: std::time::SystemTime::now(),
            action: format!("Removed {}", nation.name),
        }
        .insert(db)?;
        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub nationid: i32,
    pub game_id: i32,
    pub file_id: i32,
    pub name: String,
}

impl NewPlayer {
    pub fn insert<D>(&self, db: &D) -> Result<Player, crate::diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::players::dsl::*;
        diesel::insert_into(super::schema::players::table)
            .values(self)
            .on_conflict((game_id, nationid))
            .do_update()
            .set((file_id.eq(self.file_id), name.eq(&self.name)))
            .get_result(db)
    }
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "nations"]
pub struct Nation {
    pub id: i32,
    pub game_id: i32,
    pub nation_id: i32,
    pub name: String,
    pub epithet: String,
    pub filename: String,
}

impl Nation {
    pub fn get<D>(game_id: i32, nation_id: i32, db: &D) -> Result<Nation, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::nations::dsl as nations_dsl;
        nations_dsl::nations
            .filter(
                nations_dsl::game_id
                    .eq(game_id)
                    .and(nations_dsl::nation_id.eq(nation_id)),
            )
            .get_result(db)
    }
    pub fn get_all<D>(game_id: i32, db: &D) -> Result<Vec<Nation>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::nations::dsl as nations_dsl;
        nations_dsl::nations
            .filter(nations_dsl::game_id.eq(game_id))
            .get_results(db)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "nations"]
pub struct NewNation {
    pub game_id: i32,
    pub name: String,
    pub nation_id: i32,
    pub epithet: String,
    pub filename: String,
}

impl NewNation {
    pub fn insert<D>(&self, db: &D) -> Result<Nation, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::nations::dsl as nations_dsl;
        diesel::insert_into(nations_dsl::nations)
            .values(self)
            .on_conflict((nations_dsl::game_id, nations_dsl::nation_id))
            .do_update()
            .set((
                (nations_dsl::name.eq(&self.name)),
                (nations_dsl::epithet.eq(&self.epithet)),
                (nations_dsl::filename.eq(&self.filename)),
            ))
            .get_result(db)
    }
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
    pub archived: bool,
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
    pub fn get_player_turns<D>(&self, db: &D) -> Result<Vec<PlayerTurn>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::player_turns::dsl as pt_dsl;
        pt_dsl::player_turns
            .filter(
                pt_dsl::game_id
                    .eq(self.game_id)
                    .and(pt_dsl::archived.eq(false))
                    .and(pt_dsl::turn_number.eq(self.turn_number)),
            )
            .get_results(db)
    }
    pub fn get_ftherlnd<D>(&self, db: &D) -> Result<File, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        files_dsl::files
            .filter(files_dsl::id.eq(self.file_id))
            .get_result(db)
    }
    pub fn get_all<D>(game_id: i32, db: &D) -> Result<Vec<Turn>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::turns::dsl as turns_dsl;
        turns_dsl::turns
            .filter(
                turns_dsl::game_id
                    .eq(game_id)
                    .and(turns_dsl::archived.eq(false)),
            )
            .get_results(db)
    }
    pub fn get<D>(game_id: i32, db: &D) -> Result<Turn, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::turns::dsl as turns_dsl;
        turns_dsl::turns
            .filter(
                turns_dsl::game_id
                    .eq(game_id)
                    .and(turns_dsl::archived.eq(false)),
            )
            .order(turns_dsl::turn_number.desc())
            .limit(1)
            .get_result(db)
    }
    pub fn current_turn<D>(game_ids: &[i32], db: &D) -> Vec<Turn>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(
            format!("SELECT * FROM turns t INNER JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE game_id IN ({}) AND archived = false GROUP BY game_id ) as mt ON mt.turn_number = t.turn_number AND mt.game_id = t.game_id", game_ids.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")),
        ).get_results(db).unwrap()
    }
    pub fn turn_summary<D>(game_ids: &[i32], db: &D) -> Vec<TurnSummary>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        if game_ids.len() == 0 {
            vec![]
        } else {
            diesel::sql_query(
            format!("\
SELECT t.game_id, t.turn_number, COALESCE(pt.submitted, 0) as submitted, COALESCE(pt.total, 0) as total
FROM turns t 
INNER JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE game_id IN ({}) AND archived = false GROUP BY game_id ) as mt 
    ON mt.turn_number = t.turn_number AND mt.game_id = t.game_id
LEFT JOIN (SELECT game_id,turn_number,COUNT(twohfile_id) submitted, COUNT(trnfile_id) total FROM player_turns WHERE archived = false GROUP BY game_id, turn_number) as pt
    ON pt.turn_number = t.turn_number AND pt.game_id = t.game_id\
", game_ids.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")),
        ).get_results(db).unwrap()
        }
    }
}

#[derive(Insertable)]
#[table_name = "turns"]
pub struct NewTurn {
    pub game_id: i32,
    pub turn_number: i32,
    pub file_id: i32,
}

impl NewTurn {
    pub fn insert<D>(&self, db: &D) -> Result<bool, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::turns::dsl as turns_dsl;
        let existing_turn = turns_dsl::turns
            .filter(
                turns_dsl::game_id
                    .eq(self.game_id)
                    .and(turns_dsl::turn_number.eq(self.turn_number))
                    .and(turns_dsl::archived.eq(false)),
            )
            .get_result::<Turn>(db);
        if !existing_turn.is_ok() || existing_turn.as_ref().unwrap().file_id != self.file_id {
            log::info!("Game {}: Creating turn {}", self.game_id, self.turn_number);
            diesel::sql_query(format!("INSERT INTO turns (game_id, turn_number, file_id) VALUES({}, {}, {}) ON CONFLICT (game_id, turn_number) WHERE archived IS false DO UPDATE SET file_id={}", self.game_id, self.turn_number, self.file_id, self.file_id))
            .execute(db).unwrap();
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

#[derive(Debug, Identifiable, QueryableByName, Queryable, Associations)]
#[table_name = "player_turns"]
#[belongs_to(parent = "File", foreign_key = "trnfile_id")]
pub struct PlayerTurn {
    id: i32,
    pub turn_number: i32,
    pub nation_id: i32,
    pub game_id: i32,
    pub trnfile_id: i32,
    pub twohfile_id: Option<i32>,
    pub archived: bool,
    pub status: i32,
}

impl PlayerTurn {
    pub fn get_2h<D>(&self, db: &D) -> Result<File, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        match self.twohfile_id {
            Some(thfid) => files_dsl::files
                .filter(files_dsl::id.eq(thfid))
                .get_result(db),
            None => Err(diesel::result::Error::NotFound),
        }
    }
    pub fn get_trn<D>(&self, db: &D) -> Result<File, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        files_dsl::files
            .filter(files_dsl::id.eq(self.trnfile_id))
            .get_result(db)
    }
    pub fn save_2h<D>(
        &self,
        twoh: NewFile,
        status: i32,
        db: &D,
    ) -> Result<PlayerTurn, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        let file = twoh.insert(db);
        use crate::schema::player_turns::dsl as pt_dsl;
        diesel::update(pt_dsl::player_turns)
            .filter(pt_dsl::id.eq(self.id))
            .set((pt_dsl::status.eq(status), pt_dsl::twohfile_id.eq(file.id)))
            .get_result(db)
    }
    pub fn get<D>(
        game_id: i32,
        nation_id: i32,
        db: &D,
    ) -> Result<(PlayerTurn, File), diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        use crate::schema::player_turns::dsl as pt_dsl;
        pt_dsl::player_turns
            .filter(
                pt_dsl::game_id
                    .eq(game_id)
                    .and(pt_dsl::nation_id.eq(nation_id))
                    .and(pt_dsl::archived.eq(false)),
            )
            .order(pt_dsl::turn_number.desc())
            .limit(1)
            .inner_join(files_dsl::files.on(files_dsl::id.eq(pt_dsl::trnfile_id)))
            .get_result(db)
    }
    pub fn get_player_turns<D>(
        game_id: i32,
        db: &D,
    ) -> std::collections::HashMap<i32, Vec<PlayerTurn>>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::player_turns::dsl as pt_dsl;
        let player_turns = pt_dsl::player_turns
            .filter(pt_dsl::game_id.eq(game_id).and(pt_dsl::archived.eq(false)))
            .order(pt_dsl::turn_number)
            .get_results(db)
            .unwrap();
        player_turns.into_iter().fold(
            std::collections::HashMap::new(),
            |mut hm: std::collections::HashMap<i32, Vec<PlayerTurn>>, pt: PlayerTurn| {
                if !hm.contains_key(&pt.nation_id) {
                    hm.insert(pt.nation_id, vec![]);
                }
                hm.get_mut(&pt.nation_id).unwrap().push(pt);
                hm
            },
        )
    }
}

#[derive(Insertable)]
#[table_name = "player_turns"]
pub struct NewPlayerTurn {
    pub turn_number: i32,
    pub nation_id: i32,
    pub game_id: i32,
    pub trnfile_id: i32,
}

impl NewPlayerTurn {
    pub fn insert<D>(&self, db: &D) -> Result<PlayerTurn, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(format!("INSERT INTO player_turns (turn_number, nation_id, game_id, trnfile_id) VALUES({}, {}, {}, {}) ON CONFLICT (game_id, turn_number, nation_id) WHERE archived IS false DO UPDATE SET trnfile_id={} RETURNING *", self.turn_number, self.nation_id, self.game_id, self.trnfile_id, self.trnfile_id))
            .get_result(db)
    }
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

impl Mod {
    pub fn get_archive<D>(&self, db: &D) -> Result<File, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::files::dsl as files_dsl;
        files_dsl::files
            .filter(files_dsl::id.eq(self.file_id))
            .get_result(db)
    }
}

#[derive(Clone)]
pub struct ModDefinition {
    pub dm_filename: String,
    pub icon_filename: String,
    pub name: String,
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

#[derive(QueryableByName, Debug)]
pub struct Wakeup {
    #[sql_type = "diesel::sql_types::Timestamp"]
    pub timestamp: std::time::SystemTime,
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
    pub is_reminder: bool,
}

impl EmailConfig {
    pub fn get_reminder_wakeup<D>(
        db: &D,
    ) -> Result<Option<std::time::SystemTime>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        Ok(diesel::sql_query(
            r#"
SELECT g.next_turn - interval '1' hour * ec.hours_before_host as timestamp
FROM email_configs ec
LEFT JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE archived = false GROUP BY game_id) t
    ON t.game_id=ec.game_id
INNER JOIN games g ON g.id=t.game_id AND g.archived = FALSE
LEFT OUTER JOIN (SELECT nation_id, game_id, MAX(turn_number) as turn_number FROM player_turns pt WHERE archived = false AND twohfile_id IS NOT NULL GROUP BY game_id,nation_id) pt
    ON pt.game_id=ec.game_id AND pt.turn_number = t.turn_number AND pt.nation_id = ec.nation_id
WHERE
    (ec.last_turn_notified IS NULL OR t.turn_number != ec.last_turn_notified)
    AND g.next_turn IS NOT NULL
    AND ec.is_reminder IS TRUE
    AND t.turn_number IS NOT NULL
ORDER BY timestamp DESC
LIMIT 1
        "#,
        )
        .get_result::<Wakeup>(db)
        .optional()?
        .and_then(|w| Some(w.timestamp)))
    }
    pub fn mark_sent<D>(&self, turn: i32, db: &D) -> Result<EmailConfig, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::email_configs::dsl as email_dsl;
        diesel::update(email_dsl::email_configs.filter(email_dsl::id.eq(self.id)))
            .set(email_dsl::last_turn_notified.eq(turn))
            .get_result(db)
    }
    pub fn get_due_reminders<D>(db: &D) -> Result<Vec<EmailConfig>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        // Fetch all email_configs which are reminder types attached to games which have at least
        // one turn, whose attached nation has not submitted orders for the current turn and the
        // timer has come to pass
        diesel::sql_query(r#"
SELECT ec.*
FROM email_configs ec
LEFT JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE archived = false GROUP BY game_id) t
    ON t.game_id=ec.game_id
INNER JOIN games g ON g.id=t.game_id AND g.archived = FALSE
LEFT OUTER JOIN (SELECT nation_id, game_id, MAX(turn_number) as turn_number FROM player_turns pt WHERE archived = false AND twohfile_id IS NOT NULL GROUP BY game_id,nation_id) pt
    ON pt.game_id=ec.game_id AND pt.turn_number = t.turn_number AND pt.nation_id = ec.nation_id
WHERE
    (ec.last_turn_notified IS NULL OR t.turn_number != ec.last_turn_notified)
    AND g.next_turn IS NOT NULL
    AND g.next_turn - interval '1' hour * ec.hours_before_host < NOW()
    AND ec.is_reminder IS TRUE
    AND t.turn_number IS NOT NULL
    "#).load(db)
    }

    pub fn delete<D>(id: i32, email_address: String, db: &D) -> Result<usize, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::email_configs::dsl as emails_dsl;
        diesel::delete(emails_dsl::email_configs)
            .filter(
                emails_dsl::email_address
                    .eq(email_address)
                    .and(emails_dsl::id.eq(id)),
            )
            .execute(db)
    }

    pub fn get_notifications<D>(
        game_id: i32,
        db: &D,
    ) -> Result<Vec<EmailConfig>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::email_configs::dsl as emails_dsl;
        emails_dsl::email_configs
            .filter(emails_dsl::game_id.eq(game_id))
            .get_results(db)
    }
    pub fn get_due_notifications<D>(db: &D) -> Result<Vec<EmailConfig>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query("\
SELECT ec.*
FROM email_configs ec
LEFT JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE archived = false GROUP BY game_id) t
    ON t.game_id=ec.game_id
INNER JOIN games g ON g.id=t.game_id AND g.archived = FALSE
LEFT OUTER JOIN (SELECT nation_id, game_id, MAX(turn_number) as turn_number FROM player_turns pt WHERE archived = false AND twohfile_id IS NOT NULL GROUP BY game_id,nation_id) pt
    ON pt.game_id=ec.game_id AND pt.turn_number = t.turn_number AND pt.nation_id = ec.nation_id
WHERE
    (ec.last_turn_notified IS NULL OR t.turn_number != ec.last_turn_notified)
    AND t.turn_number IS NOT NULL 
    AND g.next_turn IS NOT NULL
    AND ec.is_reminder = FALSE
    AND (ec.last_turn_notified IS NULL OR ec.last_turn_notified != t.turn_number)
    ").load(db)
    }
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
    pub is_reminder: bool,
}

impl NewEmailConfig {
    pub fn insert<D>(self, db: &D) -> Result<EmailConfig, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::models::email_configs::dsl as emails_dsl;
        diesel::insert_into(emails_dsl::email_configs)
            .values(self)
            .get_result(db)
    }
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "disciples"]
pub struct Disciple {
    id: i32,
    pub game_id: i32,
    pub nation_id: i32,
    pub is_disciple: i32,
    pub team: Option<i32>,
}

impl Disciple {
    pub fn get<D>(game_id: i32, nation_id: i32, db: &D) -> Result<Disciple, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        match d_dsl::disciples
            .filter(
                d_dsl::game_id
                    .eq(game_id)
                    .and(d_dsl::nation_id.eq(nation_id)),
            )
            .get_result(db)
        {
            Ok(d) => Ok(d),
            Err(diesel::result::Error::NotFound) => Ok(NewDisciple {
                game_id: game_id,
                nation_id: nation_id,
                is_disciple: 0,
                team: None,
            }
            .insert(db)?),
            Err(e) => Err(e),
        }
    }
    pub fn get_all<D>(game_id: i32, db: &D) -> Result<Vec<Disciple>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        d_dsl::disciples
            .filter(d_dsl::game_id.eq(game_id))
            .get_results(db)
    }

    pub fn remove<D>(&self, db: &D) -> Result<usize, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        diesel::delete(
            d_dsl::disciples.filter(
                d_dsl::nation_id
                    .eq(self.nation_id)
                    .and(d_dsl::game_id.eq(self.game_id)),
            ),
        )
        .execute(db)
    }

    pub fn create_team<D>(&self, db: &D) -> Result<Disciple, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        let new_team = match d_dsl::disciples
            .filter(
                d_dsl::game_id
                    .eq(self.game_id)
                    .and(diesel::dsl::not(d_dsl::team.is_null())),
            )
            .order(d_dsl::team.desc())
            .limit(1)
            .get_result::<Disciple>(db)
        {
            Err(_) => 1,
            Ok(biggest) => biggest.team.unwrap_or(0) + 1,
        };
        self.set_team(new_team, db)?.set_disc(0, db)
    }

    pub fn set_team<D>(&self, team: i32, db: &D) -> Result<Disciple, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        let make_disc = if d_dsl::disciples
            .filter(
                d_dsl::game_id
                    .eq(self.game_id)
                    .and(d_dsl::team.eq(team))
                    .and(d_dsl::is_disciple.eq(0)),
            )
            .get_results::<Disciple>(db)?
            .len()
            > 0
        {
            1
        } else {
            self.is_disciple
        };
        diesel::update(
            d_dsl::disciples.filter(
                d_dsl::game_id
                    .eq(self.game_id)
                    .and(d_dsl::nation_id.eq(self.nation_id)),
            ),
        )
        .set((d_dsl::team.eq(team), d_dsl::is_disciple.eq(make_disc)))
        .get_result(db)
    }
    pub fn unset_team<D>(&self, db: &D) -> Result<Disciple, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        diesel::update(
            d_dsl::disciples.filter(
                d_dsl::game_id
                    .eq(self.game_id)
                    .and(d_dsl::nation_id.eq(self.nation_id)),
            ),
        )
        .set(d_dsl::team.eq::<Option<i32>>(None))
        .get_result(db)
    }
    pub fn set_disc<D>(&self, is_disc: i32, db: &D) -> Result<Disciple, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        if is_disc == 0 && self.team.is_some() {
            diesel::update(
                d_dsl::disciples.filter(
                    d_dsl::game_id
                        .eq(self.game_id)
                        .and(d_dsl::team.eq(self.team.unwrap())),
                ),
            )
            .set(d_dsl::is_disciple.eq(1))
            .execute(db)?;
        }
        diesel::update(
            d_dsl::disciples.filter(
                d_dsl::game_id
                    .eq(self.game_id)
                    .and(d_dsl::nation_id.eq(self.nation_id)),
            ),
        )
        .set(d_dsl::is_disciple.eq(is_disc))
        .get_result(db)
    }
}

#[derive(Insertable)]
#[table_name = "disciples"]
pub struct NewDisciple {
    game_id: i32,
    nation_id: i32,
    is_disciple: i32,
    team: Option<i32>,
}

impl NewDisciple {
    pub fn insert<D>(&self, db: &D) -> Result<Disciple, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use disciples::dsl as d_dsl;
        diesel::insert_into(d_dsl::disciples)
            .values(self)
            .get_result(db)
    }
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "game_logs"]
pub struct GameLog {
    id: i32,
    game_id: i32,
    datetime: std::time::SystemTime,
    turn_number: i32,
    output: String,
    error: String,
}

#[derive(Queryable, Debug, Identifiable, QueryableByName)]
#[table_name = "game_logs"]
pub struct GameLogLite {
    pub id: i32,
    pub game_id: i32,
    pub datetime: std::time::SystemTime,
    pub turn_number: i32,
}

impl GameLogLite {
    pub fn datetime_string(&self) -> String {
        chrono::DateTime::<chrono::Utc>::from(self.datetime).to_rfc2822()
    }
    pub fn get_all<D>(game_id: i32, db: &D) -> Result<Vec<GameLogLite>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::game_logs::dsl as gl_dsl;
        gl_dsl::game_logs
            .select((
                gl_dsl::id,
                gl_dsl::game_id,
                gl_dsl::datetime,
                gl_dsl::turn_number,
            ))
            .filter(gl_dsl::game_id.eq(game_id))
            .order(gl_dsl::datetime.desc())
            .get_results(db)
    }

    pub fn get_output<D>(&self, db: &D) -> Result<String, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        NewAdminLog {
            game_id: self.game_id,
            datetime: std::time::SystemTime::now(),
            action: format!("Viewed turn logs for turn {}", self.turn_number),
        }
        .insert(db)?;
        use crate::schema::game_logs::dsl as gl_dsl;
        gl_dsl::game_logs
            .select(gl_dsl::output)
            .filter(gl_dsl::id.eq(self.id))
            .get_result(db)
    }
    pub fn get_errors<D>(&self, db: &D) -> Result<String, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::game_logs::dsl as gl_dsl;
        gl_dsl::game_logs
            .select(gl_dsl::error)
            .filter(gl_dsl::id.eq(self.id))
            .get_result(db)
    }
}

#[derive(Insertable)]
#[table_name = "game_logs"]
pub struct NewGameLog<'a> {
    pub game_id: i32,
    pub datetime: std::time::SystemTime,
    pub turn_number: i32,
    pub output: &'a str,
    pub error: &'a str,
}

impl<'a> NewGameLog<'a> {
    pub fn insert<D>(&self, db: &D) -> Result<GameLog, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::game_logs::dsl as gl_dsl;
        diesel::insert_into(gl_dsl::game_logs)
            .values(self)
            .get_result(db)
    }
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "admin_logs"]
pub struct AdminLog {
    id: i32,
    pub game_id: i32,
    pub datetime: std::time::SystemTime,
    pub action: String,
}

impl AdminLog {
    pub fn get_all<D>(game_id: i32, db: &D) -> Result<Vec<AdminLog>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::admin_logs::dsl as al_dsl;
        al_dsl::admin_logs
            .filter(al_dsl::game_id.eq(game_id))
            .order(al_dsl::datetime.desc())
            .get_results(db)
    }
    pub fn datetime_string(&self) -> String {
        chrono::DateTime::<chrono::Utc>::from(self.datetime).to_rfc2822()
    }
}

#[derive(Insertable)]
#[table_name = "admin_logs"]
struct NewAdminLog {
    game_id: i32,
    datetime: std::time::SystemTime,
    action: String,
}
impl NewAdminLog {
    pub fn insert<D>(&self, db: &D) -> Result<usize, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::admin_logs::dsl as al_dsl;
        diesel::insert_into(al_dsl::admin_logs)
            .values(self)
            .execute(db)
    }
}

#[derive(QueryableByName, Queryable, Debug, Identifiable)]
#[table_name = "discord_configs"]
pub struct DiscordConfig {
    pub id: i32,
    pub game_id: i32,
    pub last_turn_notified: Option<i32>,
    pub discord_guildid: String,
    pub discord_channelid: String,
    pub message: String,
    pub hours_remaining: Option<i32>,
}

impl DiscordConfig {
    pub fn get_due_reminders<D>(db: &D) -> Result<Vec<DiscordConfig>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(r#"
SELECT dc.*
FROM discord_configs dc
LEFT JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE archived = false GROUP BY game_id) t
    ON t.game_id=dc.game_id
INNER JOIN games g ON g.id=t.game_id AND g.archived = FALSE
WHERE
    (dc.last_turn_notified IS NULL OR t.turn_number != dc.last_turn_notified)
    AND g.next_turn IS NOT NULL
    AND g.next_turn - interval '1' hour * dc.hours_remaining < NOW()
    AND dc.hours_remaining IS NOT NULL
    AND t.turn_number IS NOT NULL
    "#).load(db)
    }
    pub fn get_due_notifications<D>(db: &D) -> Result<Vec<DiscordConfig>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(r#"
SELECT dc.*
FROM discord_configs dc
LEFT JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE archived = false GROUP BY game_id) t
    ON t.game_id=dc.game_id
INNER JOIN games g ON g.id=t.game_id AND g.archived = FALSE
WHERE
    (dc.last_turn_notified IS NULL OR t.turn_number != dc.last_turn_notified)
    AND g.next_turn IS NOT NULL
    AND dc.hours_remaining IS NULL
    AND t.turn_number IS NOT NULL
"#).get_results(db)
    }
    pub fn get_reminders_wakeup<D>(
        db: &D,
    ) -> Result<Option<std::time::SystemTime>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        Ok(diesel::sql_query(
            r#"
SELECT g.next_turn - interval '1' hour * dc.hours_before_host as timestamp
FROM discord_configs dc
LEFT JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns WHERE archived = false GROUP BY game_id) t
    ON t.game_id=dc.game_id
INNER JOIN games g ON g.id=t.game_id AND g.archived = FALSE
WHERE
    (dc.last_turn_notified IS NULL OR t.turn_number != dc.last_turn_notified)
    AND g.next_turn IS NOT NULL
    AND dc.hours_remaining IS NOT NULL
    AND t.turn_number IS NOT NULL
ORDER BY timestamp DESC
LIMIT 1
"#).get_result::<Wakeup>(db).optional()?
        .and_then(|w| Some(w.timestamp)))
    }
    pub fn get_notifications<D>(
        game_id: i32,
        db: &D,
    ) -> Result<Vec<DiscordConfig>, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::discord_configs::dsl as d_dsl;
        d_dsl::discord_configs
            .filter(d_dsl::game_id.eq(game_id))
            .get_results(db)
    }

    pub fn mark_sent<D>(&self, db: &D) -> Result<DiscordConfig, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        diesel::sql_query(
            r#"
UPDATE discord_configs dc
SET last_turn_notified=(SELECT MAX(turn_number) FROM turns t WHERE t.game_id=dc.game_id AND t.archived=FALSE)
WHERE id = $1
RETURNING *"#,
        )
        .bind::<diesel::sql_types::Integer, _>(self.id)
        .get_result(db)
    }
}

#[derive(Insertable)]
#[table_name = "discord_configs"]
pub struct NewDiscordReminder<'a> {
    pub game_id: i32,
    pub discord_guildid: &'a str,
    pub discord_channelid: &'a str,
    pub message: &'a str,
    pub hours_remaining: i32,
}

#[derive(Insertable)]
#[table_name = "discord_configs"]
pub struct NewDiscordNotification<'a> {
    pub game_id: i32,
    pub discord_guildid: &'a str,
    pub discord_channelid: &'a str,
    pub message: &'a str,
}

impl<'a> NewDiscordNotification<'a> {
    pub fn insert<D>(&self, db: &D) -> Result<DiscordConfig, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::discord_configs::dsl as d_dsl;
        diesel::insert_into(d_dsl::discord_configs)
            .values(self)
            .get_result(db)
    }
}

impl<'a> NewDiscordReminder<'a> {
    pub fn insert<D>(&self, db: &D) -> Result<DiscordConfig, diesel::result::Error>
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::discord_configs::dsl as d_dsl;
        diesel::insert_into(d_dsl::discord_configs)
            .values(self)
            .get_result(db)
    }
}
