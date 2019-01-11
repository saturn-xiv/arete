pub mod admin;
pub mod leave_words;
pub mod locales;
pub mod ueditor;
pub mod users;

use std::ops::Deref;

use rocket_contrib::json::JsonValue;

use super::super::super::super::{
    env, errors::Result, i18n::locale::Dao as LocaleDao, orm::Database,
};

#[get("/about")]
pub fn about(db: Database) -> Result<JsonValue> {
    let db = db.deref();
    let languages = LocaleDao::languages(db)?;
    Ok(json!({
        "languages": languages,
        "version": env::version(),
    }))
}
