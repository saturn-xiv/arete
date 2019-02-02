use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::super::{
    models::friend_link::{Dao as FriendLinkDao, Item as FriendLink},
    request::Administrator,
};

#[derive(Debug, Validate, Deserialize)]
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

#[get("/admin/friend-links")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<FriendLink>> {
    let db = db.deref();
    let it = FriendLinkDao::all(db)?;
    Ok(Json(it))
}

#[get("/admin/friend-links/<id>")]
pub fn show(_user: Administrator, id: i64, db: Database) -> JsonResult<FriendLink> {
    let db = db.deref();
    let it = FriendLinkDao::by_id(db, &id)?;
    Ok(Json(it))
}

#[post("/admin/friend-links", format = "json", data = "<form>")]
pub fn create(_user: Administrator, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    FriendLinkDao::create(db, &form.title, &form.home, &form.logo, &form.position)?;
    Ok(Json(()))
}

#[post("/admin/friend-links/<id>", format = "json", data = "<form>")]
pub fn update(_user: Administrator, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    FriendLinkDao::update(db, &id, &form.title, &form.home, &form.logo, &form.position)?;
    Ok(Json(()))
}

#[delete("/admin/friend-links/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    FriendLinkDao::delete(db, &id)?;
    Ok(Json(()))
}
