pub mod leave_words;
pub mod users;

use actix_web::{get, post, Responder};

#[post("/install")]
async fn install() -> impl Responder {
    format!("install")
}

#[get("/about")]
async fn about() -> impl Responder {
    format!("install")
}
