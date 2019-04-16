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
use super::super::models::post::{Dao as PostDao, Item as Post};

#[derive(Validate, Deserialize)]
pub struct Form {
    pub topic: ID,
    pub post: Option<ID>,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

#[post("/posts", data = "<form>")]
pub fn create(user: User, db: Database, form: Json<Form>) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, FailueError, _>(|| {
        PostDao::add(
            db,
            user.id,
            form.topic,
            form.post,
            &form.body,
            &form.media_type.parse()?,
        )
    })?;
    Ok(Json(()))
}

#[post("/posts/<id>", data = "<form>")]
pub fn update(user: User, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, user.id, id)?;
    db.transaction::<_, FailueError, _>(|| {
        PostDao::update(db, id, &form.body, &form.media_type.parse()?)
    })?;

    Ok(Json(()))
}

#[get("/posts/<id>")]
pub fn show(id: ID, db: Database) -> JsonResult<Post> {
    let db = db.deref();
    let it = PostDao::get(db, id)?;
    Ok(Json(it))
}

#[get("/posts")]
pub fn index(user: User, db: Database) -> JsonResult<Vec<Post>> {
    let db = db.deref();
    let items = if PolicyDao::is(db, user.id, &Role::Admin) {
        PostDao::latest(db)?
    } else {
        PostDao::by_user(db, user.id)?
    };
    Ok(Json(items))
}

#[delete("/posts/<id>")]
pub fn destroy(user: User, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    can_edit(db, user.id, id)?;
    db.transaction::<_, FailueError, _>(|| PostDao::delete(db, id))?;
    Ok(Json(()))
}

fn can_edit(db: &Connection, user: ID, id: ID) -> Result<()> {
    let it = PostDao::get(db, id)?;
    if it.user_id == user || PolicyDao::is(db, user, &Role::Admin) {
        return Ok(());
    }
    Err(Error::Http(Status::Forbidden).into())
}
