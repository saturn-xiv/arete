use std::fmt;

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(length(min = 1))]
    pub login: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[post("/users/sign-in")]
async fn sign_in() -> impl Responder {
    format!("users sign in")
}

#[post("/users/sign-up")]
async fn sign_up() -> impl Responder {
    format!("lang ")
}

#[post("/users/confirm")]
async fn confirm() -> impl Responder {
    format!("confirm ")
}

#[post("/users/unlock")]
async fn unlock() -> impl Responder {
    format!("unlock ")
}

#[post("/users/forgot-password")]
async fn forgot_password() -> impl Responder {
    format!("forgot password ")
}

#[post("/users/confirm/{token}")]
async fn confirm_by_token(params: web::Path<String>) -> impl Responder {
    format!("confirm users {}", params)
}

#[post("/users/unlock/{token}")]
async fn unlock_by_token(params: web::Path<String>) -> impl Responder {
    format!("unlock users {}", params)
}

#[post("/users/reset-password/{token}")]
async fn reset_password(params: web::Path<String>) -> impl Responder {
    format!("reset password {}", params)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    SignIn,
    Confirm,
    Unlock,
    ResetPassword,
}

impl fmt::Display for Action {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::SignIn => fmt.write_str("sign-in"),
            Action::Confirm => fmt.write_str("confirm"),
            Action::Unlock => fmt.write_str("unlock"),
            Action::ResetPassword => fmt.write_str("reset-password"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub uid: String,
    pub act: Action,
    pub nbf: i64,
    pub exp: i64,
}

#[get("/users")]
async fn index() -> impl Responder {
    format!("users index")
}

#[get("/users/self")]
async fn self_() -> impl Responder {
    format!("users self")
}

#[post("/users/profile")]
async fn profile() -> impl Responder {
    format!("users profile")
}

#[post("/users/change-password")]
async fn change_password() -> impl Responder {
    format!("users change password")
}

#[get("/users/logs")]
async fn logs() -> impl Responder {
    HttpResponse::Ok().json(())
}

#[delete("/users/sign-out")]
async fn sign_out() -> impl Responder {
    format!("users sign-out")
}
