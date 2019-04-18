use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    i18n::locale::{Dao as LocaleDao, Item},
    orm::{Database, ID},
    request::Locale,
};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
pub struct Form {
    pub code: String,
    pub message: String,
}

#[post("/locales", data = "<form>")]
pub fn create(
    _user: Administrator,
    lang: Locale,
    db: Database,
    form: Json<Form>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LocaleDao::create(db, &lang.0, &form.code, &form.message)?;
    Ok(Json(()))
}

#[post("/locales/<id>", data = "<form>")]
pub fn update(_user: Administrator, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LocaleDao::update(db, id, &form.code, &form.message)?;
    Ok(Json(()))
}

#[get("/locales/<id>")]
pub fn show(id: ID, _user: Administrator, db: Database) -> JsonResult<Item> {
    let db = db.deref();
    let it = LocaleDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/locales")]
pub fn index(db: Database, lang: Locale) -> JsonResult<Vec<Item>> {
    let db = db.deref();
    let items = LocaleDao::by_lang(db, &lang.0)?;
    Ok(Json(items))
}

#[delete("/locales/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    LocaleDao::delete(db, id)?;
    Ok(Json(()))
}

#[get("/languages")]
pub fn languages(db: Database) -> JsonResult<Vec<String>> {
    let db = db.deref();
    let items = LocaleDao::languages(db)?;
    Ok(Json(items))
}
