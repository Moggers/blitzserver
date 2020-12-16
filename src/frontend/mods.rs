use super::AppData;
use crate::diesel::prelude::*;
use crate::models::{File, Mod, NewFile, NewMod};
use actix_multipart::Multipart;
use actix_web::{get, http::header, post, web, HttpResponse, Result};
use askama::Template;
use diesel::RunQueryDsl;
use futures::{StreamExt, TryStreamExt};
use std::io::Read;

#[derive(Template)]
#[template(path = "mods/list.html")]
pub struct ListModsTemplate {
    mods: Vec<Mod>
}

#[derive(Template)]
#[template(path = "mods/upload.html")]
pub struct UploadModTemplate {}

#[derive(Template)]
#[template(path = "mods/details.html")]
pub struct ModDetailsTemplate<'a> {
    cmod: &'a Mod
}

#[get("/mods")]
pub async fn list(app_data: web::Data<AppData>) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    Ok(HttpResponse::Ok().content_type("text/html").body((ListModsTemplate {
        mods: crate::schema::mods::dsl::mods.load::<Mod>(&db).unwrap()
    }).render().unwrap()))
}

#[get("/mods/upload")]
pub async fn upload_get(_app_data: web::Data<AppData>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body((UploadModTemplate {}).render().unwrap()))
}

#[post("/mods/upload")]
pub async fn upload_post(
    (app_data, mut payload): (web::Data<AppData>, Multipart),
) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut mod_name: Option<(String, String)> = None;
        let content_type = field.content_disposition().unwrap();
        match content_type.get_name() {
            Some("archive") => {
                let mut contents: Vec<u8> = vec![];
                while let Some(bytes) = field.next().await {
                    contents.extend_from_slice(&bytes.unwrap());
                }
                let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&contents)).unwrap();
                for i in 0..archive.len() {
                    let mut archive_file = archive.by_index(i).unwrap();
                    if archive_file.is_file()
                        && std::path::PathBuf::from(archive_file.name()).extension()
                            == Some(std::ffi::OsStr::new("dm"))
                    {
                        let mut contents: String = String::new();
                        archive_file.read_to_string(&mut contents).unwrap();
                        if let Some(captures) = regex::Regex::new(r#"#modname +"?(.+?)"?\r?\n"#)
                            .unwrap()
                            .captures(&contents)
                        {
                            mod_name = Some((
                                captures.get(1).unwrap().as_str().to_owned(),
                                archive_file.name().to_owned(),
                            ));
                        }
                    }
                }
                match mod_name {
                    Some((mod_name, dm_name)) => {
                        let new_file =
                            NewFile::new(content_type.get_filename().unwrap(), &contents);
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
                        let _inserted_mod: Mod = diesel::insert_into(crate::schema::mods::table)
                            .values(NewMod {
                                name: &mod_name,
                                dm_filename: &dm_name,
                                file_id: file.id,
                            })
                            .on_conflict(crate::schema::mods::file_id)
                            .do_update()
                            .set(crate::schema::mods::file_id.eq(crate::schema::mods::file_id))
                            .get_result(&db)
                            .unwrap();
                        return Ok(HttpResponse::Found()
                            .header(header::LOCATION, "/mods")
                            .finish());
                    }
                    _ => {
                        panic!("Havent implemented bad body stuff yet");
                    }
                }
            }
            _ => {}
        }
    }
    panic!("Havent implemented bad body stuff yet");
}

#[get("/mod/{id}")]
pub async fn details((app_data, path): (web::Data<AppData>, web::Path<i32>)) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let cmod = crate::schema::mods::dsl::mods.filter(crate::schema::mods::dsl::id.eq(*path)).get_result::<Mod>(&db).unwrap();
    Ok(HttpResponse::Ok().body((ModDetailsTemplate{cmod: &cmod}).render().unwrap()))
}

#[get("/mod/{id}/download")]
async fn download((app_data, modid): (web::Data<AppData>, web::Path<i32>)) -> Result<HttpResponse> {
    let db = app_data.pool.get().expect("Unable to connect to database");
    let (_cmod,zipfile) = crate::schema::mods::dsl::mods.filter(crate::schema::mods::dsl::id.eq(*modid)).inner_join(crate::schema::files::dsl::files.on(crate::schema::files::dsl::id.eq(crate::schema::mods::dsl::file_id))).get_result::<(Mod, File)>(&db).unwrap();

    Ok(HttpResponse::Ok().content_type("application/zip").body(zipfile.filebinary))
}
