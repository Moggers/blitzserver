use super::AppData;
use crate::diesel::prelude::*;
use crate::models::{File, Map, NewFile, NewMap};
use actix_multipart::Multipart;
use actix_web::http::header;
use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use diesel::RunQueryDsl;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

#[derive(Template)]
#[template(path = "maps/details.html")]
pub struct MapDetailsTemplate<'a> {
    map: &'a Map,
}

#[get("/map/{id}")]
pub async fn details(
    (app_data, web::Path(path_id)): (web::Data<AppData>, web::Path<i32>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().unwrap();
    let map = crate::schema::maps::dsl::maps
        .filter(crate::schema::maps::dsl::id.eq(path_id))
        .get_result::<Map>(&db)
        .unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((MapDetailsTemplate { map: &map }).render().unwrap()))
}

#[get("/images/maps/{id}.jpg")]
pub async fn image(
    (app_data, web::Path(path_id)): (web::Data<AppData>, web::Path<i32>),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().unwrap();
    let (map, file): (Map, File) = crate::schema::maps::dsl::maps
        .filter(crate::schema::maps::dsl::id.eq(path_id))
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
        std::fs::create_dir_all(&maps_dir).unwrap();
    }
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

#[derive(Template)]
#[template(path = "maps/list.html")]
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

#[get("/map/{id}/download")]
async fn download((app_data, mapid): (web::Data<AppData>, web::Path<i32>)) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let (_map,zipfile) = crate::schema::maps::dsl::maps.filter(crate::schema::maps::dsl::id.eq(mapid.0)).inner_join(crate::schema::files::dsl::files.on(crate::schema::files::dsl::id.eq(crate::schema::maps::dsl::archive_id))).get_result::<(Map, File)>(&db).unwrap();

    Ok(HttpResponse::Ok().content_type("application/zip").body(zipfile.filebinary))
}

#[get("/maps")]
async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::maps::dsl::*;
    let result_maps = maps.load::<Map>(&db).expect("Error loading games");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body((ListMapsTemplate { maps: &result_maps }).render().unwrap()))
}

#[get("/maps/upload")]
async fn upload_get() -> Result<HttpResponse> {
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
async fn upload_post(
    (app_data, mut payload): (web::Data<AppData>, Multipart),
) -> Result<HttpResponse> {
    let mut new_map = NewMap {
        name: String::new(),
        mapfile_id: 0,
        tgafile_id: 0,
        winterfile_id: 0,
        archive_id: 0,
    };
    let mut badbody: Option<UploadMapTemplate> = None;
    let db = app_data.pool.get().expect("Unable to connect to database");

    let mut bytes: Vec<u8> = vec![];
    {
        let mut map_archive = zip::ZipWriter::new(std::io::Cursor::new(&mut bytes));
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
                        let re = regex::bytes::Regex::new(r#"#dom2title ("?[^"\n]+"?)"#)
                            .map_err(|_| ())?;
                        let caps = re.captures(&contents).ok_or(())?;
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
                    map_archive
                        .start_file(
                            content_type.get_filename().unwrap(),
                            zip::write::FileOptions::default()
                                .compression_method(zip::CompressionMethod::Stored)
                                .unix_permissions(0o755),
                        )
                        .unwrap();
                    map_archive.write_all(&contents).unwrap();
                    let file: File = diesel::insert_into(crate::schema::files::table)
                        .values(&new_file)
                        .on_conflict(crate::schema::files::dsl::hash)
                        .do_update()
                        .set(
                            crate::schema::files::dsl::filename
                                .eq(crate::schema::files::dsl::filename),
                        ) // Bogus update so return row gets populated with existing stuff
                        .get_result(&db)
                        .unwrap();
                    new_map.mapfile_id = file.id;
                }
                Some("tga") => {
                    let mut contents: Vec<u8> = vec![];
                    while let Some(bytes) = field.next().await {
                        contents.extend_from_slice(&bytes.unwrap());
                    }
                    map_archive
                        .start_file(
                            content_type.get_filename().unwrap(),
                            zip::write::FileOptions::default()
                                .compression_method(zip::CompressionMethod::Stored)
                                .unix_permissions(0o755),
                        )
                        .unwrap();
                    map_archive.write_all(&contents).unwrap();
                    let new_file = NewFile::new(content_type.get_filename().unwrap(), &contents);
                    let file: File = diesel::insert_into(crate::schema::files::table)
                        .values(&new_file)
                        .on_conflict(crate::schema::files::dsl::hash)
                        .do_update()
                        .set(
                            crate::schema::files::dsl::filename
                                .eq(crate::schema::files::dsl::filename),
                        ) // Bogus update so return row gets populated with existing stuff
                        .get_result(&db)
                        .unwrap();
                    new_map.tgafile_id = file.id;
                }
                Some("tga_winter") => {
                    let mut contents: Vec<u8> = vec![];
                    while let Some(bytes) = field.next().await {
                        contents.extend_from_slice(&bytes.unwrap());
                    }
                    map_archive
                        .start_file(
                            content_type.get_filename().unwrap(),
                            zip::write::FileOptions::default()
                                .compression_method(zip::CompressionMethod::Stored)
                                .unix_permissions(0o755),
                        )
                        .unwrap();
                    map_archive.write_all(&contents).unwrap();
                    let new_file = NewFile::new(content_type.get_filename().unwrap(), &contents);
                    let file: File = diesel::insert_into(crate::schema::files::table)
                        .values(&new_file)
                        .on_conflict(crate::schema::files::dsl::hash)
                        .do_update()
                        .set(
                            crate::schema::files::dsl::filename
                                .eq(crate::schema::files::dsl::filename),
                        ) // Bogus update so return row gets populated with existing stuff
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
        map_archive.finish().unwrap();
    }
    let zip_name = format!("{}.zip", &new_map.name);
    let new_file = NewFile::new(&zip_name, &bytes);
    let file: File = diesel::insert_into(crate::schema::files::table)
        .values(&new_file)
        .on_conflict(crate::schema::files::dsl::hash)
        .do_update()
        .set(crate::schema::files::dsl::filename.eq(crate::schema::files::dsl::filename)) // Bogus update so return row gets populated with existing stuff
        .get_result(&db)
        .unwrap();
    new_map.archive_id = file.id;

    if let Some(badbody) = badbody {
        Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body(badbody.render().unwrap()))
    } else {
        diesel::insert_into(crate::schema::maps::table)
            .values(&new_map)
            .get_result::<Map>(&db)
            .expect("Error saving file");
        Ok(HttpResponse::Found()
            .header(header::LOCATION, "/maps")
            .finish())
    }
}
