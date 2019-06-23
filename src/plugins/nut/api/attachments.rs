use std::io::Read;
use std::ops::Deref;
use std::sync::Arc;

use diesel::Connection as DieselConnection;
use failure::Error as FailueError;
use multipart::server::{
    save::{Entries, SaveResult},
    Multipart,
};
use rocket::{
    http::{ContentType, Status},
    Data, State,
};
use rocket_contrib::json::Json;

use super::super::super::super::{
    errors::{Error, JsonResult, Result},
    orm::{Connection, Database, ID},
    storage::Storage,
};
use super::super::models::{
    attachment::{Dao as AttachmentDao, Item as Attachment},
    policy::{Dao as PolicyDao, Role},
    user::Item as User,
};
use super::users::Administrator;

#[post("/attachments", data = "<file>")]
pub fn create(
    user: User,
    ct: &ContentType,
    file: Data,
    st: State<Arc<Box<dyn Storage>>>,
    db: Database,
) -> JsonResult<()> {
    let db = db.deref();
    let st = st.deref();
    if ct.is_form_data() {
        if let Some((_, boundary)) = ct.params().find(|&(k, _)| k == "boundary") {
            return match Multipart::with_body(file.open(), boundary).save().temp() {
                SaveResult::Full(it) => {
                    process_entries(it, user.id, db, st)?;
                    Ok(Json(()))
                }
                SaveResult::Partial(partial, reason) => {
                    info!("{:?}", reason);
                    process_entries(partial.entries, user.id, db, st)?;
                    Ok(Json(()))
                }
                SaveResult::Error(e) => Err(Error::Io(e).into()),
            };
        }
    }
    Err(Error::Http(Status::BadRequest).into())
}

fn process_entries(it: Entries, user: ID, db: &Connection, st: &Box<dyn Storage>) -> Result<()> {
    let dir = it.save_dir.as_path();
    info!("save dir {}", dir.display());
    for (_, v) in it.fields {
        for f in v {
            debug!("{:?}", f.headers);
            if let Some(n) = f.headers.filename {
                if let Some(t) = f.headers.content_type {
                    let mut fd = f.data.readable()?;
                    let mut buf = Vec::new();
                    fd.read_to_end(&mut buf)?;
                    let url = st.save(&n, &buf)?;
                    AttachmentDao::create(
                        db,
                        user,
                        &n,
                        &t.to_string(),
                        &url,
                        f.data.size() as i64, // FIXME
                    )?;
                }
            }
        }
    }
    Ok(())
}

#[get("/attachments/<id>")]
pub fn show(id: ID, db: Database) -> JsonResult<Attachment> {
    let db = db.deref();
    let it = AttachmentDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/attachments", rank = 1)]
pub fn index_by_administrator(_user: Administrator, db: Database) -> JsonResult<Vec<Attachment>> {
    let db = db.deref();
    let items = AttachmentDao::all(db)?;
    Ok(Json(items))
}

#[get("/attachments", rank = 2)]
pub fn index_by_owner(user: User, db: Database) -> JsonResult<Vec<Attachment>> {
    let db = db.deref();
    let items = AttachmentDao::by_user(db, user.id)?;
    Ok(Json(items))
}

#[delete("/attachments/<id>")]
pub fn destroy(user: User, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, user.id, id)?;
    db.transaction::<_, FailueError, _>(|| AttachmentDao::delete(db, id))?;
    Ok(Json(()))
}

fn can_edit(db: &Connection, user: ID, id: ID) -> Result<()> {
    let it = AttachmentDao::by_id(db, id)?;
    if it.user_id == user || PolicyDao::is(db, user, &Role::Admin) {
        return Ok(());
    }
    Err(Error::Http(Status::Forbidden).into())
}
