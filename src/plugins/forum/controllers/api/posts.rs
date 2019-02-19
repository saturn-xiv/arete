use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error as FailueError;
use rocket::http::Status;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{
    errors::{Error, JsonResult, Result},
    orm::{Connection, Database},
};
use super::super::super::super::nut::request::CurrentUser;
use super::super::super::models::post::{Dao as PostDao, Item as Post};

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub topic: i64,
    pub post: Option<i64>,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

#[get("/posts")]
pub fn index(user: CurrentUser, db: Database) -> JsonResult<Vec<Post>> {
    let db = db.deref();
    let items = if user.is_admin() {
        PostDao::latest(db)?
    } else {
        PostDao::by_user(db, &user.id)?
    };
    Ok(Json(items))
}

#[get("/posts/<id>")]
pub fn show(id: i64, db: Database) -> JsonResult<Post> {
    let db = db.deref();
    let it = PostDao::get(db, &id)?;
    Ok(Json(it))
}

#[post("/posts", format = "json", data = "<form>")]
pub fn create(user: CurrentUser, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    db.transaction::<_, FailueError, _>(|| {
        PostDao::add(
            db,
            &user.id,
            &form.topic,
            &form.post,
            &form.body,
            &form.media_type.parse()?,
        )
    })?;
    Ok(Json(()))
}

#[post("/posts/<id>", format = "json", data = "<form>")]
pub fn update(user: CurrentUser, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    can_edit(db, &user, id)?;
    db.transaction::<_, FailueError, _>(|| {
        PostDao::update(db, &id, &form.body, &form.media_type.parse()?)
    })?;
    Ok(Json(()))
}

#[delete("/posts/<id>")]
pub fn destory(user: CurrentUser, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, &user, id)?;
    db.transaction::<_, FailueError, _>(|| PostDao::delete(db, &id))?;
    Ok(Json(()))
}

fn can_edit(db: &Connection, user: &CurrentUser, id: i64) -> Result<()> {
    let it = PostDao::get(db, &id)?;
    if user.is_admin() || it.user_id == user.id {
        return Ok(());
    }
    Err(Error::Http(Status::Forbidden).into())
}
