pub mod leave_words;
pub mod users;

use std::default::Default;
use std::ops::Deref;

use actix_web::{get, post, web, HttpResponse, Responder};
use bytesize::ByteSize;
use chrono::{NaiveDateTime, Utc};
use humantime::format_duration;
use nix::sys::{sysinfo::sysinfo, utsname::uname};

use super::super::super::{
    env::{AUTHORS, BUILD_TIME, DESCRIPTION, HOMEPAGE, NAME, VERSION},
    errors::Result,
    i18n::{self, I18n},
    orm::Pool as Db,
    request::Locale,
    rfc::RFC3399,
    STARTUP,
};

#[post("/install")]
async fn install() -> impl Responder {
    HttpResponse::Ok().json(())
}

#[get("/status")]
async fn status() -> Result<impl Responder> {
    let si = sysinfo()?;
    let un = uname();
    let load = si.load_average();
    Ok(HttpResponse::Ok().json(json!({
            "uptime": format_duration(si.uptime()).to_string(),
            "uname": format!("{} {} {} {} {}", un.sysname(), un.nodename(), un.machine(), un.release(), un.version()),
            "process": si.process_count(),
            "load": (format!("1 Minute: {:.2}", load.0), format!("5 Minutes: {:.2}", load.1), format!("15 Minutes: {:.2}", load.2)),
            "swap": format!("{}/{}", ByteSize(si.swap_total()-si.swap_free()), ByteSize(si.swap_total())),
            "ram": format!("{}/{}", ByteSize(si.ram_total()-si.ram_unused()), ByteSize(si.ram_total())),
    })))
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
        "startup": STARTUP.to_rfc3399(),
        "now": Utc::now().naive_local(),
    })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ok {
    pub created_at: NaiveDateTime,
}

impl Default for Ok {
    fn default() -> Self {
        Self {
            created_at: Utc::now().naive_local(),
        }
    }
}
