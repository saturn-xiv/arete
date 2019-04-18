use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error as FailueError;
use rocket::http::Status;
use rocket_contrib::json::Json;

use super::super::super::super::{
    errors::{Error, JsonResult, Result},
    orm::{Connection, Database, ID},
};
use super::super::models::{
    attachment::{Dao as AttachmentDao, Item as Attachment},
    policy::{Dao as PolicyDao, Role},
    user::Item as User,
};
use super::users::Administrator;

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
