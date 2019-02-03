use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::Path;

use chrono::Utc;
use multipart::server::{
    save::{Entries, SaveResult::*},
    Multipart,
};
use rocket::{http::ContentType, Data, State};
use rocket_contrib::json::Json;
use uuid::Uuid;

use super::super::super::super::super::{
    env::Config,
    errors::{JsonResult, Result},
    i18n::I18n,
    orm::Database,
};
use super::super::super::{
    models::{
        attachment::{Dao as AttachmentDao, Item as Attachment},
        policy::{Dao as PolicyDao, Role},
    },
    request::CurrentUser,
};

#[get("/attachments")]
pub fn index(user: CurrentUser, db: Database) -> JsonResult<Vec<Attachment>> {
    let db = db.deref();
    let items = if PolicyDao::can(db, &user.id, &Role::Admin, &None) {
        AttachmentDao::all(db)?
    } else {
        AttachmentDao::by_user(db, &user.id)?
    };
    Ok(Json(items))
}

#[post("/attachments", data = "<data>")]
pub fn create(
    content_type: &ContentType,
    user: CurrentUser,
    data: Data,
    i18n: I18n,
    cfg: State<Config>,
    db: Database,
) -> JsonResult<Vec<Attachment>> {
    let db = db.deref();
    let cfg = cfg.deref();
    if content_type.is_form_data() {
        if let Some((_, title)) = content_type.params().find(|&(k, _)| k == "boundary") {
            let mut items = Vec::new();
            for (title, mime_type, url, size) in process_upload(title, data, cfg)? {
                items.push(AttachmentDao::create(
                    db,
                    &user.id,
                    &title,
                    &mime_type,
                    &url,
                    &(size as i64),
                )?);
            }
            return Ok(Json(items));
        }
    }
    Err(i18n.e("flashes.bad-action", &None::<String>).into())
}

fn process_upload(boundary: &str, data: Data, cfg: &Config) -> Result<Vec<Temp>> {
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, cfg),
        Partial(partial, _) => process_entries(partial.entries, cfg),
        Error(e) => Err(e.into()),
    }
}

type Temp = (String, String, String, u64);

fn process_entries(entries: Entries, cfg: &Config) -> Result<Vec<Temp>> {
    let now = Utc::now().format("%F").to_string();
    let mut items = Vec::new();
    for (_, files) in entries.fields {
        for src in files {
            let title = src.headers.filename.ok_or(format_err!("empty filename"))?;
            let mime_type = src
                .headers
                .content_type
                .ok_or(format_err!("empty content type"))?
                .to_string();
            let size = src.data.size();

            let mut file = Path::new(&now).join(Uuid::new_v4().to_string());
            if let Some(ext) = Path::new(&title).extension() {
                file.set_extension(ext);
            }

            {
                let file = cfg.http.upload().join(&file);
                if let Some(d) = file.parent() {
                    create_dir_all(d)?;
                }
                let mut buf = Vec::new();
                src.data.readable()?.read_to_end(&mut buf)?;
                let mut dst = File::create(file)?;
                dst.write_all(&buf)?;
            }

            items.push((
                title,
                mime_type,
                Path::new("upload").join(&file).display().to_string(),
                size,
            ));
        }
    }

    Ok(items)
}

#[delete("/attachments/<id>")]
pub fn destory(user: CurrentUser, i18n: I18n, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    let it = AttachmentDao::by_id(db, &id)?;
    if !PolicyDao::can(db, &user.id, &Role::Admin, &None) && it.user_id != user.id {
        return Err(i18n.e("flashes.forbidden", &None::<String>).into());
    }
    AttachmentDao::delete(db, &id)?;
    Ok(Json(()))
}
