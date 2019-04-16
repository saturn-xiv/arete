use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use failure::Error as FailueError;
use rocket::http::Status;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, JsonResult, Result},
    orm::{Connection, Database, ID},
};
use super::super::super::nut::models::{
    policy::{Dao as PolicyDao, Role},
    user::Item as User,
};
use super::super::models::topic::{Dao as TopicDao, Item as Topic};

#[derive(Validate, Deserialize)]
pub struct Form {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    pub tags: Vec<ID>,
    pub categories: Vec<ID>,
}

#[post("/topics", data = "<form>")]
pub fn create(user: User, db: Database, form: Json<Form>) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, FailueError, _>(|| {
        TopicDao::add(
            db,
            user.id,
            &form.title,
            &form.body,
            &form.media_type.parse()?,
            &form.tags,
            &form.categories,
        )
    })?;
    Ok(Json(()))
}

#[post("/topics/<id>", data = "<form>")]
pub fn update(user: User, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, user.id, id)?;
    db.transaction::<_, FailueError, _>(|| {
        TopicDao::update(
            db,
            id,
            &form.title,
            &form.body,
            &form.media_type.parse()?,
            &form.tags,
            &form.categories,
        )
    })?;
    Ok(Json(()))
}

#[get("/topics/<id>")]
pub fn show(id: ID, db: Database) -> JsonResult<Topic> {
    let db = db.deref();
    let it = TopicDao::get(db, id)?;
    Ok(Json(it))
}

#[get("/topics")]
pub fn index(user: User, db: Database) -> JsonResult<Vec<Topic>> {
    let db = db.deref();
    let items = if PolicyDao::is(db, user.id, &Role::Admin) {
        TopicDao::latest(db)?
    } else {
        TopicDao::by_user(db, user.id)?
    };

    Ok(Json(items))
}

#[delete("/topics/<id>")]
pub fn destroy(user: User, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, user.id, id)?;
    db.transaction::<_, FailueError, _>(|| TopicDao::delete(db, id))?;
    Ok(Json(()))
}

fn can_edit(db: &Connection, user: ID, topic: ID) -> Result<()> {
    let it = TopicDao::get(db, topic)?;
    if it.user_id == user || PolicyDao::is(db, user, &Role::Admin) {
        return Ok(());
    }
    Err(Error::Http(Status::Forbidden).into())
}
