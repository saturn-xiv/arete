use actix_web::{delete, get, post, web, Responder};

use super::super::super::super::orm::ID;

#[get("/leave-words")]
async fn index() -> impl Responder {
    format!("leave-words index")
}

#[post("/leave-words")]
async fn create() -> impl Responder {
    format!("leave-words index")
}

#[delete("/leave-words/{id}")]
async fn destroy(params: web::Path<ID>) -> impl Responder {
    format!("destroy {}", params)
}
