use super::AppData;
use crate::diesel::prelude::*;
use crate::game_manager;
use crate::models::{
    Era, Game, GameMod, Map, Mod, Nation, NewGame, NewGameMod, Player, PlayerTurn, Turn,
};
use crate::StartGame;
use actix_web::http::header;
use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use futures::StreamExt;
use serde::Deserialize;
use std::collections::HashMap;

fn default_one() -> i32 {
    1
}

#[derive(Debug, Deserialize)]
struct CreateGame {
    #[serde(default)]
    name: String,
    #[serde(default = "default_one")]
    era: i32,
    #[serde(default = "default_one")]
    map: i32,
    #[serde(default)]
    cmods: Vec<i32>,
    #[serde(default)]
    mapfilter: String,
    #[serde(default)]
    modfilter: String,
}

#[derive(Debug, Deserialize)]
struct SetTimer {
    timer: i32,
}

#[derive(Template)]
#[template(path = "games/list.html")]
struct GamesTemplate<'a> {
    games: &'a [Game],
}

#[derive(Template)]
#[template(path = "games/create.html")]
struct AddGameTemplate<'a> {
    params: &'a CreateGame,
    maps: &'a [Map],
    mods: &'a [Mod],
}

#[derive(Template)]
#[template(path = "games/details.html")]
struct GameDetailsTemplate<'a> {
    game: Game,
    status: &'a str,
    turn_number: i32,
    turns: HashMap<i32, Vec<PlayerTurn>>,
    hostname: String,
    players: &'a Vec<(i32, String)>,
    mods: &'a Vec<String>,
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
            .set(timer.eq(Some(timer_form.timer)))
            .execute(&db)
            .unwrap();
    }
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::GameMsg(
            self::game_manager::GameMsg {
                id: game.id,
                cmd: self::game_manager::GameCmd::SetTimerCmd,
            },
        ))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}

#[get("/game/{id}")]
async fn details(
    (app_data, web::Path(path_id)): (web::Data<AppData>, web::Path<String>),
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
        games
            .filter(name.ilike(path_id))
            .inner_join(crate::schema::maps::dsl::maps.on(crate::schema::maps::dsl::id.eq(map_id)))
            .get_result::<(Game, Map)>(&*db)
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
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (GameDetailsTemplate {
            map,
            turns: player_turn_map,
            status: if turns.len() > 0 {
                "Active"
            } else {
                "Waiting for plyers "
            },
            turn_number: turns.len() as i32,
            mods: &crate::schema::game_mods::dsl::game_mods
                .filter(crate::schema::game_mods::dsl::game_id.eq(game.id))
                .inner_join(
                    crate::schema::mods::dsl::mods
                        .on(crate::schema::mods::dsl::id.eq(crate::schema::game_mods::dsl::mod_id)),
                )
                .get_results::<(GameMod, Mod)>(&db)
                .unwrap()
                .iter()
                .map(|(_, m)| m.name.clone())
                .collect::<Vec<String>>(),
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

#[get("/games/create")]
async fn create_get(
    (app_data, params): (web::Data<AppData>, serde_qs::actix::QsQuery<CreateGame>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let result_maps = {
        use crate::schema::maps::dsl::*;
        maps.filter(name.ilike(format!("%{}%", params.mapfilter)))
            .load::<Map>(&db)
            .unwrap()
    };
    let result_mods = {
        use crate::schema::mods::dsl::*;
        mods.load::<Mod>(&db).unwrap()
    };
    println!("What: {:?}", params);
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

    let era = match params.era {
        Era::EARLY => Era::EARLY,
        Era::MIDDLE => Era::MIDDLE,
        Era::LATE => Era::LATE,
        _ => Era::EARLY,
    };

    let new_game = NewGame {
        name: &params.name,
        era,
        map_id: params.map,
    };

    let game: Game = diesel::insert_into(crate::schema::games::table)
        .values(&new_game)
        .get_result(&db)
        .expect("Error saving new game");

    diesel::insert_into(crate::schema::game_mods::table)
        .values(
            params
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

#[post("/game/{id}/launch")]
async fn launch(
    (app_data, web::Path(path_id), form): (
        web::Data<AppData>,
        web::Path<String>,
        web::Form<StartGame>,
    ),
) -> Result<HttpResponse> {
    let game = {
        let db = app_data.pool.get().expect("Unable to connect to database");
        if let Ok(id_i32) = path_id.parse::<i32>() {
            use crate::schema::games::dsl::*;
            games
                .filter(id.eq(id_i32))
                .get_result::<Game>(&*db)
                .unwrap()
        } else {
            use crate::schema::games::dsl::*;
            games
                .filter(name.ilike(path_id))
                .get_result::<Game>(&*db)
                .unwrap()
        }
    };
    app_data
        .manager_notifier
        .send(game_manager::ManagerMsg::GameMsg(
            self::game_manager::GameMsg {
                id: game.id,
                cmd: self::game_manager::GameCmd::LaunchCmd(self::game_manager::LaunchCmd {
                    countdown: std::time::Duration::from_secs(form.countdown),
                }),
            },
        ))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}
