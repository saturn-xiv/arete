use std::ops::Deref;

use rocket_contrib::json::Json;

use super::super::super::super::super::{
    errors::Result,
    i18n::locale::{Dao as LocaleDao, Item as Locale},
    orm::Database,
};

#[get("/locales/<lang>")]
pub fn index(lang: String, db: Database) -> Result<Json<Vec<Locale>>> {
    let db = db.deref();
    let items = LocaleDao::by_lang(db, &lang)?;
    Ok(Json(items))
}
