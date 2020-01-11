use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::Duration;
use diesel::Connection;
use failure::Error;
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto,
    errors::Result,
    i18n::I18n,
    jwt::Jwt,
    orm::Pool,
    request::{ClientIp, Locale},
};
use super::super::{
    models::log::Dao as LogDao,
    models::user::{Dao as UserDao, Item as User},
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(length(min = 1))]
    pub login: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[post("/users/sign-in")]
pub async fn sign_in(
    form: web::Json<SignIn>,
    jwt: web::Data<Arc<Jwt>>,
    db: web::Data<Pool>,
    remote: ClientIp,
    lang: Locale,
) -> Result<impl Responder> {
    form.validate()?;

    let db = db.get()?;
    let db = db.deref();
    let user: Result<User> = match UserDao::by_email(db, &form.login) {
        Ok(v) => Ok(v),
        Err(_) => match UserDao::by_nick_name(db, &form.login) {
            Ok(v) => Ok(v),
            Err(_) => __i18n_e!(
                db,
                &lang.0,
                "nut.errors.user.is-not-exist",
                &json!({"login": form.login})
            ),
        },
    };
    let user = user?;

    if let Err(e) = user.auth::<Crypto>(&form.password) {
        __i18n_l!(
            db,
            user.id,
            &remote.0,
            &lang.0,
            "nut.logs.user.sign-in.failed"
        )?;
        return Err(e.into());
    }
    user.available()?;

    let uid = user.uid.clone();
    db.transaction::<_, Error, _>(move || {
        UserDao::sign_in(db, user.id, &remote.0)?;
        __i18n_l!(
            db,
            user.id,
            &remote.0,
            &lang.0,
            "nut.logs.user.sign-in.success"
        )?;
        Ok(())
    })?;
    let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
    let token = jwt.sum(
        None,
        &Token {
            uid: uid,
            act: Action::SignIn,
            nbf: nbf,
            exp: exp,
        },
    )?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
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

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    #[validate(length(min = 1))]
    pub current_password: String,
    #[validate(length(min = 6, max = 32))]
    pub new_password: String,
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
