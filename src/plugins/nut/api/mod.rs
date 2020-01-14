pub mod leave_words;
pub mod users;

use std::ops::Deref;

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};

use super::super::super::{
    env::{AUTHORS, BUILD_TIME, DESCRIPTION, HOMEPAGE, NAME, VERSION},
    errors::Result,
    i18n::{self, I18n},
    orm::Pool as Db,
    request::Locale,
};

#[post("/install")]
async fn install() -> impl Responder {
    HttpResponse::Ok().json(())
}

#[get("/about")]
async fn about(lang: Locale, db: web::Data<Db>) -> Result<impl Responder> {
    let db = db.get()?;
    let db = db.deref();
    let languages = i18n::locale::Dao::languages(db)?;
    let lang = lang.0;

    Ok(HttpResponse::Ok().json(json!({
        "name": NAME,
        "version": VERSION,
        "build": BUILD_TIME,
        "homepage": HOMEPAGE,
        "authors": AUTHORS,
        "title": I18n::t(db, &lang, "site.title", &None::<String>),
        "subhead": I18n::t(db, &lang, "site.subhead", &None::<String>),
        "copyright": I18n::t(db, &lang, "site.copyright", &None::<String>),
        "description": DESCRIPTION,
        "languages": languages,
    })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ok {
    pub created_at: NaiveDateTime,
}

impl Ok {
    pub fn new() -> Self {
        Self {
            created_at: Utc::now().naive_local(),
        }
    }
}
