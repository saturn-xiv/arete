use std::fmt;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel::{prelude::*, update};
use failure::Error;
use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;
use validator::Validate;




#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "1"))]
    pub home: String,
}

#[post("/users/confirm", format = "json", data = "<form>")]
pub fn confirm(
    form: Json<Email>,
    queue: State<Arc<RabbitMQ>>,
    db: Database,
    jwt: State<Arc<Jwt>>,
    i18n: I18n,
) -> JsonValueResult {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    if let Some(_) = it.confirmed_at {
        return Err(i18n
            .e("nut.errors.user.already-confirm", &None::<String>)
            .into());
    }
    send_email(&i18n, jwt, queue, &it, &Action::Confirm, &form.home)?;
    Ok(json!({}))
}

#[post("/users/unlock", format = "json", data = "<form>")]
pub fn unlock(
    form: Json<Email>,
    queue: State<Arc<RabbitMQ>>,
    i18n: I18n,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> JsonValueResult {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    if None == it.locked_at {
        return Err(i18n
            .e("nut.errors.user.is-not-lock", &None::<String>)
            .into());
    }
    send_email(&i18n, jwt, queue, &it, &Action::Unlock, &form.home)?;
    Ok(json!({}))
}

#[post("/users/forgot-password", format = "json", data = "<form>")]
pub fn forgot_password(
    form: Json<Email>,
    queue: State<Arc<RabbitMQ>>,
    db: Database,
    i18n: I18n,
    jwt: State<Arc<Jwt>>,
) -> JsonValueResult {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    send_email(&i18n, jwt, queue, &it, &Action::ResetPassword, &form.home)?;
    Ok(json!({}))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassword {
    #[validate(length(min = "1"))]
    pub token: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

#[post("/users/reset-password", format = "json", data = "<form>")]
pub fn reset_password(
    form: Json<ResetPassword>,
    db: Database,
    jwt: State<Arc<Jwt>>,
    i18n: I18n,
) -> JsonValueResult {
    form.validate()?;
    let token = jwt.parse::<Token>(&form.token)?.claims;
    if token.act != Action::ResetPassword {
        return Err(i18n.e("flashes.bad-action", &None::<String>).into());
    }

    let db = db.deref();
    let it = UserDao::by_uid(db, &token.uid)?;

    UserDao::password::<Sodium>(db, &it.id, &form.password)?;
    i18n.l(&it.id, "nut.logs.user.reset-password", &None::<String>)?;
    Ok(json!({}))
}

#[get("/users/logs")]
pub fn logs(user: CurrentUser, db: Database) -> JsonResult<Vec<Log>> {
    let db = db.deref();
    let items = LogDao::all(db, &user.id, 1 << 10)?;
    Ok(Json(items))
}

#[get("/users/profile")]
pub fn get_profile(user: CurrentUser, db: Database) -> JsonResult<Profile> {
    let db = db.deref();
    let it = UserDao::by_id(db, &user.id)?;
    Ok(Json(it.into()))
}

#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "2", max = "32"))]
    pub nick_name: String,
    #[validate(length(min = "2", max = "32"))]
    pub real_name: String,
    #[validate(length(min = "1"))]
    pub logo: String,
}

impl From<User> for Profile {
    fn from(it: User) -> Self {
        Self {
            real_name: it.real_name,
            nick_name: it.nick_name,
            logo: it.logo,
            email: it.email,
        }
    }
}

#[post("/users/profile", format = "json", data = "<form>")]
pub fn post_profile(user: CurrentUser, form: Json<Profile>, db: Database) -> JsonResult<()> {
    let db = db.deref();
    let now = Utc::now().naive_utc();
    let it = users::dsl::users.filter(users::dsl::id.eq(&user.id));
    update(it)
        .set((
            users::dsl::real_name.eq(&form.real_name),
            users::dsl::logo.eq(&form.logo),
            users::dsl::updated_at.eq(&now),
        ))
        .execute(db)?;

    Ok(Json(()))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    #[validate(length(min = "1"))]
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

#[post("/users/change-password", format = "json", data = "<form>")]
pub fn change_password(
    db: Database,
    form: Json<ChangePassword>,
    user: CurrentUser,
    i18n: I18n,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let user = UserDao::by_id(db, &user.id)?;
    user.auth::<Sodium>(&form.current_password)?;
    UserDao::password::<Sodium>(db, &user.id, &form.new_password)?;
    i18n.l(&user.id, "nut.logs.user.change-password", &None::<String>)?;
    Ok(Json(()))
}

#[delete("/users/sign-out")]
pub fn sign_out(user: CurrentUser, i18n: I18n) -> JsonResult<()> {
    i18n.l(&user.id, "nut.logs.user.sign-out", &None::<String>)?;
    Ok(Json(()))
}

