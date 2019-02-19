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
use super::super::super::models::topic::{Dao as TopicDao, Item as Topic};

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    pub tags: Vec<i64>,
    pub categories: Vec<i64>,
}

#[get("/topics")]
pub fn index(user: CurrentUser, db: Database) -> JsonResult<Vec<Topic>> {
    let db = db.deref();
    let items = if user.is_admin() {
        TopicDao::latest(db)?
    } else {
        TopicDao::by_user(db, &user.id)?
    };
    Ok(Json(items))
}

#[get("/topics/<id>")]
pub fn show(id: i64, db: Database) -> JsonResult<Topic> {
    let db = db.deref();
    let it = TopicDao::get(db, &id)?;
    Ok(Json(it))
}

#[post("/topics", format = "json", data = "<form>")]
pub fn create(user: CurrentUser, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    db.transaction::<_, FailueError, _>(|| {
        TopicDao::add(
            db,
            &user.id,
            &form.title,
            &form.body,
            &form.media_type.parse()?,
            &form.tags,
            &form.categories,
        )
    })?;
    Ok(Json(()))
}

#[post("/topics/<id>", format = "json", data = "<form>")]
pub fn update(user: CurrentUser, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    can_edit(db, &user, id)?;
    db.transaction::<_, FailueError, _>(|| {
        TopicDao::update(
            db,
            &id,
            &form.title,
            &form.body,
            &form.media_type.parse()?,
            &form.tags,
            &form.categories,
        )
    })?;
    Ok(Json(()))
}

#[delete("/topics/<id>")]
pub fn destory(user: CurrentUser, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, &user, id)?;
    db.transaction::<_, FailueError, _>(|| TopicDao::delete(db, &id))?;
    Ok(Json(()))
}

fn can_edit(db: &Connection, user: &CurrentUser, id: i64) -> Result<()> {
    let it = TopicDao::get(db, &id)?;
    if user.is_admin() || it.user_id == user.id {
        return Ok(());
    }
    Err(Error::Http(Status::Forbidden).into())
}
