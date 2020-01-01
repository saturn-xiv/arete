use actix_web::{get, web, Responder};

use super::super::super::super::orm::ID;

#[get("/users")]
async fn index() -> impl Responder {
    format!("users index")
}

#[get("/users/{id}")]
async fn show(params: web::Path<ID>) -> impl Responder {
    format!("lang {}", params)
}
