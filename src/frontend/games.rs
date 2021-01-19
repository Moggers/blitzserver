use super::utils::*;
use super::AppData;
use crate::diesel::prelude::*;
use crate::game_manager;
use crate::models::{
    EmailConfig, Game, GameMod, Map, Mod, Nation, NewGame, NewGameMod, Player, PlayerTurn, Turn,
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
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    newai: i32,
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
            newai: game.newai as i32,
            map: game.map_id,
            mapfilter: "".to_string(),
            modfilter: "".to_string(),
            cmods: mods.iter().map(|m| m.id).collect(),
            loaded: None,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct EmailForm {
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
}
impl Default for EmailForm {
    fn default() -> Self {
        Self {
            email_address: "".to_string(),
            nation: 0,
            subject: "".to_string(),
            body: "".to_string(),
            hours_remaining: 0,
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
}
#[derive(Debug, Deserialize)]
struct CreateGame {
    name: String,
    password: String,
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

// === Templates ===
struct PlayerSummary {
    name: String,
    id: i32,
    submitted: bool,
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
    players: &'a Vec<PlayerSummary>,
    mods: &'a Vec<Mod>,
    maps: &'a Vec<Map>,
    tab: String,
    authed: AuthStatus,
}
impl<'a> GameDetailsTemplate<'a> {
    fn get_email_config_nation_name(&self, id: &i32) -> &str {
        self.email_configs
            .iter()
            .find(|e| e.id == *id)
            .map_or("", |e| {
                &self
                    .players
                    .iter()
                    .find(|p| p.id == e.nation_id)
                    .unwrap()
                    .name
            })
    }
    fn get_turn_submitted(&self, nation_id: &i32, turn_number: &i32) -> bool {
        self.turns.get(nation_id).map_or(false, |t| {
            t.get(*turn_number as usize)
                .map_or(false, |t| t.twohfile_id.is_some())
        })
    }
    fn get_turn_pips(turns: &[PlayerTurn]) -> Vec<&PlayerTurn> {
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
    {
        use crate::schema::games::dsl::*;
        diesel::update(games.filter(id.eq(game.id)))
            .set((next_turn
                .eq(Some(std::time::SystemTime::now().add(
                    std::time::Duration::from_secs(60 * timer_form.timer as u64),
                ))),))
            .execute(&db)
            .unwrap();
    }
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::GameMsg(
            crate::dom5_proxy::GameMsg {
                id: game.id,
                cmd: crate::dom5_proxy::GameMsgType::GameCmd(
                    crate::dom5_proc::GameCmd::SetTimerCmd,
                ),
            },
        ))
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
    let game: Game = {
        use crate::schema::games::dsl::*;
        games
            .filter(id.eq(path_id))
            .get_result::<Game>(&db)
            .unwrap()
    };
    {
        use crate::schema::games::dsl::*;
        diesel::update(games.filter(id.eq(game.id)))
            .set((
                timer.eq(Some(timer_form.timer)),
                next_turn
                    .eq(Some(std::time::SystemTime::now().add(
                        std::time::Duration::from_secs(60 * timer_form.timer as u64),
                    ))),
            ))
            .execute(&db)
            .unwrap();
    }
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::GameMsg(
            crate::dom5_proxy::GameMsg {
                id: game.id,
                cmd: crate::dom5_proxy::GameMsgType::GameCmd(
                    crate::dom5_proc::GameCmd::SetTimerCmd,
                ),
            },
        ))
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
            return Ok(HttpResponse::PermanentRedirect()
                .header(header::LOCATION, format!("/game/{}/{}", game.id, tab))
                .finish());
        } else {
            session
                .set(&format!("auth_{}", game.id), AuthStatus::AuthFail)
                .unwrap();
            return Ok(HttpResponse::PermanentRedirect()
                .header(header::LOCATION, format!("/game/{}/{}", game.id, tab))
                .finish());
        }
    }
    let authed: AuthStatus = session
        .get(&format!("auth_{}", game.id))
        .unwrap_or(Some(AuthStatus::Unauthed))
        .unwrap_or(AuthStatus::Unauthed);
    let game_players: Vec<(Player, Nation)> = {
        use crate::schema::players::dsl::*;
        Player::belonging_to(&game)
            .inner_join(
                crate::schema::nations::dsl::nations.on(crate::schema::nations::dsl::game_id
                    .eq(game.id)
                    .and(crate::schema::nations::dsl::nation_id.eq(nationid))),
            )
            .get_results(&db)
            .unwrap()
    };
    let player_turn_map = game_players.iter().fold(
        HashMap::new(),
        |mut f: HashMap<i32, Vec<PlayerTurn>>, (_, n)| {
            let pt = crate::schema::player_turns::dsl::player_turns
                .filter(
                    crate::schema::player_turns::dsl::game_id
                        .eq(n.game_id)
                        .and(crate::schema::player_turns::dsl::nation_id.eq(n.nation_id)),
                )
                .order(crate::schema::player_turns::dsl::turn_number)
                .get_results(&db)
                .unwrap();
            f.insert(n.nation_id, pt);
            f
        },
    );
    let turns: Vec<Turn> = { Turn::belonging_to(&game).get_results(&db).unwrap() };
    let settings: GameSettings = if let Some(_) = game_settings.loaded {
        game_settings.clone()
    } else {
        let mods = {
            &crate::schema::game_mods::dsl::game_mods
                .filter(crate::schema::game_mods::dsl::game_id.eq(game.id))
                .inner_join(
                    crate::schema::mods::dsl::mods
                        .on(crate::schema::mods::dsl::id.eq(crate::schema::game_mods::dsl::mod_id)),
                )
                .get_results::<(GameMod, Mod)>(&db)
                .unwrap()
                .iter()
                .map(move |(_, m)| (*m).clone())
                .collect::<Vec<Mod>>()
        };
        (&game, &mods[..]).into()
    };
    let (maps, mods): (Vec<Map>, Vec<Mod>) = (
        {
            use crate::schema::maps::dsl::*;
            maps.filter(name.ilike(&format!("%{}%", game_settings.mapfilter)))
                .get_results(&db)
                .unwrap()
        },
        {
            use crate::schema::mods::dsl::*;
            mods.filter(name.ilike(&format!("%{}%", game_settings.modfilter)))
                .get_results(&db)
                .unwrap()
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
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (GameDetailsTemplate {
            settings: settings,
            authed: authed,
            email_form: (*email_form).clone(),
            email_configs: &email_configs,
            tab: tab,
            turn_number: turns.len() as i32,
            maps: &maps,
            mods: &mods,
            game,
            players: &game_players
                .iter()
                .map(|(_, nation)| PlayerSummary {
                    id: nation.nation_id,
                    name: nation.name.clone(),
                    submitted: match player_turn_map.get(&nation.nation_id) {
                        Some(t) => t.last().map_or(false, |t| t.twohfile_id.is_some()),
                        None => false,
                    },
                })
                .collect(),
            turns: player_turn_map,
            hostname: std::env::var("HOSTNAME").unwrap(),
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
    let game = games
        .filter(id.eq(path_id))
        .get_result::<Game>(&*db)
        .unwrap();

    diesel::update(games)
        .filter(id.eq(path_id))
        .set(
            next_turn
                .eq(std::time::SystemTime::now()
                    .add(std::time::Duration::from_secs(form.countdown))),
        )
        .execute(&db)
        .unwrap();
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::GameMsg(
            crate::dom5_proxy::GameMsg {
                id: game.id,
                cmd: crate::dom5_proxy::GameMsgType::GameCmd(
                    crate::dom5_proc::GameCmd::SetTimerCmd,
                ),
            },
        ))
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
        .manager_notifier
        .send(game_manager::ManagerMsg::Start(game.id))
        .unwrap();

    Ok(HttpResponse::Found()
        .header(
            header::LOCATION,
            format!("/game/{}/settings?password={}", game.id, new_game.password),
        )
        .finish())
}
#[get("/games")]
async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");

    // Create game
    use crate::schema::games::dsl::games;
    let mut results = games.load::<Game>(&db).expect("Error loading games");
    let player_counts = crate::models::Game::get_player_count(
        results.iter().map(|g| g.id).collect::<Vec<i32>>(),
        &db,
    );
    let turn_summaries =
        crate::models::Turn::turn_summary(&results.iter().map(|g| g.id).collect::<Vec<i32>>(), &db);
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
    let archived_games: Vec<ArchivedGame> = results
        .drain_filter(|g| g.archived)
        .map(|g| ArchivedGame { id: g.id })
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
                        newai.eq(body.newai > 0),
                        map_id.eq(body.map),
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
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::GameMsg(
            crate::dom5_proxy::GameMsg {
                id: game.id,
                cmd: crate::dom5_proxy::GameMsgType::RebootCmd,
            },
        ))
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
        .set((games_dsl::archived.eq(true), games_dsl::port.eq::<Option<i32>>(None)))
        .execute(&db)
        .unwrap();
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::Archive(*path_id))
        .unwrap();
    return Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}/schedule", path_id))
        .finish());
}
