use super::utils::*;
use super::AppData;
use crate::diesel::prelude::*;

use crate::discord::DiscordManager;
use crate::models::{
    AdminLog, Disciple, DiscordConfig, EmailConfig, Game, GameLogLite, Map, Mod, Nation, NewGame,
    NewGameMod, Player, PlayerTurn, Turn,
};
use crate::msgbus::{
    CreateGameMsg, EraChangedMsg, GameArchivedMsg, GameScheduleMsg, MapChangedMsg, ModsChangedMsg,
    Msg,
};
use crate::StartGame;
use actix_web::http::header;
use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;

// === Payloads ===
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct GamesListPayload {
    antibot_failed: Option<i32>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct GameSettings {
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    era: i32,
    #[serde(default = "default_five")]
    #[serde(deserialize_with = "from_str")]
    thrones_t1: i32,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    thrones_t2: i32,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    thrones_t3: i32,
    #[serde(default = "default_five")]
    #[serde(deserialize_with = "from_str")]
    throne_points_required: i32,
    #[serde(default = "default_two")]
    #[serde(deserialize_with = "from_str")]
    research_diff: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    research_rand: i32,
    #[serde(default = "default_ten")]
    #[serde(deserialize_with = "from_str")]
    hof_size: i32,
    #[serde(default = "default_five")]
    #[serde(deserialize_with = "from_str")]
    global_size: i32,
    #[serde(default = "default_five")]
    #[serde(deserialize_with = "from_str")]
    indepstr: i32,
    #[serde(default = "default_forty")]
    #[serde(deserialize_with = "from_str")]
    magicsites: i32,
    #[serde(default = "default_two")]
    #[serde(deserialize_with = "from_str")]
    eventrarity: i32,
    #[serde(default = "default_hundred")]
    #[serde(deserialize_with = "from_str")]
    richness: i32,
    #[serde(default = "default_hundred")]
    #[serde(deserialize_with = "from_str")]
    resources: i32,
    #[serde(default = "default_hundred")]
    #[serde(deserialize_with = "from_str")]
    recruitment: i32,
    #[serde(default = "default_hundred")]
    #[serde(deserialize_with = "from_str")]
    supplies: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    startprov: i32,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    renaming: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    scoregraphs: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    nationinfo: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    artrest: i32,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    teamgame: i32,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    clustered: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    storyevents: i32,
    #[serde(default = "default_two")]
    #[serde(deserialize_with = "from_str")]
    newailvl: i32,
    #[serde(deserialize_with = "from_str")]
    #[serde(default = "default_one")]
    map: i32,
    #[serde(default)]
    mapfilter: String,
    #[serde(default)]
    #[serde(deserialize_with = "from_str_seq")]
    cmods: Vec<i32>,
    #[serde(default)]
    modfilter: String,
    masterpass: Option<String>,
    #[serde(default)]
    private: i32,
    // Marker to tell whether the form is on its first load
    loaded: Option<String>,
}
impl From<(&Game, &[Mod])> for GameSettings {
    fn from((game, mods): (&Game, &[Mod])) -> GameSettings {
        GameSettings {
            era: game.era,
            thrones_t1: game.thrones_t1,
            thrones_t2: game.thrones_t2,
            thrones_t3: game.thrones_t3,
            throne_points_required: game.throne_points_required,
            research_diff: game.research_diff,
            research_rand: game.research_rand as i32,
            hof_size: game.hof_size,
            global_size: game.global_size,
            indepstr: game.indepstr,
            magicsites: game.magicsites,
            eventrarity: game.eventrarity,
            richness: game.richness,
            resources: game.resources,
            recruitment: game.recruitment,
            supplies: game.supplies,
            startprov: game.startprov,
            renaming: game.renaming as i32,
            scoregraphs: game.scoregraphs as i32,
            nationinfo: game.nationinfo as i32,
            artrest: game.artrest as i32,
            teamgame: game.teamgame as i32,
            clustered: game.clustered as i32,
            storyevents: game.storyevents,
            newailvl: game.newailvl,
            map: game.map_id,
            mapfilter: "".to_string(),
            modfilter: "".to_string(),
            cmods: mods.iter().map(|m| m.id).collect(),
            masterpass: game.masterpass.clone(),
            loaded: None,
            private: game.private as i32,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailForm {
    #[serde(default)]
    email_address: String,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    nation: i32,
    #[serde(default)]
    subject: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    #[serde(deserialize_with = "from_str")]
    hours_remaining: i32,
    #[serde(default)]
    is_reminder: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamForm {
    team: i32,
    nation: i32,
    disciple: i32,
}
impl Default for EmailForm {
    fn default() -> Self {
        Self {
            email_address: "".to_string(),
            nation: 0,
            subject: "".to_string(),
            body: "".to_string(),
            hours_remaining: 0,
            is_reminder: false,
        }
    }
}
#[derive(PartialEq, Debug, Serialize, Deserialize)]
enum AuthStatus {
    Unauthed = 0,
    AuthSuccess = 1,
    AuthFail = 2,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct GameDetailsPayload {
    #[serde(flatten)]
    game_settings: GameSettings,
    #[serde(flatten)]
    email_form: EmailForm,
    password: Option<String>,
    log_errors: Option<i32>,
    log_output: Option<i32>,
}
#[derive(Debug, Deserialize)]
struct CreateGame {
    name: String,
    password: String,
    antibot: Option<String>,
}
#[derive(Debug, Deserialize)]
struct SetTimer {
    timer: i32,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct EmailFormEntry {
    pub selected_nation: i32,
    pub remaining_hours: Option<i32>,
}

#[derive(Debug)]
struct PlayerSummary {
    name: String,
    epithet: String,
    pretender_name: String,
    id: i32,
    status: i32,
    is_disciple: bool,
    team: i32,
}
#[derive(Template)]
#[template(path = "games/details.html")]
struct GameDetailsTemplate<'a> {
    game: Game,
    settings: GameSettings,
    email_form: EmailForm,
    email_configs: &'a [EmailConfig],
    turn_number: i32,
    turns: HashMap<i32, Vec<PlayerTurn>>,
    hostname: String,
    players: &'a Vec<Player>,
    disciples: &'a Vec<Disciple>,
    nations: &'a Vec<Nation>,
    mods: &'a Vec<Mod>,
    maps: &'a Vec<Map>,
    tab: String,
    authed: AuthStatus,
    logs: Vec<GameLogLite>,
    focused_log: (i32, String, String),
    admin_logs: &'a Vec<AdminLog>,
    discord_clientid: Option<String>,
    discord_notifications: Vec<DiscordConfig>,
    discord_manager: Option<&'a DiscordManager>,
}
impl<'a> GameDetailsTemplate<'a> {
    pub fn get_channel_name(&self, server: &str, channel: &str) -> String {
        match self.discord_manager {
            None => "Discord not enabled".to_string(),
            Some(d) => d.get_channel_name(server, channel),
        }
    }
    pub fn get_server_name(&self, server: &str) -> String {
        match self.discord_manager {
            None => "Discord not enabled".to_string(),
            Some(d) => d.get_server_name(server),
        }
    }
    fn get_team_leaders(&self) -> Vec<(i32, PlayerSummary)> {
        let mut teams = self
            .disciples
            .iter()
            .filter_map(|d| {
                if d.team.unwrap_or(0) > 0 {
                    d.team
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>();
        teams.sort_unstable();
        teams.dedup();
        let mut disc: Vec<(i32, PlayerSummary)> = teams
            .into_iter()
            .filter_map(|t| {
                match self
                    .disciples
                    .iter()
                    .find(|d| d.team.unwrap_or(0) == t && d.is_disciple == 0)
                {
                    Some(d) => Some((d.team.unwrap_or(0), self.get_player_summary(t, d.nation_id))),
                    None => Some((t, self.get_player_summary(t, 0))),
                }
            })
            .collect();
        disc.sort_unstable_by(|a, b| match a.0 - b.0 {
            a if a > 0 => std::cmp::Ordering::Greater,
            a if a < 0 => std::cmp::Ordering::Less,
            a if a == 0 => std::cmp::Ordering::Equal,
            _ => std::cmp::Ordering::Greater,
        });
        disc
    }
    fn get_team_disciples(&self, team: &i32) -> Vec<PlayerSummary> {
        self.disciples
            .iter()
            .filter_map(|d| {
                if self.turn_number != 0
                    && !self
                        .players
                        .iter()
                        .find(|p| p.nationid == d.nation_id)
                        .is_some()
                    || d.is_disciple == 0
                    || d.team.unwrap_or(0) != *team
                {
                    return None;
                }
                Some(self.get_player_summary(d.team.unwrap_or(0), d.nation_id))
            })
            .collect()
    }
    fn get_player_summary(&self, team: i32, nation_id: i32) -> PlayerSummary {
        match self.nations.iter().find(|n| n.nation_id == nation_id) {
            Some(n) => {
                let is_disciple = match self.disciples.iter().find(|d| d.nation_id == nation_id) {
                    Some(d) => d.is_disciple > 0,
                    None => true,
                };
                match self.players.iter().find(|p| p.nationid == nation_id) {
                    Some(p) => PlayerSummary {
                        name: n.name.clone(),
                        epithet: n.epithet.clone(),
                        pretender_name: p.name.clone(),
                        id: n.nation_id,
                        team,
                        status: self
                            .turns
                            .get(&n.nation_id)
                            .map_or(0, |ts| ts.last().map_or(0, |t| t.status)),
                        is_disciple,
                    },
                    None => PlayerSummary {
                        name: n.name.clone(),
                        epithet: n.epithet.clone(),
                        pretender_name: "Nation has not joined".to_string(),
                        id: n.nation_id,
                        status: -1,
                        team,
                        is_disciple,
                    },
                }
            }
            None => PlayerSummary {
                name: "Not Selected".to_string(),
                epithet: "".to_string(),
                pretender_name: "Nation has not joined".to_string(),
                id: 0,
                team,
                status: -1,
                is_disciple: false,
            },
        }
    }
    fn get_joined_nations(&self) -> Vec<&Nation> {
        self.nations
            .iter()
            .filter(|n| {
                self.players
                    .iter()
                    .find(|p| p.nationid == n.nation_id)
                    .is_some()
            })
            .collect()
    }
    fn get_player_summaries(&self) -> Vec<PlayerSummary> {
        self.players
            .iter()
            .map(|p| self.get_player_summary(0, p.nationid))
            .collect()
    }
    fn get_email_config_nation_name(&self, id: &i32) -> &str {
        self.email_configs
            .iter()
            .find(|e| e.id == *id)
            .map_or("", |e| {
                &self
                    .nations
                    .iter()
                    .find(|n| n.nation_id == e.nation_id)
                    .unwrap()
                    .name
            })
    }
    fn get_turn_status(&self, nation_id: &i32, turn_number: &i32) -> i32 {
        self.turns
            .get(nation_id)
            .map_or(0, |t| t.get(*turn_number as usize).map_or(0, |t| t.status))
    }
    fn get_turn_pips(&self, nation_id: &i32) -> Vec<&PlayerTurn> {
        match self.turns.get(&nation_id) {
            Some(turns) => {
                let len = turns.len();
                if len > 0 {
                    turns
                        .iter()
                        .rev()
                        .take(10)
                        .rev()
                        .take(if len < 9 { len - 1 } else { 9 })
                        .collect()
                } else {
                    vec![]
                }
            }
            None => vec![],
        }
    }
    fn get_current_mods(&self) -> Vec<&Mod> {
        if self.settings.cmods.len() == 0 {
            vec![]
        } else {
            self.mods
                .iter()
                .filter(|m| self.settings.cmods.iter().find(|cm| **cm == m.id).is_some())
                .collect()
        }
    }
    fn get_status_string(&self) -> String {
        if self.game.archived {
            "Archived".to_string()
        } else {
            if self.turn_number == 0 {
                "Waiting for pretenders".to_string()
            } else {
                format!("Turn {}", self.turn_number)
            }
        }
    }
    fn get_shorthand_description(&self) -> String {
        format!(
            "{}\n{}{}",
            &self.get_status_string(),
            match self.game.port {
                Some(p) => format!("{}:{}\n", self.hostname, p),
                None => "".to_owned(),
            },
            match self.game.next_turn {
                Some(_t) => format!("Next turn in {}\n", self.game.next_turn_string()),
                None => "".to_owned(),
            }
        )
    }

    fn get_current_map(&self) -> Option<&Map> {
        if self.settings.map != 0 {
            self.maps.iter().find(|m| m.id == self.settings.map)
        } else {
            None
        }
    }
}
struct ActiveGame {
    id: i32,
    name: String,
    address: String,
    turn_summary: crate::models::TurnSummary,
    next_turn_string: String,
}
struct PendingGame {
    id: i32,
    name: String,
    address: String,
    players: i32,
}
struct ArchivedGame {
    id: i32,
}
#[derive(Template)]
#[template(path = "games/list.html")]
struct GamesTemplate<'a> {
    pending_games: &'a [PendingGame],
    active_games: &'a [ActiveGame],
    archived_games: &'a [ArchivedGame],
    antibot_question: Option<String>,
    antibot_failed: Option<i32>,
}
impl<'a> GamesTemplate<'a> {
    pub fn question_string(&self) -> String {
        match &self.antibot_question {
            Some(s) => s.to_owned(),
            None => "".to_owned(),
        }
    }
}

// === Routes ====
#[post("/game/{id}/postpone")]
async fn postpone(
    (app_data, web::Path(path_id), timer_form): (
        web::Data<AppData>,
        web::Path<i32>,
        web::Form<SetTimer>,
    ),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let game: Game = {
        use crate::schema::games::dsl::*;
        games
            .filter(id.eq(path_id))
            .get_result::<Game>(&db)
            .unwrap()
    };
    let sched = std::time::SystemTime::now()
        .add(std::time::Duration::from_secs(60 * timer_form.timer as u64));
    use crate::schema::games::dsl::*;
    diesel::update(games.filter(id.eq(game.id)))
        .set(next_turn.eq(Some(sched)))
        .execute(&db)
        .unwrap();
    app_data
        .msgbus_sender
        .send(Msg::GameSchedule(GameScheduleMsg {
            game_id: game.id,
            schedule: sched,
        }))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", game.id))
        .finish())
}
#[post("/game/{id}/timer")]
async fn timer(
    (app_data, web::Path(path_id), timer_form): (
        web::Data<AppData>,
        web::Path<i32>,
        web::Form<SetTimer>,
    ),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::games::dsl as games_dsl;
    use crate::schema::turns::dsl as turns_dsl;
    let sched = std::time::SystemTime::now()
        .add(std::time::Duration::from_secs(60 * timer_form.timer as u64));
    let game: Game = games_dsl::games
        .filter(games_dsl::id.eq(path_id))
        .get_result::<Game>(&db)
        .unwrap();
    let turns: Vec<Turn> = turns_dsl::turns
        .filter(
            turns_dsl::game_id
                .eq(game.id)
                .and(turns_dsl::archived.eq(false)),
        )
        .get_results(&db)
        .unwrap();
    if turns.len() > 0 {
        diesel::update(games_dsl::games.filter(games_dsl::id.eq(game.id)))
            .set((
                games_dsl::timer.eq(Some(timer_form.timer)),
                games_dsl::next_turn.eq(sched),
            ))
            .execute(&db)
            .unwrap();
    } else {
        diesel::update(games_dsl::games.filter(games_dsl::id.eq(game.id)))
            .set(games_dsl::timer.eq(Some(timer_form.timer)))
            .execute(&db)
            .unwrap();
    }
    app_data
        .msgbus_sender
        .send(Msg::GameSchedule(GameScheduleMsg {
            game_id: game.id,
            schedule: sched,
        }))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", game.id))
        .finish())
}
#[get("/game/{id}/{tab}")]
async fn details(
    (app_data, web::Path((path_id, tab)), session, payload): (
        web::Data<AppData>,
        web::Path<(String, String)>,
        actix_session::Session,
        serde_qs::actix::QsQuery<GameDetailsPayload>,
    ),
) -> Result<HttpResponse> {
    let game_settings = &payload.game_settings;
    let email_form = &payload.email_form;
    let db = app_data.pool.get().expect("Unable to connect to database");
    let game = if let Ok(id_i32) = path_id.parse::<i32>() {
        use crate::schema::games::dsl::*;
        games
            .filter(id.eq(id_i32))
            .get_result::<Game>(&*db)
            .unwrap()
    } else {
        use crate::schema::games::dsl::*;
        let game: Game = games.filter(name.ilike(path_id)).get_result(&*db).unwrap();
        return Ok(HttpResponse::PermanentRedirect()
            .header(header::LOCATION, format!("/game/{}/{}", game.id, tab))
            .finish());
    };
    if let Some(p) = &payload.password {
        if *p == game.password {
            session
                .set(&format!("auth_{}", game.id), AuthStatus::AuthSuccess)
                .unwrap();
        } else {
            session
                .set(&format!("auth_{}", game.id), AuthStatus::AuthFail)
                .unwrap();
        }
        return Ok(HttpResponse::TemporaryRedirect()
            .header(header::LOCATION, format!("/game/{}/{}", game.id, tab))
            .finish());
    }
    let authed: AuthStatus = session
        .get(&format!("auth_{}", game.id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed);
    let players = Player::get_players(game.id, &db).unwrap();
    let nations = Nation::get_all(game.id, &db).unwrap();
    let disciples = Disciple::get_all(game.id, &db).unwrap();
    let player_turn_map = crate::models::PlayerTurn::get_player_turns(game.id, &db);
    let turns = Turn::get_all(game.id, &db).unwrap();
    let settings: GameSettings = if let Some(_) = game_settings.loaded {
        game_settings.clone()
    } else {
        (&game, &game.get_mods(&db).unwrap()[..]).into()
    };
    let (maps, mods): (Vec<Map>, Vec<Mod>) = (
        {
            use crate::schema::maps::dsl::*;
            if authed == AuthStatus::AuthSuccess {
                maps.filter(name.ilike(&format!("%{}%", game_settings.mapfilter)))
                    .get_results(&db)
                    .unwrap()
            } else {
                maps.filter(id.eq(game.map_id)).get_results(&db).unwrap()
            }
        },
        {
            if authed == AuthStatus::AuthSuccess {
                use crate::schema::mods::dsl::*;
                mods.filter(name.ilike(&format!("%{}%", game_settings.modfilter)))
                    .get_results(&db)
                    .unwrap()
            } else {
                game.get_mods(&db).unwrap()
            }
        },
    );
    use crate::schema::email_configs::dsl as email_dsl;
    let email_configs: Vec<EmailConfig> = email_dsl::email_configs
        .filter(
            email_dsl::email_address
                .eq(&email_form.email_address)
                .and(email_dsl::game_id.eq(game.id)),
        )
        .get_results(&db)
        .unwrap();
    let logs = GameLogLite::get_all(game.id, &db).unwrap();
    let logs_detail = if let Some(output_id) = payload.log_output {
        logs.iter()
            .find(|l| l.id == output_id)
            .unwrap()
            .get_output(&db)
            .unwrap()
    } else if let Some(errors_id) = payload.log_errors {
        logs.iter()
            .find(|l| l.id == errors_id)
            .unwrap()
            .get_errors(&db)
            .unwrap()
    } else {
        (0, String::new(), String::new())
    };
    let admin_logs = AdminLog::get_all(game.id, &db).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (GameDetailsTemplate {
            logs: GameLogLite::get_all(game.id, &db).unwrap(),
            focused_log: logs_detail,
            settings,
            authed,
            email_form: (*email_form).clone(),
            email_configs: &email_configs,
            tab,
            turn_number: turns.len() as i32,
            maps: &maps,
            mods: &mods,
            players: &players,
            disciples: &disciples,
            nations: &nations,
            turns: player_turn_map,
            hostname: std::env::var("HOSTNAME").unwrap(),
            admin_logs: &admin_logs,
            discord_clientid: app_data
                .discord_manager
                .as_ref()
                .and_then(|d| Some(d.client_id.clone())),
            discord_notifications: DiscordConfig::get_notifications(game.id, &db).unwrap(),
            discord_manager: app_data.discord_manager.as_ref(),
            game,
        })
        .render()
        .unwrap(),
    ))
}
#[post("/game/{id}/launch")]
async fn launch(
    (app_data, web::Path(path_id), form): (
        web::Data<AppData>,
        web::Path<i32>,
        web::Form<StartGame>,
    ),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::games::dsl::*;
    let sched = std::time::SystemTime::now().add(std::time::Duration::from_secs(form.countdown));
    let game = games
        .filter(id.eq(path_id))
        .get_result::<Game>(&*db)
        .unwrap();

    diesel::update(games)
        .filter(id.eq(path_id))
        .set(next_turn.eq(sched))
        .execute(&db)
        .unwrap();

    app_data
        .msgbus_sender
        .send(Msg::GameSchedule(GameScheduleMsg {
            game_id: game.id,
            schedule: sched,
        }))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", game.id))
        .finish())
}
#[post("/games/create")]
async fn create_post(
    (app_data, mut body): (web::Data<AppData>, web::Payload),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::maps::dsl as maps_dsl;
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    let config = serde_qs::Config::new(10, false);
    let params: CreateGame = config.deserialize_bytes(&*bytes).unwrap();
    if AppData::get_antibot_question().is_some() {
        if params.antibot.is_none()
            || AppData::get_antibot_answer()
                .as_ref()
                .unwrap()
                .to_ascii_lowercase()
                != params.antibot.unwrap().to_ascii_lowercase()
        {
            return Ok(HttpResponse::Found()
                .header(header::LOCATION, "/games?antibot_failed=1")
                .finish());
        }
    }

    let mut new_game = NewGame::default();
    new_game.name = params.name;
    new_game.password = params.password;
    new_game.map_id = maps_dsl::maps
        .order(maps_dsl::id.desc())
        .limit(1)
        .get_result::<Map>(&db)
        .unwrap()
        .id;

    let game: Game = diesel::insert_into(crate::schema::games::table)
        .values(&new_game)
        .get_result(&db)
        .expect("Error saving new game");

    app_data
        .msgbus_sender
        .send(Msg::CreateGame(CreateGameMsg { game_id: game.id }))
        .unwrap();

    Ok(HttpResponse::Found()
        .header(
            header::LOCATION,
            format!("/game/{}/settings?password={}", game.id, new_game.password),
        )
        .finish())
}
#[get("/games")]
async fn list(
    app_data: web::Data<AppData>,
    query: serde_qs::actix::QsQuery<GamesListPayload>,
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");

    // Create game
    let mut results = Game::get_public(&db).unwrap();
    let player_counts = crate::models::Game::get_player_count(
        results.iter().map(|g| g.id).collect::<Vec<i32>>(),
        &db,
    );
    let turn_summaries =
        crate::models::Turn::turn_summary(&results.iter().map(|g| g.id).collect::<Vec<i32>>(), &db);
    let archived_games: Vec<ArchivedGame> = results
        .drain_filter(|g| g.archived)
        .map(|g| ArchivedGame { id: g.id })
        .collect();
    let active_games: Vec<ActiveGame> = results
        .drain_filter(|g| turn_summaries.iter().find(|t| t.game_id == g.id).is_some())
        .map(|g| ActiveGame {
            next_turn_string: g.next_turn_string(),
            id: g.id,
            turn_summary: turn_summaries
                .iter()
                .find(|t| t.game_id == g.id)
                .unwrap()
                .clone(),
            name: g.name.clone(),
            address: format!(
                "{}:{}",
                std::env::var("HOSTNAME").unwrap(),
                g.port.unwrap_or(0)
            ),
        })
        .collect();
    let pending_games: Vec<PendingGame> = results
        .iter()
        .map(|g| PendingGame {
            id: g.id,
            players: match player_counts.contains_key(&g.id) {
                true => player_counts[&g.id],
                false => 0,
            },
            address: format!(
                "{}:{}",
                std::env::var("HOSTNAME").unwrap(),
                g.port.unwrap_or(0)
            ),
            name: g.name.clone(),
        })
        .collect();
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (GamesTemplate {
            active_games: &active_games,
            pending_games: &pending_games,
            archived_games: &archived_games,
            antibot_failed: query.antibot_failed,
            antibot_question: AppData::get_antibot_question(),
        })
        .render()
        .unwrap(),
    ))
}
#[post("/game/{id}/settings")]
async fn settings_post(
    (app_data, path_id, mut body, session): (
        web::Data<AppData>,
        web::Path<i32>,
        web::Payload,
        actix_session::Session,
    ),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    let config = serde_qs::Config::new(10, false);
    let body: GameSettings = config.deserialize_bytes(&*bytes).unwrap();
    if session
        .get(&format!("auth_{}", path_id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed)
        == AuthStatus::Unauthed
    {
        return Ok(HttpResponse::Unauthorized()
            .header(header::LOCATION, format!("/game/{}/settings", path_id))
            .finish());
    }
    let old_game = Game::get(*path_id, &db).unwrap();
    let old_mods = old_game.get_mods(&db).unwrap();
    let game: Game = db
        .transaction::<_, diesel::result::Error, _>(|| {
            let game: Game = {
                use crate::schema::games::dsl::*;
                diesel::update(games.filter(id.eq(*path_id)))
                    .set((
                        era.eq(body.era),
                        thrones_t1.eq(body.thrones_t1),
                        thrones_t2.eq(body.thrones_t2),
                        thrones_t3.eq(body.thrones_t3),
                        throne_points_required.eq(body.throne_points_required),
                        research_diff.eq(body.research_diff),
                        research_rand.eq(body.research_rand > 0),
                        hof_size.eq(body.hof_size),
                        global_size.eq(body.global_size),
                        indepstr.eq(body.indepstr),
                        magicsites.eq(body.magicsites),
                        eventrarity.eq(body.eventrarity),
                        richness.eq(body.richness),
                        resources.eq(body.resources),
                        recruitment.eq(body.recruitment),
                        supplies.eq(body.supplies),
                        startprov.eq(body.startprov),
                        renaming.eq(body.renaming > 0),
                        scoregraphs.eq(body.scoregraphs > 0),
                        nationinfo.eq(body.nationinfo > 0),
                        artrest.eq(body.artrest > 0),
                        teamgame.eq(body.teamgame > 0),
                        clustered.eq(body.clustered > 0),
                        storyevents.eq(body.storyevents),
                        newailvl.eq(body.newailvl),
                        map_id.eq(body.map),
                        masterpass.eq(body.masterpass),
                        private.eq(body.private > 0),
                    ))
                    .get_result(&db)?
            };
            {
                use crate::schema::game_mods::dsl::*;
                diesel::delete(game_mods)
                    .filter(game_id.eq(game.id))
                    .execute(&db)?;
                diesel::insert_into(game_mods)
                    .values(
                        body.cmods
                            .iter()
                            .map(|cm| NewGameMod {
                                game_id: game.id,
                                mod_id: *cm,
                            })
                            .collect::<Vec<NewGameMod>>(),
                    )
                    .execute(&db)?;
            };
            Ok(game)
        })
        .unwrap();
    if game.era != old_game.era {
        app_data
            .msgbus_sender
            .send(Msg::EraChanged(EraChangedMsg {
                game_id: game.id,
                new_era: game.era,
            }))
            .unwrap();
    }
    let mods = old_game.get_mods(&db).unwrap();
    if mods.len() != old_mods.len()
        || mods
            .iter()
            .any(|m| !old_mods.iter().any(|om| om.id == m.id))
    {
        app_data
            .msgbus_sender
            .send(Msg::ModsChanged(ModsChangedMsg { game_id: game.id }))
            .unwrap();
    }
    app_data
        .msgbus_sender
        .send(Msg::MapChanged(MapChangedMsg {
            game_id: game.id,
            map_id: game.map_id,
        }))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/settings", game.id))
        .finish())
}
#[post("/game/{id}/email")]
async fn emails_post(
    (app_data, bytes, web::Path(game_id)): (
        web::Data<AppData>,
        actix_web::web::Bytes,
        web::Path<i32>,
    ),
) -> Result<HttpResponse> {
    let cfg = serde_qs::Config::new(10, false);
    match cfg.deserialize_bytes::<EmailForm>(&bytes) {
        Ok(email_form) => {
            app_data.email_manager.create_config(
                game_id,
                email_form.email_address.clone(),
                email_form.subject,
                email_form.body,
                email_form.nation,
                email_form.hours_remaining,
                email_form.is_reminder,
            );
            Ok(HttpResponse::Found()
                .header(
                    header::LOCATION,
                    format!(
                        "/game/{}/emails?email_address={}",
                        game_id, email_form.email_address
                    ),
                )
                .finish())
        }
        Err(_) => Ok(HttpResponse::BadRequest()
            .header(header::LOCATION, format!("/game/{}/email", game_id))
            .finish()),
    }
}
#[post("/game/{id}/emails/{emailid}/delete")]
async fn emails_delete(
    (app_data, bytes, web::Path((game_id, email_id))): (
        web::Data<AppData>,
        actix_web::web::Bytes,
        web::Path<(i32, i32)>,
    ),
) -> Result<HttpResponse> {
    let cfg = serde_qs::Config::new(10, false);
    match cfg.deserialize_bytes::<EmailForm>(&bytes) {
        Ok(email_form) => {
            app_data
                .email_manager
                .delete_config(email_id, email_form.email_address.clone());
            Ok(HttpResponse::Found()
                .header(
                    header::LOCATION,
                    format!(
                        "/game/{}/emails?email_address={}",
                        game_id, email_form.email_address
                    ),
                )
                .finish())
        }
        Err(_) => Ok(HttpResponse::BadRequest()
            .header(header::LOCATION, format!("/game/{}/email", game_id))
            .finish()),
    }
}

#[post("/game/{id}/archive")]
pub async fn archive_post(
    (app_data, path_id, session): (web::Data<AppData>, web::Path<i32>, actix_session::Session),
) -> Result<HttpResponse> {
    if session
        .get(&format!("auth_{}", path_id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed)
        == AuthStatus::Unauthed
    {
        return Ok(HttpResponse::Unauthorized()
            .header(header::LOCATION, format!("/game/{}/schedule", path_id))
            .finish());
    }
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::games::dsl as games_dsl;
    diesel::update(games_dsl::games.filter(games_dsl::id.eq(*path_id)))
        .set((
            games_dsl::archived.eq(true),
            games_dsl::port.eq::<Option<i32>>(None),
        ))
        .execute(&db)
        .unwrap();
    app_data
        .msgbus_sender
        .send(Msg::GameArchived(GameArchivedMsg { game_id: *path_id }))
        .unwrap();
    return Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", path_id))
        .finish());
}

#[post("/game/{id}/remove/{playerid}")]
pub async fn remove_post(
    (app_data, web::Path((path_id, nation_id)), session): (
        web::Data<AppData>,
        web::Path<(i32, i32)>,
        actix_session::Session,
    ),
) -> Result<HttpResponse> {
    if session
        .get(&format!("auth_{}", path_id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed)
        == AuthStatus::Unauthed
    {
        return Ok(HttpResponse::Unauthorized()
            .header(header::LOCATION, format!("/game/{}/schedule", path_id))
            .finish());
    }
    let db = app_data.pool.get().expect("Unable to connect to database");
    let turns = crate::models::Turn::current_turn(&vec![path_id], &db);
    // Check if there were no turns found for the vector of games containing only our game, *not*
    // that there were zero games for just ours, this is not a logic error, though it looks odd.
    if turns.len() == 0 {
        let players = Player::get_players(path_id, &db).unwrap();
        if let Some(player) = players.iter().find(|p| p.nationid == nation_id) {
            player.remove(&db).unwrap();
        }
    }
    return Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/status", path_id))
        .finish());
}

#[post("/game/{id}/rollback")]
pub async fn rollback_post(
    (app_data, web::Path(path_id), session): (
        web::Data<AppData>,
        web::Path<i32>,
        actix_session::Session,
    ),
) -> Result<HttpResponse> {
    if session
        .get(&format!("auth_{}", path_id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed)
        == AuthStatus::Unauthed
    {
        return Ok(HttpResponse::Unauthorized()
            .header(header::LOCATION, format!("/game/{}/schedule", path_id))
            .finish());
    }
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::games::dsl as games_dsl;
    let game: Game = games_dsl::games
        .filter(games_dsl::id.eq(path_id))
        .get_result(&db)
        .unwrap();
    game.rollback(&db).unwrap();
    return Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", path_id))
        .finish());
}

#[post("/game/{id}/unstart")]
pub async fn unstart_post(
    (app_data, web::Path(path_id), session): (
        web::Data<AppData>,
        web::Path<i32>,
        actix_session::Session,
    ),
) -> Result<HttpResponse> {
    if session
        .get(&format!("auth_{}", path_id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed)
        == AuthStatus::Unauthed
    {
        return Ok(HttpResponse::Unauthorized()
            .header(header::LOCATION, format!("/game/{}/schedule", path_id))
            .finish());
    }
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::games::dsl as games_dsl;
    let game: Game = games_dsl::games
        .filter(games_dsl::id.eq(path_id))
        .get_result(&db)
        .unwrap();
    game.unstart(&db).unwrap();
    let _game = game.remove_timer(&db).unwrap();
    return Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", path_id))
        .finish());
}
#[post("/game/{id}/assign-team")]
pub async fn assign_team(
    (app_data, web::Path(path_id), session, form): (
        web::Data<AppData>,
        web::Path<i32>,
        actix_session::Session,
        web::Form<TeamForm>,
    ),
) -> Result<HttpResponse> {
    if session
        .get(&format!("auth_{}", path_id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed)
        == AuthStatus::Unauthed
    {
        return Ok(HttpResponse::Unauthorized()
            .header(header::LOCATION, format!("/game/{}/status", path_id))
            .finish());
    }
    let db = app_data.pool.get().expect("Unable to connect to database");
    let disciple = Disciple::get(path_id, form.nation, &db).unwrap();
    if form.team == 0 {
        if form.disciple == 0 {
            disciple.create_team(&db).unwrap();
        } else {
            disciple.remove(&db).unwrap();
        }
    } else {
        disciple
            .set_team(form.team, &db)
            .unwrap()
            .set_disc(form.disciple, &db)
            .unwrap();
    }
    return Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/status", path_id))
        .finish());
}
