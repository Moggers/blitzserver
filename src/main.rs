#![feature(try_blocks)]
#![feature(drain_filter)]
#![feature(async_closure)]
#![feature(type_alias_impl_trait)]
#![feature(get_mut_unchecked)]
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
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Result};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use game_manager::{GameManager, GameManagerConfig};
use serde::Deserialize;
use std::env;
use std::io::Write;

pub mod dom5_proc;
pub mod dom5_proxy;
pub mod email_manager;
pub mod frontend;
pub mod game_manager;
pub mod models;
pub mod schema;
pub mod statusdump;
pub mod twoh;

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

    let mut manager = {
        let port_var = env::var("PORT_RANGE").expect("PORT_RANGE must be set (ie. '10000,10999')");
        let internal_port_var = env::var("INTERNAL_PORT_RANGE")
            .expect("INTERNAL_PORT_RANGE must be set (ie. '11000,11999')");
        let range: Vec<&str> = port_var.split(",").collect();
        let internal_range: Vec<&str> = internal_port_var.split(",").collect();
        let cfg = GameManagerConfig {
            db_pool: &pool.clone(),
            tmp_dir: &env::current_dir().unwrap().join("tmp"),
            dom5_bin: &std::path::PathBuf::from(env::var("DOM5_BIN").expect("DOM5_BIN mus be set")),
            port_range: &[
                range[0].parse::<i32>().unwrap(),
                range[1].parse::<i32>().unwrap(),
            ],
            internal_port_range: &[
                internal_range[0].parse::<i32>().unwrap(),
                internal_range[1].parse::<i32>().unwrap(),
            ],
        };

        GameManager::new(&cfg)
    };

    let app_data = AppData {
        pool: pool.clone(),
        manager_notifier: manager.get_sender(),
        email_manager: crate::email_manager::EmailManager {
            db_pool: pool.clone(),
            smtp_user: env::var("SMTP_USER").expect("SMTP_USER must be said to the SMTP user"),
            smtp_pass: env::var("SMTP_PASS").expect("SMTP_PASS must be said to the SMTP password"),
            smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be said to the SMTP server"),
            hostname: env::var("HOSTNAME").expect("HOSTNAME must be set to accessible address")
        },
    };
    app_data.email_manager.monitor();

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
            .service(frontend::games::timer)
            .service(frontend::games::details)
            .service(frontend::games::launch)
            .service(frontend::games::list)
            .service(frontend::games::create_get)
            .service(frontend::games::create_post)
            .service(frontend::games::settings_post)
            .service(frontend::games::emails_get)
            .service(frontend::games::emails_post)
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
