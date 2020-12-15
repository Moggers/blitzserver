use super::AppData;
use crate::diesel::prelude::*;
use crate::models::{File, Map};
use actix_web::{get, web, HttpResponse, Result};
use askama::Template;
use diesel::RunQueryDsl;

#[derive(Template)]
#[template(path = "maps/details.html")]
pub struct MapDetailsTemplate<'a> {
    map: &'a Map,
}

#[get("/map/{id}")]
pub async fn details(
    (app_data, web::Path(path_id)): (web::Data<AppData>, web::Path<String>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().unwrap();
    let id_i32: i32 = path_id.parse().unwrap();
    let map = crate::schema::maps::dsl::maps
        .filter(crate::schema::maps::dsl::id.eq(id_i32))
        .get_result::<Map>(&db)
        .unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((MapDetailsTemplate { map: &map }).render().unwrap()))
}

#[get("/images/maps/{id}.jpg")]
pub async fn image(
    (app_data, web::Path(path_id)): (web::Data<AppData>, web::Path<String>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().unwrap();
    let id_i32: i32 = path_id.parse().unwrap();
    let (map, file): (Map, File) = crate::schema::maps::dsl::maps
        .filter(crate::schema::maps::dsl::id.eq(id_i32))
        .inner_join(
            crate::schema::files::dsl::files
                .on(crate::schema::files::dsl::id.eq(crate::schema::maps::dsl::tgafile_id)),
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
    if !maps_dir.exists() {
        std::fs::create_dir_all(&maps_dir);
    }
    std::fs::create_dir_all(&maps_dir);
    let mut file = std::fs::File::create(maps_dir.join(format!("{}.jpg", map.id))).unwrap();
    reader
        .write_to(&mut file, crate::image::ImageFormat::Jpeg)
        .unwrap();
    let mut jpg_bytes: Vec<u8> = Vec::new();
    reader
        .write_to(
            &mut std::io::Cursor::new(&mut jpg_bytes),
            crate::image::ImageFormat::Jpeg,
        )
        .unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/jpg")
        .body(jpg_bytes))
}

pub async fn image_handler(
    req: actix_web::dev::ServiceRequest,
) -> impl futures::Future<
    Output = Result<actix_web::dev::ServiceResponse<actix_web::body::Body>, actix_web::Error>,
> {
    let (http_req, _payload) = req.into_parts();
    let map_id_regex = regex::Regex::new(r#"/images/maps/[0-9]+.jpg"#).unwrap();
    if let Some(captures) = map_id_regex.captures(http_req.path()) {}
    async {
        Ok(actix_web::dev::ServiceResponse::new(
            http_req,
            HttpResponse::NotFound().finish(),
        ))
    }
}
