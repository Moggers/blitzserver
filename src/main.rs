#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#[macro_use]
extern crate num_enum;
#[macro_use]
extern crate diesel;
extern crate byteorder;
extern crate zip;
use self::diesel::prelude::*;
use self::models::*;
use actix_multipart::Multipart;
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

#[derive(Template)]
#[template(path = "maps/list_maps.html")]
struct ListMapsTemplate<'a> {
    maps: &'a [Map],
}

#[derive(Template)]
#[template(path = "maps/upload.html")]
struct UploadMapTemplate<'a> {
    map_err: &'a str,
    tga_err: &'a str,
    winter_err: &'a str,
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

#[get("/maps")]
async fn get_maps(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use self::schema::maps::dsl::*;
    let result_maps = maps.load::<Map>(&db).expect("Error loading games");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((ListMapsTemplate { maps: &result_maps }).render().unwrap()))
}

#[get("/maps/upload")]
async fn upload_map_get() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (UploadMapTemplate {
            map_err: "",
            tga_err: "",
            winter_err: "",
        })
        .render()
        .unwrap(),
    ))
}

#[post("/maps/upload")]
async fn upload_map_post(
    (app_data, mut payload): (web::Data<AppData>, Multipart),
) -> Result<HttpResponse> {
    let mut new_map = NewMap {
        name: String::new(),
        mapfile_id: 0,
        tgafile_id: 0,
        winterfile_id: 0,
    };
    let mut badbody: Option<UploadMapTemplate> = None;
    let db = app_data.pool.get().expect("Unable to connect to database");
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some(_) = badbody {
            continue;
        }
        let content_type = field.content_disposition().unwrap();
        match content_type.get_name() {
            Some("map") => {
                let mut contents: Vec<u8> = vec![];
                while let Some(bytes) = field.next().await {
                    contents.extend_from_slice(&bytes.unwrap());
                }
                let new_file = NewFile::new(content_type.get_filename().unwrap(), &contents);
                let name: Result<String> = try {
                    let re =
                        regex::bytes::Regex::new(r#"#dom2title ("?[^"\n]+"?)"#).map_err(|_| ())?;
                    let caps = re.captures(&contents).ok_or(())?;
                    String::from_utf8(caps.get(1).ok_or(())?.as_bytes().to_vec()).map_err(|_| ())?
                };
                match name {
                    Ok(name) => new_map.name = name,
                    Err(_) => {
                        badbody = Some(UploadMapTemplate {
                            map_err: "Unable to find #dom2title, is this a valid .map file?",
                            tga_err: "",
                            winter_err: "",
                        })
                    }
                }
                let file: File = diesel::insert_into(self::schema::files::table)
                    .values(&new_file)
                    .on_conflict(self::schema::files::dsl::hash)
                    .do_update()
                    .set(self::schema::files::dsl::filename.eq(self::schema::files::dsl::filename)) // Bogus update so return row gets populated with existing stuff
                    .get_result(&db)
                    .unwrap();
                new_map.mapfile_id = file.id;
            }
            Some("tga") => {
                let mut contents: Vec<u8> = vec![];
                while let Some(bytes) = field.next().await {
                    contents.extend_from_slice(&bytes.unwrap());
                }
                let new_file = NewFile::new(content_type.get_filename().unwrap(), &contents);
                let file: File = diesel::insert_into(self::schema::files::table)
                    .values(&new_file)
                    .on_conflict(self::schema::files::dsl::hash)
                    .do_update()
                    .set(self::schema::files::dsl::filename.eq(self::schema::files::dsl::filename)) // Bogus update so return row gets populated with existing stuff
                    .get_result(&db)
                    .unwrap();
                new_map.tgafile_id = file.id;
            }
            Some("tga_winter") => {
                let mut contents: Vec<u8> = vec![];
                while let Some(bytes) = field.next().await {
                    contents.extend_from_slice(&bytes.unwrap());
                }
                let new_file = NewFile::new(content_type.get_filename().unwrap(), &contents);
                let file: File = diesel::insert_into(self::schema::files::table)
                    .values(&new_file)
                    .on_conflict(self::schema::files::dsl::hash)
                    .do_update()
                    .set(self::schema::files::dsl::filename.eq(self::schema::files::dsl::filename)) // Bogus update so return row gets populated with existing stuff
                    .get_result(&db)
                    .unwrap();
                new_map.winterfile_id = file.id;
            }
            _ => {}
        };
    }
    if let None = badbody {
        if new_map.winterfile_id == 0 || new_map.mapfile_id == 0 || new_map.tgafile_id == 0 {
            badbody = Some(UploadMapTemplate {
                map_err: if new_map.mapfile_id == 0 {
                    "No map file"
                } else {
                    ""
                },
                tga_err: if new_map.tgafile_id == 0 {
                    "No tga file"
                } else {
                    ""
                },
                winter_err: if new_map.winterfile_id == 0 {
                    "No winter file"
                } else {
                    ""
                },
            });
        }
    }

    if let Some(badbody) = badbody {
        Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body(badbody.render().unwrap()))
    } else {
        diesel::insert_into(self::schema::maps::table)
            .values(&new_map)
            .get_result::<Map>(&db)
            .expect("Error saving file");
        Ok(HttpResponse::Found()
            .header(header::LOCATION, "/maps")
            .finish())
    }
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
            .unwrap().iter().map(|(_, m)| { m.name.clone() }).collect::<Vec<String>>(),
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
        .values(params.cmods.iter().map(|m| NewGameMod { game_id: game.id, mod_id: *m }).collect::<Vec<NewGameMod>>())
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
        App::new()
            .wrap(middleware::Logger::default())
            .data(app_data.clone())
            .service(index)
            .service(favicon)
            .service(upload_map_get)
            .service(upload_map_post)
            .service(get_maps)
            .service(game_details_get)
            .service(game_launch_post)
            .service(get_games)
            .service(add_game_get)
            .service(add_game_post)
            .service(frontend::mods::list_mods)
            .service(frontend::mods::upload_mod_get)
            .service(frontend::mods::upload_mod_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
