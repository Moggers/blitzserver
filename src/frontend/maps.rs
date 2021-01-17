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
    use crate::schema::maps::dsl::*;
    let result_maps = maps.load::<Map>(&db).expect("Error loading games");
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
    };
    let db = app_data.pool.get().expect("Unable to connect to database");
    let mut tga_name: Option<String> = None;
    let mut winter_name: Option<String> = None;

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
                    // == DETERMINE NAME
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
                            errors.push(
                                "Unable to find #dom2title, is this a valid .map file?\n"
                                    .to_string(),
                            );
                        }
                    }
                    // == DETERMINE (and check) TGA FILENAME
                    let new_tga_name: Result<String> = try {
                        let re = regex::bytes::Regex::new(r#"#imagefile ("?[^"\n]+"?)"#)
                            .map_err(|_| ())?;
                        let caps = re.captures(&contents).ok_or(())?;
                        String::from_utf8(caps.get(1).ok_or(())?.as_bytes().to_vec())
                            .map_err(|_| ())?
                    };
                    match new_tga_name {
                        Ok(ntn) => match tga_name {
                            None => tga_name = Some(ntn),
                            Some(ref mut tn) => {
                                if *tn != ntn {
                                    errors.push(format!(
                                        "TGA file is {}, but map specifies {}",
                                        tn, ntn
                                    ));
                                }
                            }
                        },
                        Err(_) => {
                            errors.push("Failed to parse image name from map file".to_string())
                        }
                    }
                    // == DETERMINE (and check) WINTER TGA FILENAME
                    let new_winter_name: Result<String> = try {
                        let re = regex::bytes::Regex::new(r#"#winterimagefile ("?[^"\n]+"?)"#)
                            .map_err(|_| ())?;
                        let caps = re.captures(&contents).ok_or(())?;
                        String::from_utf8(caps.get(1).ok_or(())?.as_bytes().to_vec())
                            .map_err(|_| ())?
                    };
                    match new_winter_name {
                        Ok(nwn) => match winter_name {
                            None => winter_name = Some(nwn),
                            Some(ref mut wn) => {
                                if *wn != nwn {
                                    errors.push(format!(
                                        "Winter TGA file is {}, but map specifies {}",
                                        wn, nwn
                                    ));
                                }
                            }
                        },
                        Err(_) => {} // Lacking a winter image is not fatal
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
                    match tga_name {
                        None => {
                            tga_name = Some(filename.to_string());
                        }
                        Some(ref mut wn) => {
                            if wn.trim() != filename.trim() {
                                errors.push(format!(
                                    "Map #imagefile is {}, but winter TGA filename is {}",
                                    wn, filename
                                ));
                                continue;
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
                        match winter_name {
                            None => winter_name = Some(filename.to_string()),
                            Some(ref mut wn) => {
                                if wn.trim() != filename.trim() {
                                    errors.push(format!(
                                        "Map #winterimagefile is {}, but TGA filename is {}",
                                        wn, filename
                                    ));
                                    continue;
                                }
                            }
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
    if new_map.mapfile_id == 0 {
        errors.push("Missing map file\n".to_string());
    }
    if new_map.tgafile_id == 0 {
        errors.push("Missing tga file\n".to_string());
    }
    if errors.len() > 0 {
        let db = app_data.pool.get().expect("Unable to connect to database");
        use crate::schema::maps::dsl::*;
        let result_maps = maps.load::<Map>(&db).expect("Error loading games");
        Ok(HttpResponse::Ok().content_type("text/html").body(
            (ListMapsTemplate {
                errors: &errors,
                maps: &result_maps,
            })
            .render()
            .unwrap(),
        ))
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
        Ok(HttpResponse::Found()
            .header(header::LOCATION, "/maps")
            .finish())
    }
}
