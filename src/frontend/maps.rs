use super::AppData;
use crate::diesel::prelude::*;
use crate::image::GenericImageView;
use crate::models::{File, Map, NewFile, NewMap};
use actix_multipart::Multipart;
use actix_web::http::header;
use actix_web::{get, post, web, HttpResponse, Result};
use askama::Template;
use diesel::RunQueryDsl;
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::convert::TryFrom;
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
    errors: &'a [String],
}

#[get("/map/{id}/download")]
async fn download((app_data, mapid): (web::Data<AppData>, web::Path<i32>)) -> Result<HttpResponse> {
    let mut bytes: Vec<u8> = vec![];
    let mut map_archive = zip::ZipWriter::new(std::io::Cursor::new(&mut bytes));

    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::files::dsl as files_dsl;
    use crate::schema::maps::dsl as maps_dsl;
    let map: Map = maps_dsl::maps
        .filter(maps_dsl::id.eq(mapid.0))
        .get_result(&db)
        .unwrap();

    let mapfile: File = files_dsl::files
        .filter(files_dsl::id.eq(map.mapfile_id))
        .get_result(&db)
        .unwrap();
    map_archive
        .start_file(
            mapfile.filename,
            zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755),
        )
        .unwrap();
    map_archive.write_all(&mapfile.filebinary).unwrap();
    let tgafile: File = files_dsl::files
        .filter(files_dsl::id.eq(map.tgafile_id))
        .get_result(&db)
        .unwrap();
    map_archive
        .start_file(
            tgafile.filename,
            zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755),
        )
        .unwrap();
    map_archive.write_all(&tgafile.filebinary).unwrap();
    if let Some(winterfile_id) = map.winterfile_id {
        let winterfile: File = files_dsl::files
            .filter(files_dsl::id.eq(winterfile_id))
            .get_result(&db)
            .unwrap();
        map_archive
            .start_file(
                &winterfile.filename,
                zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Stored)
                    .unix_permissions(0o755),
            )
            .unwrap();
        map_archive.write_all(&winterfile.filebinary).unwrap();
    }
    drop(map_archive);

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .body(bytes))
}

#[get("/maps")]
async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    use crate::schema::files::dsl as files_dsl;
    use crate::schema::maps::dsl as maps_dsl;
    let result_maps = maps_dsl::maps
        .load::<Map>(&db)
        .expect("Error loading games")
        .iter()
        .map(|m| {
            if m.province_count == 0 && m.uw_count == 0 {
                let map_file: File = files_dsl::files
                    .filter(files_dsl::id.eq(m.mapfile_id))
                    .get_result(&db)
                    .unwrap();
                let map_definition =
                    crate::map_file::MapFile::try_from(&map_file.filebinary[..]).unwrap();
                diesel::update(maps_dsl::maps.filter(maps_dsl::id.eq(m.id)))
                    .set((
                        maps_dsl::province_count.eq(map_definition.prov_count),
                        maps_dsl::uw_count.eq(map_definition.uwprov_count),
                    ))
                    .get_result::<Map>(&db)
                    .unwrap()
            } else {
                m.clone()
            }
        })
        .collect::<Vec<Map>>();
    Ok(HttpResponse::Ok().content_type("text/html").body(
        (ListMapsTemplate {
            errors: &vec![],
            maps: &result_maps,
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
        winterfile_id: None,
        province_count: 0,
        uw_count: 0,
    };
    let db = app_data.pool.get().expect("Unable to connect to database");
    let mut tga_name: Option<String> = None;
    let mut winter_name: Option<String> = None;
    let mut map_file_data: Option<crate::map_file::MapFile> = None;

    let mut errors: Vec<String> = vec![];
    {
        while let Ok(Some(mut field)) = payload.try_next().await {
            if errors.len() > 0 {
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
                    match crate::map_file::MapFile::try_from(&contents[..]) {
                        Ok(mfd) => {
                            if tga_name.is_some() && *tga_name.as_ref().unwrap() != mfd.tga_filename
                            {
                                errors.push(
                                    "#imagefile in .map does not match filename of uploaded image"
                                        .to_owned(),
                                );
                            } else if winter_name.is_some() == mfd.winter_filename.is_some()
                                && mfd.winter_filename != mfd.winter_filename
                            {
                                errors.push("#winterimagefile in .map does not match filename of uploaded winter image".to_owned());
                            } else {
                                new_map.name = mfd.map_name.clone();
                                new_map.province_count = mfd.prov_count;
                                new_map.uw_count = mfd.uwprov_count;
                                map_file_data = Some(mfd);
                            }
                        }
                        Err(e) => {
                            errors.push(e.to_string());
                            break;
                        }
                    }
                    // == DONE
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
                    let filename = content_type.get_filename().unwrap();
                    match map_file_data {
                        None => {
                            tga_name = Some(filename.to_string());
                        }
                        Some(ref mut mfd) => {
                            if mfd.tga_filename.trim() != filename.trim() {
                                errors.push(format!(
                                    "Map #imagefile is {}, but TGA filename is {}",
                                    mfd.tga_filename, filename
                                ));
                                break;
                            }
                        }
                    }
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
                    let filename = content_type.get_filename().unwrap();
                    if filename != "" {
                        match map_file_data {
                            None => winter_name = Some(filename.to_string()),
                            Some(ref mfd) => match mfd.winter_filename {
                                None => {
                                    errors.push(format!("Map does not contain a #winterimagefile, but {} has been uploaded as one", filename));
                                    break;
                                }
                                Some(ref winter_filename) => {
                                    if winter_filename.trim() != filename.trim() {
                                        errors.push(format!(
                                            "Map #winterimagefile is {}, but TGA filename is {}",
                                            winter_filename, filename
                                        ));
                                        break;
                                    }
                                }
                            },
                        }
                        let new_file = NewFile::new(filename, &contents);
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
                        new_map.winterfile_id = Some(file.id);
                    }
                }
                _ => {}
            };
        }
    }
    if errors.len() == 0 {
        if new_map.mapfile_id == 0 {
            errors.push("Missing map file\n".to_string());
        }
        if new_map.tgafile_id == 0 {
            errors.push("Missing tga file\n".to_string());
        }
        if map_file_data.as_ref().unwrap().winter_filename.is_some()
            && !new_map.winterfile_id.is_some()
        {
            errors.push(format!(
                "Map contains #winterimagefile {}, but none has been provided",
                map_file_data
                    .as_ref()
                    .unwrap()
                    .winter_filename
                    .as_ref()
                    .unwrap()
                    .clone()
            ));
        }
    }
    if errors.len() > 0 {
        let db = app_data.pool.get().expect("Unable to connect to database");
        use crate::schema::maps::dsl::*;
        let result_maps = maps.load::<Map>(&db).expect("Error loading games");
        return Ok(HttpResponse::Ok().content_type("text/html").body(
            (ListMapsTemplate {
                errors: &errors,
                maps: &result_maps,
            })
            .render()
            .unwrap(),
        ));
    } else {
        use crate::schema::maps::dsl as maps_dsl;
        diesel::insert_into(maps_dsl::maps)
            .values(&new_map)
            .on_conflict((
                maps_dsl::mapfile_id,
                maps_dsl::tgafile_id,
                maps_dsl::winterfile_id,
            ))
            .do_update()
            .set((
                maps_dsl::mapfile_id.eq(new_map.mapfile_id),
                maps_dsl::tgafile_id.eq(new_map.tgafile_id),
                maps_dsl::winterfile_id.eq(new_map.winterfile_id),
            ))
            .get_result::<Map>(&db)
            .expect("Error saving file");
        return Ok(HttpResponse::Found()
            .header(header::LOCATION, "/maps")
            .finish());
    }
}

pub async fn map_thumb_handler(
    req: actix_web::dev::ServiceRequest,
) -> actix_web::Result<actix_web::dev::ServiceResponse> {
    let app_data = req
        .app_data::<actix_web::web::Data<crate::frontend::AppData>>()
        .unwrap();
    let db = app_data.pool.get().unwrap();
    let (http_req, _payload) = req.into_parts();
    let map_id_regex = regex::Regex::new(r#"/images/maps/([0-9]+)_([0-9]+)-([0-9]+).jpg"#).unwrap();
    if let Some(captures) = map_id_regex.captures(http_req.path()) {
        let id_i32: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let width: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let height: u32 = captures.get(3).unwrap().as_str().parse().unwrap();
        let (map, file): (Map, File) = crate::schema::maps::dsl::maps
            .filter(crate::schema::maps::dsl::id.eq(id_i32))
            .inner_join(
                crate::schema::files::dsl::files
                    .on(crate::schema::files::dsl::id.eq(crate::schema::maps::dsl::tgafile_id)),
            )
            .get_result::<(Map, File)>(&db)
            .unwrap();
        let cursor = std::io::Cursor::new(file.filebinary);
        let reader =
            crate::image::io::Reader::with_format(cursor.clone(), crate::image::ImageFormat::Tga)
                .decode()
                .unwrap_or_else(|_| {
                    crate::image::io::Reader::with_format(
                        cursor.clone(),
                        crate::image::ImageFormat::Sgi,
                    )
                    .decode()
                    .unwrap()
                });
        let mut resized = reader.resize(width, width, crate::image::imageops::FilterType::Lanczos3);
        let (imgwidth, imgheight) = resized.dimensions();
        let cropped = if height > 0 {
            resized.crop(0, imgheight / 2 - height / 2, imgwidth, height)
        } else {
            resized
        };
        let maps_dir = std::path::PathBuf::from("./images/maps");
        let mut file =
            std::fs::File::create(maps_dir.join(format!("{}_{}-{}.jpg", map.id, width, height)))
                .unwrap();
        let mut jpg_bytes: Vec<u8> = Vec::new();
        cropped
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
}
