use std::ops::Deref;

use chrono::{Duration, NaiveDate, Utc};
use diesel::prelude::*;
use failure::Error as FailueError;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{
    crypto::Crypto,
    errors::{Error, JsonResult},
    orm::{Database, ID},
};
use super::super::super::super::nut::api::users::Administrator;
use super::super::models::{
    log::Dao as LogDao,
    user::{Dao as UserDao, Item as User},
};
use super::Token;

#[get("/users")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<User>> {
    let db = db.deref();
    let items = UserDao::all(db)?;
    Ok(Json(items))
}

#[get("/users/<id>")]
pub fn show(_user: Administrator, id: ID, db: Database) -> JsonResult<User> {
    let db = db.deref();
    let it = UserDao::by_id(db, id)?;
    Ok(Json(it))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
    pub startup: NaiveDate,
    pub shutdown: NaiveDate,
}

#[post("/users", data = "<form>")]
pub fn create(_user: Administrator, db: Database, form: Json<Create>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    UserDao::add::<Crypto>(
        db,
        &form.name,
        &form.email,
        &form.password,
        &form.startup,
        &form.shutdown,
    )?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    #[validate(length(min = "1", max = "32"))]
    pub name: String,
    pub startup: NaiveDate,
    pub shutdown: NaiveDate,
}

#[post("/users/<id>", data = "<form>")]
pub fn update(id: ID, _user: Administrator, db: Database, form: Json<Update>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    UserDao::update(db, id, &form.name, &form.startup, &form.shutdown)?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Bind {
    pub address: Option<String>,
}

#[post("/users/<id>/bind", data = "<form>")]
pub fn bind(_user: Administrator, id: ID, db: Database, form: Json<Bind>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    UserDao::bind(db, id, &form.address)?;
    Ok(Json(()))
}

#[delete("/users/<id>")]
pub fn destroy(id: ID, _user: Administrator, db: Database) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, FailueError, _>(move || {
        UserDao::delete(db, id)?;
        Ok(())
    })?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

#[post("/users/change-password", data = "<form>")]
pub fn change_password(db: Database, form: Json<ChangePassword>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let user = UserDao::by_email(db, &form.email)?;
    user.auth::<Crypto>(&form.current_password)?;
    UserDao::password::<Crypto>(db, user.id, &form.new_password)?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(email, length(min = "1"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

#[post("/users/sign-in", data = "<form>")]
pub fn sign_in(_token: Token, db: Database, form: Json<SignIn>) -> JsonResult<User> {
    form.validate()?;
    let db = db.deref();
    if let Ok(user) = UserDao::by_email(db, &form.email) {
        user.auth::<Crypto>(&form.password)?;
        user.enable()?;
        return Ok(Json(user));
    }

    info!("auto register vpn user {}", form.email);
    let now = Utc::now().naive_utc().date();
    UserDao::add::<Crypto>(
        db,
        &"Guest".to_string(),
        &form.email,
        &form.password,
        &(now + Duration::days(1)),
        &(now - Duration::days(1)),
    )?;
    Err(Error::UserIsNotConfirmed.into())
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Connect {
    pub email: String,
    pub remote_ip: String,
    pub remote_port: i32,
    pub trusted_ip: String,
    pub trusted_port: i32,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Disconnect {
    pub email: String,
    pub trusted_ip: String,
    pub trusted_port: i32,
    pub received: i64,
    pub send: i64,
}

#[post("/users/connect", data = "<form>")]
pub fn connect(_token: Token, db: Database, form: Json<Connect>) -> JsonResult<()> {
    let db = db.deref();
    let user = UserDao::by_email(db, &form.email)?;
    user.enable()?;

    db.transaction::<_, FailueError, _>(move || {
        UserDao::online(db, user.id, true)?;
        LogDao::connect(
            db,
            user.id,
            &form.trusted_ip,
            form.trusted_port,
            &form.remote_ip,
            form.remote_port,
        )?;
        Ok(())
    })?;
    Ok(Json(()))
}

#[post("/users/disconnect", data = "<form>")]
pub fn disconnect(_token: Token, db: Database, form: Json<Disconnect>) -> JsonResult<()> {
    let db = db.deref();
    let user = UserDao::by_email(db, &form.email)?;
    user.enable()?;

    db.transaction::<_, FailueError, _>(move || {
        UserDao::online(db, user.id, false)?;
        LogDao::disconnect(
            db,
            user.id,
            &form.trusted_ip,
            form.trusted_port,
            form.received,
            form.send,
        )?;
        Ok(())
    })?;
    Ok(Json(()))
}
