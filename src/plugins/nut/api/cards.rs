use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
};
use super::super::models::card::{Dao as CardDao, Item as Card};
use super::users::Administrator;

#[derive(Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub action: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub position: i16,
}

#[post("/cards", data = "<form>")]
pub fn create(_user: Administrator, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CardDao::create(
        db,
        &form.lang,
        &form.title,
        &form.logo,
        &form.body,
        &form.media_type.parse()?,
        &form.href,
        &form.action,
        &form.loc,
        form.position,
    )?;
    Ok(Json(()))
}

#[post("/cards/<id>", data = "<form>")]
pub fn update(_user: Administrator, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CardDao::update(
        db,
        id,
        &form.lang,
        &form.title,
        &form.logo,
        &form.body,
        &form.media_type.parse()?,
        &form.href,
        &form.action,
        &form.loc,
        form.position,
    )?;
    Ok(Json(()))
}

#[get("/cards/<id>")]
pub fn show(id: ID, _user: Administrator, db: Database) -> JsonResult<Card> {
    let db = db.deref();
    let it = CardDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/cards")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Card>> {
    let db = db.deref();
    let items = CardDao::all(db)?;
    Ok(Json(items))
}

#[delete("/cards/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    CardDao::delete(db, id)?;
    Ok(Json(()))
}
