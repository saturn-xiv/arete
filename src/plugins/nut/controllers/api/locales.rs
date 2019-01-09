use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{
    errors::Result,
    i18n::locale::{Dao as LocaleDao, Item as Locale},
    orm::Database,
};
use super::super::super::request::Administrator;

#[get("/<lang>")]
pub fn index(lang: String, db: Database) -> Result<Json<Vec<Locale>>> {
    let db = db.deref();
    let items = LocaleDao::all(db, &lang)?;
    Ok(Json(items))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

#[post("/", format = "json", data = "<form>")]
pub fn save(_user: Administrator, form: Json<Form>, db: Database) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    LocaleDao::set(db, &form.lang, &form.code, &form.message)?;
    Ok(Json(()))
}

#[delete("/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> Result<Json<()>> {
    let db = db.deref();
    LocaleDao::delete(db, &id)?;
    Ok(Json(()))
}
