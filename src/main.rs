#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#[macro_use]
extern crate num_enum;
#[macro_use]
extern crate diesel;
extern crate byteorder;
extern crate image;
extern crate zip;
use self::diesel::prelude::*;
use self::models::*;
use actix_web::http::header;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use game_manager::{GameManager, GameManagerConfig, ManagerMsg};
use serde::Deserialize;
use std::env;
use std::io::Write;

pub mod frontend;
pub mod game_manager;
pub mod models;
pub mod schema;
pub mod statusdump;
pub mod twoh;

use frontend::AppData;

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
}

#[derive(Template)]
#[template(path = "games.html")]
struct GamesTemplate<'a> {
    games: &'a [Game],
}

#[derive(Template)]
#[template(path = "add_game.html")]
struct AddGameTemplate<'a> {
    params: &'a CreateGame,
    maps: &'a [Map],
    mods: &'a [Mod],
}

#[derive(Template)]
#[template(path = "game_details.html")]
struct GameDetailsTemplate<'a> {
    game: Game,
    hostname: String,
    players: &'a Vec<String>,
    mods: &'a Vec<String>,
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::PermanentRedirect()
        .header(header::LOCATION, "/games")
        .finish())
}

#[get("/favicon.ico")]
async fn favicon() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(&include_bytes!("../content/favicon.ico")[..]))
}

#[get("/game/{id}")]
async fn game_details_get(
    (app_data, web::Path(path_id)): (web::Data<AppData>, web::Path<String>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let game = if let Ok(id_i32) = path_id.parse::<i32>() {
        use self::schema::games::dsl::*;
        games.filter(id.eq(id_i32)).get_result::<Game>(&*db)
    } else {
        use self::schema::games::dsl::*;
        games.filter(name.ilike(path_id)).get_result::<Game>(&*db)
    }
    .unwrap();
    let game_players: Vec<(Player, Nation)> = {
        use self::schema::players::dsl::*;
        Player::belonging_to(&game)
            .inner_join(
                self::schema::nations::dsl::nations.on(self::schema::nations::dsl::game_id
                    .eq(game.id)
                    .and(self::schema::nations::dsl::nation_id.eq(nationid))),
            )
            .get_results(&db)
            .unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (GameDetailsTemplate {
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
                .map(|(_, nation)| nation.name.clone())
                .collect(),
            hostname: std::env::var("HOSTNAME").unwrap(),
        })
        .render()
        .unwrap(),
    ))
}

#[get("/games/create")]
async fn add_game_get(
    (req, app_data): (web::HttpRequest, web::Data<AppData>),
) -> Result<HttpResponse> {
    let config = serde_qs::Config::new(10, false);
    let params: CreateGame = config.deserialize_str(req.query_string()).unwrap();
    let db = app_data.pool.get().expect("Unable to connect to database");
    use self::schema::maps::dsl::*;
    let result_maps = maps
        .filter(name.ilike(format!("%{}%", params.mapfilter)))
        .load::<Map>(&db)
        .unwrap();
    let result_mods = self::schema::mods::dsl::mods.load::<Mod>(&db).unwrap();
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
async fn add_game_post(
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

    let game: Game = diesel::insert_into(self::schema::games::table)
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
async fn get_games(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");

    // Create game
    use self::schema::games::dsl::games;
    let results = games.load::<Game>(&db).expect("Error loading games");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((GamesTemplate { games: &results }).render().unwrap()))
}

#[derive(Deserialize)]
struct StartGame {
    countdown: u64,
}

#[post("/game/{id}/launch")]
async fn game_launch_post(
    (app_data, web::Path(path_id), form): (
        web::Data<AppData>,
        web::Path<String>,
        web::Form<StartGame>,
    ),
) -> Result<HttpResponse> {
    let game = {
        let db = app_data.pool.get().expect("Unable to connect to database");
        if let Ok(id_i32) = path_id.parse::<i32>() {
            use self::schema::games::dsl::*;
            games
                .filter(id.eq(id_i32))
                .get_result::<Game>(&*db)
                .unwrap()
        } else {
            use self::schema::games::dsl::*;
            games
                .filter(name.ilike(path_id))
                .get_result::<Game>(&*db)
                .unwrap()
        }
    };
    app_data
        .manager_notifier
        .send(ManagerMsg::GameMsg(self::game_manager::GameMsg {
            id: game.id,
            cmd: self::game_manager::GameCmd::LaunchCmd(self::game_manager::LaunchCmd {
                countdown: std::time::Duration::from_secs(form.countdown),
            }),
        }))
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, format!("/game/{}", game.id))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Db
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::new(manager).unwrap().clone()
    };

    let mut manager = {
        let port_var = env::var("PORT_RANGE").expect("PORT_RANGE must be set (ie. '10000,11000')");
        let range: Vec<&str> = port_var.split(",").collect();
        let cfg = GameManagerConfig {
            db_pool: &pool.clone(),
            tmp_dir: &env::current_dir().unwrap().join("tmp"),
            dom5_bin: &std::path::PathBuf::from(env::var("DOM5_BIN").expect("DOM5_BIN mus be set")),
            port_range: &[
                range[0].parse::<i32>().unwrap(),
                range[1].parse::<i32>().unwrap(),
            ],
        };

        GameManager::new(&cfg)
    };

    let app_data = AppData {
        pool: pool.clone(),
        manager_notifier: manager.get_sender(),
    };

    std::thread::spawn(move || {
        manager.start();
    });

    HttpServer::new(move || {

        // TODO: Hack here, we create the map thumbnail dir VERY ahead of time since the file
        // watcher needs it to be there otherwise it wont discover new files
        let maps_dir = std::path::PathBuf::from("./images/maps");
        if !maps_dir.exists() {
            std::fs::create_dir_all(&maps_dir).unwrap();
        }

        App::new()
            .wrap(middleware::Logger::default())
            .data(app_data.clone())
            .service(
                actix_files::Files::new("/images/maps", "./images/maps")
                    .show_files_listing()
                    .default_handler(|req: actix_web::dev::ServiceRequest| async {
                        let app_data = req
                            .app_data::<actix_web::web::Data<self::frontend::AppData>>()
                            .unwrap();
                        let db = app_data.pool.get().unwrap();
                        let (http_req, _payload) = req.into_parts();
                        let map_id_regex =
                            regex::Regex::new(r#"/images/maps/([0-9]+).jpg"#).unwrap();
                        if let Some(captures) = map_id_regex.captures(http_req.path()) {
                            let id_i32: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                            let (map, file): (Map, File) = crate::schema::maps::dsl::maps
                                .filter(crate::schema::maps::dsl::id.eq(id_i32))
                                .inner_join(
                                    crate::schema::files::dsl::files
                                        .on(crate::schema::files::dsl::id
                                            .eq(crate::schema::maps::dsl::tgafile_id)),
                                )
                                .get_result::<(Map, File)>(&db)
                                .unwrap();
                            let reader = crate::image::io::Reader::with_format(
                                std::io::Cursor::new(file.filebinary),
                                crate::image::ImageFormat::Tga,
                            )
                            .decode()
                            .unwrap();
                            let maps_dir = std::path::PathBuf::from("./images/maps");
                            let mut file =
                                std::fs::File::create(maps_dir.join(format!("{}.jpg", map.id)))
                                    .unwrap();
                            let mut jpg_bytes: Vec<u8> = Vec::new();
                            reader
                                .write_to(
                                    &mut std::io::Cursor::new(&mut jpg_bytes),
                                    crate::image::ImageOutputFormat::Jpeg(30),
                                )
                                .unwrap();
                            file.write(&jpg_bytes).unwrap();

                            Ok(actix_web::dev::ServiceResponse::new(
                                http_req,
                                HttpResponse::Ok()
                                    .content_type("application/jpg")
                                    .body(jpg_bytes),
                            ))
                        } else {
                            Ok(actix_web::dev::ServiceResponse::new(
                                http_req,
                                HttpResponse::NotFound().finish(),
                            ))
                        }
                    }),
            )
            .service(index)
            .service(favicon)
            .service(frontend::maps::details)
            .service(frontend::maps::image)
            .service(frontend::maps::upload_get)
            .service(frontend::maps::upload_post)
            .service(frontend::maps::list)
            .service(frontend::maps::download)
            .service(game_details_get)
            .service(game_launch_post)
            .service(get_games)
            .service(add_game_get)
            .service(add_game_post)
            .service(frontend::mods::list)
            .service(frontend::mods::upload_get)
            .service(frontend::mods::upload_post)
            .service(frontend::mods::details)
            .service(frontend::mods::download)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
