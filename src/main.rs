#![feature(duration_constants)]
#![feature(try_blocks)]
#![feature(drain_filter)]
#![feature(async_closure)]
#![feature(type_alias_impl_trait)]
#![feature(get_mut_unchecked)]
#![feature(hash_drain_filter)]
#[macro_use]
extern crate num_enum;
#[macro_use]
extern crate diesel;
extern crate base64;
extern crate byteorder;
extern crate crc;
extern crate crossbeam_channel;
extern crate fletcher;
extern crate image;
extern crate num_derive;
extern crate num_traits;
extern crate reqwest;
extern crate serde_json;
extern crate thiserror;
extern crate tungstenite;
extern crate url;
extern crate zip;

use self::diesel::prelude::*;
use self::models::*;
use actix_web::http::header;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Result};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use game_manager::{GameManager, GameManagerConfig};
use serde::Deserialize;
use std::env;
use std::io::Write;

pub mod discord;
pub mod dom5_emu;
pub mod dom5_proc;
pub mod email_manager;
pub mod frontend;
pub mod game_manager;
pub mod map_file;
pub mod models;
pub mod msgbus;
pub mod packets;
pub mod schema;
pub mod statusdump;
pub mod twoh;
pub mod util;

use frontend::AppData;

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
#[get("/styles.css")]
async fn styles() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/css").body(
        &concat!(
            include_str!("../content/map-list.css"),
            include_str!("../content/mod-list.css"),
            include_str!("../content/styles.css")
        )[..],
    ))
}

#[derive(Deserialize)]
struct StartGame {
    countdown: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // Db
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::new(manager).unwrap().clone()
    };
    let bus = msgbus::MsgBus::new();

    let mut manager = {
        let cfg = GameManagerConfig {
            bus_rx: bus.new_recv(),
            bus_tx: bus.sender.clone(),
            db_pool: &pool.clone(),
            tmp_dir: &env::current_dir().unwrap().join("tmp"),
        };

        GameManager::new(cfg)
    };

    let discord_manager = match discord::DiscordManager::new(bus.sender.clone(), pool.clone()) {
        Ok(d) => {
            match (
                d.clone().monitor_bus(bus.new_recv()),
                d.clone().monitor_discord(),
            ) {
                (Err(e), _) => {
                    log::warn!("Discord integration disabled, {:?}", e);
                    None
                }
                (_, Err(e)) => {
                    log::warn!("Discord integration disabled, {:?}", e);
                    None
                }
                _ => Some(d),
            }
        }
        Err(e) => {
            log::warn!("Discord integration disabled, {:?}", e);
            None
        }
    };
    let app_data = AppData {
        msgbus_sender: bus.sender.clone(),
        pool: pool.clone(),
        discord_manager,
        email_manager: crate::email_manager::EmailManager {
            msgbus_tx: bus.sender.clone(),
            db_pool: pool.clone(),
            smtp_user: env::var("SMTP_USER").expect("SMTP_USER must be said to the SMTP user"),
            smtp_pass: env::var("SMTP_PASS").expect("SMTP_PASS must be said to the SMTP password"),
            smtp_server: env::var("SMTP_SERVER")
                .expect("SMTP_SERVER must be said to the SMTP server"),
            hostname: env::var("HOSTNAME").expect("HOSTNAME must be set to accessible address"),
        },
    };
    crate::email_manager::EmailManager {
        msgbus_tx: bus.sender.clone(),
        db_pool: pool.clone(),
        smtp_user: env::var("SMTP_USER").expect("SMTP_USER must be said to the SMTP user"),
        smtp_pass: env::var("SMTP_PASS").expect("SMTP_PASS must be said to the SMTP password"),
        smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be said to the SMTP server"),
        hostname: env::var("HOSTNAME").expect("HOSTNAME must be set to accessible address"),
    }
    .monitor(bus.new_recv());

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
            .wrap(
                actix_session::CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false),
            )
            .app_data(
                serde_qs::actix::QsQueryConfig::default()
                    .qs_config(serde_qs::Config::new(10, false)),
            )
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
                            let cursor = std::io::Cursor::new(file.filebinary);
                            let reader = crate::image::io::Reader::with_format(
                                cursor.clone(),
                                crate::image::ImageFormat::Tga,
                            )
                            .decode()
                            .unwrap_or_else(|_| {
                                crate::image::io::Reader::with_format(
                                    cursor.clone(),
                                    crate::image::ImageFormat::Sgi,
                                )
                                .decode()
                                .unwrap()
                            });
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
            .service(styles)
            .service(frontend::maps::details)
            .service(frontend::maps::image)
            .service(frontend::maps::upload_post)
            .service(frontend::maps::list)
            .service(frontend::maps::download)
            .service(frontend::games::timer)
            .service(frontend::games::postpone)
            .service(frontend::games::details)
            .service(frontend::games::launch)
            .service(frontend::games::list)
            .service(frontend::games::create_post)
            .service(frontend::games::remove_post)
            .service(frontend::games::settings_post)
            .service(frontend::games::emails_delete)
            .service(frontend::games::emails_post)
            .service(frontend::games::archive_post)
            .service(frontend::games::rollback_post)
            .service(frontend::games::unstart_post)
            .service(frontend::games::assign_team)
            .service(frontend::mods::list)
            .service(frontend::mods::upload_post)
            .service(frontend::mods::details)
            .service(frontend::mods::download)
            .service(frontend::mods::image)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
