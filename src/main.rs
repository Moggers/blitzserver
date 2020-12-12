#![feature(try_blocks)]
#[macro_use]
extern crate diesel;
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
use std::ops::Deref;

pub mod game_manager;
pub mod models;
pub mod schema;

fn default_one() -> i32 {
    1
}
#[derive(Deserialize)]
struct CreateGame {
    #[serde(default)]
    name: String,
    #[serde(default = "default_one")]
    era: i32,
    #[serde(default = "default_one")]
    map: i32,
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
}

#[derive(Template)]
#[template(path = "game_details.html")]
struct GameDetailsTemplate {
    game: Game,
    hostname: String,
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
    while let Ok(Some(field)) = payload.try_next().await {
        if let Some(_) = badbody {
            continue;
        }
        let content_type = field.content_disposition().unwrap();
        match content_type.get_name() {
            Some("map") => {
                let (res, _) = field.into_future().await;
                if let Some(Ok(content)) = res {
                    let new_file = NewFile {
                        filename: content_type.get_filename().unwrap(),
                        filebinary: &content,
                    };
                    let name: Result<String> = try {
                        let re = regex::bytes::Regex::new(r#"#dom2title ("?[^"\n]+"?)"#)
                            .map_err(|_| ())?;
                        let caps = re.captures(&content).ok_or(())?;
                        String::from_utf8(caps.get(1).ok_or(())?.as_bytes().to_vec())
                            .map_err(|_| ())?
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
                        .get_result(&db)
                        .expect("Error saving file");
                    new_map.mapfile_id = file.id;
                }
            }
            Some("tga") => {
                let (res, _) = field.into_future().await;
                if let Some(Ok(content)) = res {
                    let new_file = NewFile {
                        filename: content_type.get_filename().unwrap(),
                        filebinary: &content,
                    };
                    let file: File = diesel::insert_into(self::schema::files::table)
                        .values(&new_file)
                        .get_result(&db)
                        .expect("Error saving file");
                    new_map.tgafile_id = file.id;
                }
            }
            Some("tga_winter") => {
                let (res, _) = field.into_future().await;
                if let Some(Ok(content)) = res {
                    let new_file = NewFile {
                        filename: content_type.get_filename().unwrap(),
                        filebinary: &content,
                    };
                    let file: File = diesel::insert_into(self::schema::files::table)
                        .values(&new_file)
                        .get_result(&db)
                        .expect("Error saving file");
                    new_map.winterfile_id = file.id;
                }
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
        let map: Map = diesel::insert_into(self::schema::maps::table)
            .values(&new_map)
            .get_result(&db)
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
    use self::schema::games::dsl::*;
    let game = if let Ok(id_i32) = path_id.parse::<i32>() {
        games.filter(id.eq(id_i32)).get_result::<Game>(&*db)
    } else {
        games.filter(name.ilike(path_id)).get_result::<Game>(&*db)
    }
    .unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((GameDetailsTemplate { game, hostname: std::env::var("HOSTNAME").unwrap() }).render().unwrap()))
}

#[get("/games/create")]
async fn add_game_get(
    (params, app_data): (web::Query<CreateGame>, web::Data<AppData>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use self::schema::maps::dsl::*;
    let result_maps = maps
        .filter(name.ilike(format!("%{}%", params.mapfilter)))
        .load::<Map>(&db)
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (AddGameTemplate {
            params: params.deref(),
            maps: &result_maps,
        })
        .render()
        .unwrap(),
    ))
}

#[post("/games/create")]
async fn add_game_post(
    (app_data, params): (web::Data<AppData>, web::Form<CreateGame>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");

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

    app_data.manager_notifier.send(game_manager::ManagerMsg::Start(game.id)).unwrap();

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

#[derive(Clone)]
struct AppData {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    manager_notifier: crossbeam_channel::Sender<game_manager::ManagerMsg>,
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
            .service(get_games)
            .service(add_game_get)
            .service(add_game_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
