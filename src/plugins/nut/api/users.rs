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
    orm::Pool as Db,
    request::{ClientIp, Locale, Pager, Pagination},
};
use super::super::{
    models::{
        log::Dao as LogDao,
        user::{Dao as UserDao, Item as User},
    },
    request::{Action, CurrentUser, Token},
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
    db: web::Data<Db>,
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

#[get("/users")]
async fn index() -> impl Responder {
    format!("users index")
}

#[get("/users/self")]
async fn self_(user: CurrentUser) -> impl Responder {
    let mut it = user.0;
    it.password = None;
    it.access_token = None;
    HttpResponse::Ok().json(it)
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
pub async fn change_password() -> impl Responder {
    format!("users change password")
}

#[get("/users/logs")]
pub async fn logs(
    user: CurrentUser,
    pag: web::Query<Pager>,
    db: web::Data<Db>,
) -> Result<impl Responder> {
    let db = db.get()?;
    let db = db.deref();

    let user = user.0.id;
    let total = LogDao::count(db, user)?;
    let items = LogDao::all(db, user, pag.offset(total), pag.limit())?;
    Ok(HttpResponse::Ok().json(Pagination::new(total, pag.size, pag.page, items)))
}

#[delete("/users/sign-out")]
pub async fn sign_out(
    user: CurrentUser,
    lang: Locale,
    remote: ClientIp,
    db: web::Data<Db>,
) -> Result<impl Responder> {
    let db = db.get()?;
    let db = db.deref();
    __i18n_l!(db, user.0.id, &remote.0, &lang.0, "nut.logs.user.sign-out")?;
    Ok(HttpResponse::Ok().json(()))
}
