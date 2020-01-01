pub mod leave_words;
pub mod users;

use actix_web::{get, post, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};

use super::super::super::env::{AUTHORS, BUILD_TIME, DESCRIPTION, HOMEPAGE, NAME, VERSION};

#[post("/install")]
async fn install() -> impl Responder {
    HttpResponse::Ok().json(())
}

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": NAME,
        "version": VERSION,
        "build": BUILD_TIME,
        "homepage": HOMEPAGE,
        "authors": AUTHORS,
        "description": DESCRIPTION
    }))
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
