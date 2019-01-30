use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::super::{
    models::card::{Dao as CardDao, Item as Card},
    request::Administrator,
};

#[derive(Debug, Validate, Deserialize)]
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

#[get("/admin/cards")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Card>> {
    let db = db.deref();
    let it = CardDao::all(db)?;
    Ok(Json(it))
}

#[get("/admin/cards/<id>")]
pub fn show(_user: Administrator, id: i64, db: Database) -> JsonResult<Card> {
    let db = db.deref();
    let it = CardDao::by_id(db, &id)?;
    Ok(Json(it))
}

#[post("/admin/cards", format = "json", data = "<form>")]
pub fn create(_user: Administrator, form: Json<Form>, db: Database) -> JsonResult<()> {
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
        &form.position,
    )?;
    Ok(Json(()))
}

#[post("/admin/cards/<id>", format = "json", data = "<form>")]
pub fn update(_user: Administrator, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CardDao::update(
        db,
        &id,
        &form.lang,
        &form.title,
        &form.logo,
        &form.body,
        &form.media_type.parse()?,
        &form.href,
        &form.action,
        &form.loc,
        &form.position,
    )?;
    Ok(Json(()))
}

#[delete("/admin/cards/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    CardDao::delete(db, &id)?;
    Ok(Json(()))
}
