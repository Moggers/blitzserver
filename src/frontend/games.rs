use super::AppData;
use super::utils::{from_str, from_str_seq};
use crate::diesel::prelude::*;
use crate::game_manager;
use crate::models::{
    EmailConfig, Era, Game, GameMod, Map, Mod, Nation, NewEmailConfig, NewGame, NewGameMod, Player,
    PlayerTurn, Turn,
};
use crate::StartGame;
use actix_web::http::header;
use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use futures::StreamExt;
use serde::{de::Error, Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;

fn default_one() -> i32 {
    1
}

fn default_five() -> i32 {
    5
}

fn default_two() -> i32 {
    2
}

fn default_ten() -> i32 {
    10
}
fn default_forty() -> i32 {
    40
}
fn default_hundred() -> i32 {
    100
}

#[derive(Default, Clone, Debug, Deserialize)]
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
    #[serde(deserialize_with = "de_map_to_scalar")]
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
    #[serde(deserialize_with = "de_map_to_scalar")]
    renaming: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "de_map_to_scalar")]
    scoregraphs: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "de_map_to_scalar")]
    nationinfo: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "de_map_to_scalar")]
    artrest: i32,
    #[serde(default)]
    #[serde(deserialize_with = "de_map_to_scalar")]
    teamgame: i32,
    #[serde(default)]
    #[serde(deserialize_with = "de_map_to_scalar")]
    clustered: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "from_str")]
    storyevents: i32,
    #[serde(default = "default_two")]
    #[serde(deserialize_with = "from_str")]
    newailvl: i32,
    #[serde(default = "default_one")]
    #[serde(deserialize_with = "de_map_to_scalar")]
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

#[derive(Debug, Deserialize)]
struct CreateGame {
    #[serde(default)]
    name: String,
    #[serde(default)]
    #[serde(flatten)]
    settings: GameSettings,
}

fn de_map_to_scalar<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    // define a visitor that deserializes
    // `ActualData` encoded as json within a string
    struct MapVisitor;

    impl<'de> serde::de::Visitor<'de> for MapVisitor {
        type Value = i32;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("A length 1 sequence containing an int")
        }
        fn visit_seq<A>(self, mut v: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut last = Err(A::Error::custom("Zero length seq"));
            while let Some(v) = v.next_element::<String>()? {
                last = Ok(v.parse().unwrap())
            }
            last
        }
    }
    // use our visitor to deserialize an `ActualValue`
    deserializer.deserialize_seq(MapVisitor)
}

#[derive(Debug, Deserialize)]
struct SetTimer {
    timer: i32,
}

#[derive(Template)]
#[template(path = "games/details.html")]
struct GameDetailsTemplate<'a> {
    game: Game,
    settings: GameSettings,
    status: &'a str,
    turn_number: i32,
    turns: HashMap<i32, Vec<PlayerTurn>>,
    hostname: String,
    players: &'a Vec<(i32, String)>,
    mods: &'a Vec<Mod>,
    maps: &'a Vec<Map>,
    map: Map,
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
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}

#[get("/game/{id}")]
async fn details(
    (app_data, web::Path(path_id), game_settings): (
        web::Data<AppData>,
        web::Path<String>,
        serde_qs::actix::QsQuery<GameSettings>,
    ),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let (game, map) = if let Ok(id_i32) = path_id.parse::<i32>() {
        use crate::schema::games::dsl::*;
        games
            .filter(id.eq(id_i32))
            .inner_join(crate::schema::maps::dsl::maps.on(crate::schema::maps::dsl::id.eq(map_id)))
            .get_result::<(Game, Map)>(&*db)
    } else {
        use crate::schema::games::dsl::*;
        let game: Game = games.filter(name.ilike(path_id)).get_result(&*db).unwrap();
        return Ok(HttpResponse::PermanentRedirect()
            .header(header::LOCATION, format!("/game/{}", game.id))
            .finish());
    }
    .unwrap();
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
        (*game_settings).clone()
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
    let (maps, mods): (Vec<Map>, Vec<Mod>) = if turns.len() == 0 {
        (
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
        )
    } else {
        (vec![], vec![])
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (GameDetailsTemplate {
            map,
            settings: settings,
            turns: player_turn_map,
            status: if turns.len() > 0 {
                "Active"
            } else {
                "Waiting for players"
            },
            turn_number: turns.len() as i32,
            maps: &maps,
            mods: &mods,
            game,
            players: &game_players
                .iter()
                .map(|(_, nation)| (nation.nation_id, nation.name.clone()))
                .collect(),
            hostname: std::env::var("HOSTNAME").unwrap(),
        })
        .render()
        .unwrap(),
    ))
}

#[derive(Template)]
#[template(path = "games/create.html")]
struct AddGameTemplate<'a> {
    params: &'a CreateGame,
    maps: &'a [Map],
    mods: &'a [Mod],
}

#[get("/games/create")]
async fn create_get(
    (app_data, params): (web::Data<AppData>, serde_qs::actix::QsQuery<CreateGame>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let result_maps = {
        use crate::schema::maps::dsl::*;
        maps.load::<Map>(&db).unwrap()
    };
    let result_mods = {
        use crate::schema::mods::dsl::*;
        mods.load::<Mod>(&db).unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (AddGameTemplate {
            params: &params,
            maps: &result_maps,
            mods: &result_mods,
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
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}

#[post("/games/create")]
async fn create_post(
    (app_data, mut body): (web::Data<AppData>, web::Payload),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    let config = serde_qs::Config::new(10, false);
    let params: CreateGame = config.deserialize_bytes(&*bytes).unwrap();

    let era = match params.settings.era {
        Era::EARLY => Era::EARLY,
        Era::MIDDLE => Era::MIDDLE,
        Era::LATE => Era::LATE,
        _ => Era::EARLY,
    };

    let new_game = NewGame {
        name: &params.name,
        era,
        map_id: params.settings.map,
        thrones_t1: params.settings.thrones_t1,
        thrones_t2: params.settings.thrones_t2,
        thrones_t3: params.settings.thrones_t3,
        throne_points_required: params.settings.throne_points_required,
        research_diff: params.settings.research_diff,
        research_rand: params.settings.research_rand > 0,
        hof_size: params.settings.hof_size,
        global_size: params.settings.global_size,
        indepstr: params.settings.indepstr,
        magicsites: params.settings.magicsites,
        eventrarity: params.settings.eventrarity,
        richness: params.settings.richness,
        resources: params.settings.resources,
        recruitment: params.settings.recruitment,
        supplies: params.settings.supplies,
        startprov: params.settings.startprov,
        renaming: params.settings.renaming > 0,
        scoregraphs: params.settings.scoregraphs > 0,
        nationinfo: params.settings.nationinfo > 0,
        artrest: params.settings.artrest > 0,
        teamgame: params.settings.teamgame > 0,
        clustered: params.settings.clustered > 0,
        storyevents: params.settings.storyevents,
        newailvl: params.settings.newailvl,
        newai: params.settings.newai > 0,
    };

    let game: Game = diesel::insert_into(crate::schema::games::table)
        .values(&new_game)
        .get_result(&db)
        .expect("Error saving new game");

    diesel::insert_into(crate::schema::game_mods::table)
        .values(
            params
                .settings
                .cmods
                .iter()
                .map(|m| NewGameMod {
                    game_id: game.id,
                    mod_id: *m,
                })
                .collect::<Vec<NewGameMod>>(),
        )
        .get_results::<GameMod>(&db)
        .unwrap();
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::Start(game.id))
        .unwrap();

    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}

#[derive(Template)]
#[template(path = "games/list.html")]
struct GamesTemplate<'a> {
    games: &'a [Game],
}

#[get("/games")]
async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");

    // Create game
    use crate::schema::games::dsl::games;
    let results = games.load::<Game>(&db).expect("Error loading games");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((GamesTemplate { games: &results }).render().unwrap()))
}

#[post("/game/{id}/settings")]
async fn settings_post(
    (app_data, path_id, mut body): (web::Data<AppData>, web::Path<i32>, web::Payload),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    let config = serde_qs::Config::new(10, false);
    let body: GameSettings = config.deserialize_bytes(&*bytes).unwrap();
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
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}

#[derive(Template)]
#[template(path = "games/email.html")]
struct EmailsTemplate<'a> {
    nations: Vec<Nation>,
    form: &'a EmailForm,
    game: Game,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct EmailFormEntry {
    selected_nation: i32,
    remaining_hours: Option<i32>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct EmailForm {
    #[serde(default)]
    email: String,
    #[serde(default)]
    entries: Vec<EmailFormEntry>,
}

#[get("/game/{id}/email")]
async fn emails_get(
    (app_data, mut email_form, web::Path((game_id))): (
        web::Data<AppData>,
        serde_qs::actix::QsQuery<EmailForm>,
        web::Path<(i32)>,
    ),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::email_configs::dsl as emails_dsl;
    use crate::schema::games::dsl as games_dsl;
    use crate::schema::nations::dsl as nations_dsl;
    use crate::schema::players::dsl as players_dsl;
    let invalid: Vec<EmailFormEntry> = email_form
        .entries
        .drain_filter(|e| !e.remaining_hours.is_some())
        .collect();
    let game: Game = games_dsl::games
        .filter(games_dsl::id.eq(game_id))
        .get_result(&db)
        .unwrap();
    let nations: Vec<(Nation, Player)> = nations_dsl::nations
        .filter(nations_dsl::game_id.eq(game_id))
        .inner_join(
            players_dsl::players.on(players_dsl::nationid
                .eq(nations_dsl::nation_id)
                .and(players_dsl::game_id.eq(game.id))),
        )
        .get_results(&db)
        .unwrap();
    // let existing_emails: Vec<EmailConfig> = emails_dsl::email_configs.filter(emails_dsl::email_address.eq(address)).get_results(&db).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (EmailsTemplate {
            form: &email_form,
            game,
            nations: nations.into_iter().map(move |(n, _)| n).collect(),
        })
        .render()
        .unwrap(),
    ))
}
