use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
};
use super::super::models::friend_link::{Dao as FriendLinkDao, Item as FriendLink};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub home: String,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    pub position: i16,
}

#[post("/friend-links", data = "<form>")]
pub fn create(_user: Administrator, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    FriendLinkDao::create(db, &form.title, &form.home, &form.logo, form.position)?;
    Ok(Json(()))
}

#[post("/friend-links/<id>", data = "<form>")]
pub fn update(_user: Administrator, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    FriendLinkDao::update(db, id, &form.title, &form.home, &form.logo, form.position)?;
    Ok(Json(()))
}

#[get("/friend-links/<id>")]
pub fn show(id: ID, _user: Administrator, db: Database) -> JsonResult<FriendLink> {
    let db = db.deref();
    let it = FriendLinkDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/friend-links")]
pub fn index(db: Database) -> JsonResult<Vec<FriendLink>> {
    let db = db.deref();
    let items = FriendLinkDao::all(db)?;
    Ok(Json(items))
}

#[delete("/friend-links/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    FriendLinkDao::delete(db, id)?;
    Ok(Json(()))
}
