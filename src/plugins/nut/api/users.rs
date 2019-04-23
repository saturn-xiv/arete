use std::fmt;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;

use chrono::{Duration, NaiveDate};
use diesel::prelude::*;
use failure::Error as FailueError;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};
use rocket_contrib::json::Json;
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto,
    errors::{JsonResult, Result},
    i18n::I18n,
    jwt::Jwt,
    orm::{Connection as Db, Database, ID},
    queue::{rabbitmq::RabbitMQ, Queue},
    request::{Locale, Token as Auth},
};
use super::super::{
    models::{
        log::{Dao as LogDao, Item as Log},
        policy::{Dao as PolicyDao, Item as Policy, Role},
        user::{Dao as UserDao, Item as User},
    },
    tasks::send_email,
};

#[derive(Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apply {
    #[validate(length(min = "1"))]
    pub role: String,
    pub resource: Option<String>,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
}

#[post("/users/authorities/<id>", data = "<form>")]
pub fn apply_authority(
    _user: Administrator,
    id: ID,
    db: Database,
    remote: SocketAddr,
    lang: Locale,
    form: Json<Apply>,
) -> JsonResult<()> {
    form.validate()?;
    let form = form.deref();
    let ip = remote.ip().to_string();
    let db = db.deref();

    let user = UserDao::by_id(db, id)?;
    db.transaction::<_, FailueError, _>(move || {
        PolicyDao::apply(
            db,
            user.id,
            &form.role.parse()?,
            &form.resource,
            &form.nbf,
            &form.exp,
        )?;
        __i18n_l!(
            db,
            user.id,
            &ip,
            &lang.0,
            "nut.logs.user.authority.apply",
            form
        )?;
        Ok(())
    })?;

    Ok(Json(()))
}

#[get("/users/authorities/<id>")]
pub fn index_authority(_user: Administrator, id: ID, db: Database) -> JsonResult<Vec<Policy>> {
    let db = db.deref();
    let user = UserDao::by_id(db, id)?;
    let items = PolicyDao::all(db, user.id)?;
    Ok(Json(items))
}

#[derive(Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deny {
    #[validate(length(min = "1"))]
    pub role: String,
    pub resource: Option<String>,
}

#[patch("/users/authorities/<id>", data = "<form>")]
pub fn deny_authority(
    _user: Administrator,
    id: ID,
    db: Database,
    lang: Locale,
    remote: SocketAddr,
    form: Json<Deny>,
) -> JsonResult<()> {
    form.validate()?;
    let form = form.deref();
    let db = db.deref();
    let ip = remote.ip().to_string();
    let user = UserDao::by_id(db, id)?;
    db.transaction::<_, FailueError, _>(move || {
        PolicyDao::deny(db, user.id, &form.role.parse()?, &form.resource)?;
        __i18n_l!(
            db,
            user.id,
            &ip,
            &lang.0,
            "nut.logs.user.authority.deny",
            form
        )?;
        Ok(())
    })?;
    Ok(Json(()))
}

#[get("/users/<id>")]
pub fn show(_user: Administrator, id: ID, db: Database) -> JsonResult<User> {
    let db = db.deref();
    let it = UserDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/users")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<User>> {
    let db = db.deref();
    let items = UserDao::all(db)?;
    Ok(Json(items))
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(length(min = "1"))]
    pub login: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

#[post("/users/sign-in", data = "<form>")]
pub fn sign_in(
    db: Database,
    lang: Locale,
    jwt: State<Arc<Jwt>>,
    remote: SocketAddr,
    form: Json<SignIn>,
) -> JsonResult<String> {
    form.validate()?;
    let ip = remote.ip().to_string();
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
        __i18n_l!(db, user.id, &ip, &lang.0, "nut.logs.user.sign-in.failed")?;
        return Err(e.into());
    }
    user.available()?;

    let uid = user.uid.clone();
    db.transaction::<_, FailueError, _>(move || {
        UserDao::sign_in(db, user.id, &ip)?;
        __i18n_l!(db, user.id, &ip, &lang.0, "nut.logs.user.sign-in.success")?;
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
    Ok(Json(token))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignUp {
    #[validate(length(min = "1", max = "32"))]
    pub real_name: String,
    #[validate(length(min = "1", max = "32"))]
    pub nick_name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
    #[validate(length(min = "1"))]
    pub home: String,
}

#[post("/users/sign-up", data = "<form>")]
pub fn sign_up(
    db: Database,
    lang: Locale,
    jwt: State<Arc<Jwt>>,
    queue: State<Arc<RabbitMQ>>,
    remote: SocketAddr,
    form: Json<SignUp>,
) -> JsonResult<()> {
    form.validate()?;
    let ip = remote.ip().to_string();
    let db = db.deref();
    let jwt = jwt.deref();
    let queue = queue.deref();

    if let Ok(_) = UserDao::by_email(db, &form.email) {
        return __i18n_e!(
            db,
            &lang.0,
            "nut.errors.already-exist.email",
            &json!({"email": form.email})
        );
    }
    if let Ok(_) = UserDao::by_nick_name(db, &form.nick_name) {
        return __i18n_e!(
            db,
            &lang.0,
            "nut.errors.already-exist.nick-name",
            &json!({"name": &form.nick_name})
        );
    }

    let lng = lang.0.clone();
    let home = form.home.clone();
    let user = db.transaction::<_, FailueError, _>(move || {
        UserDao::sign_up::<Crypto>(
            db,
            &form.real_name,
            &form.nick_name,
            &form.email,
            &form.password,
        )?;
        let it = UserDao::by_email(db, &form.email)?;
        __i18n_l!(db, it.id, &ip, &lang.0, "nut.logs.user.sign-up")?;
        Ok(it)
    })?;

    send_email(db, &lng, jwt, queue, &user, &Action::Confirm, &home)?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "1"))]
    pub home: String,
}

#[post("/users/confirm", data = "<form>")]
pub fn confirm(
    db: Database,
    lang: Locale,
    jwt: State<Arc<Jwt>>,
    queue: State<Arc<RabbitMQ>>,
    form: Json<Email>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    if let Some(_) = it.confirmed_at {
        return __i18n_e!(db, &lang.0, "nut.errors.user.already-confirm");
    }
    send_email(db, &lang.0, &jwt, &queue, &it, &Action::Confirm, &form.home)?;
    Ok(Json(()))
}

#[patch("/users/confirm/<token>")]
pub fn confirm_token(
    db: Database,
    remote: SocketAddr,
    lang: Locale,
    token: String,
    jwt: State<Arc<Jwt>>,
) -> JsonResult<()> {
    let db = db.deref();
    let jwt = jwt.deref();
    let ip = remote.ip().to_string();

    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Confirm {
        return __i18n_e!(db, &lang.0, "flashes.bad-action");
    }

    let it = UserDao::by_uid(db, &token.uid)?;
    if let Some(_) = it.confirmed_at {
        return __i18n_e!(db, &lang.0, "nut.errors.user.already-confirm");
    }

    db.transaction::<_, FailueError, _>(move || {
        UserDao::confirm(db, it.id)?;
        __i18n_l!(db, it.id, &ip, &lang.0, "nut.logs.user.confirm")?;
        Ok(())
    })?;
    Ok(Json(()))
}

#[post("/users/unlock", data = "<form>")]
pub fn unlock(
    db: Database,
    lang: Locale,
    jwt: State<Arc<Jwt>>,
    queue: State<Arc<RabbitMQ>>,
    form: Json<Email>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    if None == it.locked_at {
        return __i18n_e!(db, &lang.0, "nut.errors.user.is-not-lock");
    }
    send_email(&db, &lang.0, jwt, queue, &it, &Action::Unlock, &form.home)?;
    Ok(Json(()))
}

#[patch("/users/unlock/<token>")]
pub fn unlock_token(
    db: Database,
    remote: SocketAddr,
    lang: Locale,
    token: String,
    jwt: State<Arc<Jwt>>,
) -> JsonResult<()> {
    let db = db.deref();
    let jwt = jwt.deref();
    let ip = remote.ip().to_string();

    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Unlock {
        return __i18n_e!(db, &lang.0, "flashes.bad-action");
    }

    let it = UserDao::by_uid(db, &token.uid)?;
    if None == it.locked_at {
        return __i18n_e!(db, &lang.0, "nut.errors.user.is-not-lock");
    }
    db.transaction::<_, FailueError, _>(move || {
        UserDao::unlock(db, it.id)?;
        __i18n_l!(db, it.id, &ip, &lang.0, "nut.logs.user.unlock")?;
        Ok(())
    })?;

    Ok(Json(()))
}

#[post("/users/forgot-password", data = "<form>")]
pub fn forgot_password(
    db: Database,
    lang: Locale,
    jwt: State<Arc<Jwt>>,
    queue: State<Arc<RabbitMQ>>,
    form: Json<Email>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    send_email(
        db,
        &lang.0,
        &jwt,
        &queue,
        &it,
        &Action::ResetPassword,
        &form.home,
    )?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassword {
    #[validate(length(min = "1"))]
    pub token: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

#[post("/users/reset-password", data = "<form>")]
pub fn reset_password(
    db: Database,
    lang: Locale,
    jwt: State<Arc<Jwt>>,
    remote: SocketAddr,
    form: Json<ResetPassword>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let jwt = jwt.deref();
    let ip = remote.ip().to_string();

    let token = jwt.parse::<Token>(&form.token)?.claims;
    if token.act != Action::ResetPassword {
        return __i18n_e!(db, &lang.0, "flashes.bad-action");
    }

    let it = UserDao::by_uid(db, &token.uid)?;

    UserDao::password::<Crypto>(db, it.id, &form.password)?;
    __i18n_l!(db, it.id, &ip, &lang.0, "nut.logs.user.reset-password")?;
    Ok(Json(()))
}

#[get("/users/logs?<limit>")]
pub fn logs(db: Database, limit: i64, user: User) -> JsonResult<Vec<Log>> {
    let db = db.deref();
    let items = LogDao::all(db, user.id, limit)?;
    Ok(Json(items))
}

#[get("/users/profile")]
pub fn get_profile(user: User) -> JsonResult<User> {
    Ok(Json(user))
}

#[derive(Deserialize, Validate)]
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

#[post("/users/profile", data = "<form>")]
pub fn set_profile(db: Database, user: User, form: Json<Profile>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    UserDao::set_profile(db, user.id, &form.real_name, &form.logo)?;
    Ok(Json(()))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    #[validate(length(min = "1"))]
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

#[post("/users/change-password", data = "<form>")]
pub fn change_password(
    db: Database,
    lang: Locale,
    remote: SocketAddr,
    user: User,
    form: Json<ChangePassword>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let ip = remote.ip().to_string();

    user.auth::<Crypto>(&form.current_password)?;
    db.transaction::<_, FailueError, _>(move || {
        UserDao::password::<Crypto>(db, user.id, &form.new_password)?;
        __i18n_l!(db, user.id, &ip, &lang.0, "nut.logs.user.change-password")?;
        Ok(())
    })?;

    Ok(Json(()))
}

#[delete("/users/sign-out")]
pub fn sign_out(db: Database, lang: Locale, remote: SocketAddr, user: User) -> JsonResult<()> {
    let db = db.deref();

    let ip = remote.ip().to_string();
    __i18n_l!(db, user.id, &ip, &lang.0, "nut.logs.user.sign-out")?;

    Ok(Json(()))
}

fn send_email(
    db: &Db,
    lang: &String,
    jwt: &Jwt,
    queue: &RabbitMQ,
    user: &User,
    act: &Action,
    home: &String,
) -> Result<()> {
    let expire = 1;
    let (nbf, exp) = Jwt::timestamps(Duration::hours(expire));
    let token = jwt.sum(
        None,
        &Token {
            uid: user.uid.clone(),
            act: act.clone(),
            nbf: nbf,
            exp: exp,
        },
    )?;

    let args =
        Some(json!({ "name": user.real_name, "home": home, "expire":expire, "token":token }));
    let subject = I18n::t(db, lang, format!("nut.mailer.users.{}.subject", act), &args);
    let body = I18n::t(db, lang, format!("nut.mailer.users.{}.body", act), &args);

    queue.publish(
        send_email::NAME.to_string(),
        Uuid::new_v4().to_string(),
        send_email::Task {
            email: user.email.clone(),
            name: user.real_name.clone(),
            subject: subject,
            body: body,
        },
    )?;
    Ok(())
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let Auth(token) = req.guard::<Auth>()?;
        let jwt = req.guard::<State<Arc<Jwt>>>()?;
        if let Ok(token) = jwt.parse::<Token>(&token) {
            let token = token.claims;
            if token.act == Action::SignIn {
                let Database(db) = req.guard::<Database>()?;
                let db = db.deref();
                if let Ok(user) = UserDao::by_uid(db, &token.uid) {
                    if let Ok(_) = user.available() {
                        return Outcome::Success(user);
                    }
                }
            }
        }

        Outcome::Failure((Status::NonAuthoritativeInformation, ()))
    }
}

pub struct Administrator(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for Administrator {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = req.guard::<User>()?;
        let Database(db) = req.guard::<Database>()?;
        let db = db.deref();
        if PolicyDao::is(db, user.id, &Role::Admin) {
            return Outcome::Success(Self(user));
        }

        Outcome::Failure((Status::Forbidden, ()))
    }
}
