use std::fmt;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel::{prelude::*, update};
use failure::Error;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    errors::Result,
    i18n::I18n,
    jwt::Jwt,
    orm::{schema::users, Database},
    queue::{rabbitmq::RabbitMQ, Queue},
};
use super::super::super::{
    models::{
        log::{Dao as LogDao, Item as Log},
        policy::Dao as PolicyDao,
        user::{Dao as UserDao, Item as User},
    },
    request::CurrentUser,
    tasks::send_email,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub uid: String,
    pub act: Action,
    pub nbf: i64,
    pub exp: i64,
    pub roles: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(length(min = "1"))]
    pub login: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

#[post("/users/sign-in", format = "json", data = "<form>")]
pub fn sign_in(
    db: Database,
    jwt: State<Arc<Jwt>>,
    i18n: I18n,
    remote: SocketAddr,
    form: Json<SignIn>,
) -> Result<JsonValue> {
    form.validate()?;
    let ip = remote.ip();
    let db = db.deref();

    let user: Result<User> = match UserDao::by_email(db, &form.login) {
        Ok(v) => Ok(v),
        Err(_) => match UserDao::by_nick_name(db, &form.login) {
            Ok(v) => Ok(v),
            Err(_) => Err(i18n.e(
                "nut.errors.user.is-not-exist",
                &Some(json!({"login": form.login})),
            )),
        },
    };
    let user = user?;

    if let Err(e) = user.auth::<Sodium>(&form.password) {
        i18n.l(&user.id, "nut.logs.user.sign-in.failed", &None::<String>)?;
        return Err(e);
    }
    user.available()?;
    UserDao::sign_in(db, &user.id, &ip)?;
    i18n.l(&user.id, "nut.logs.user.sign-in.success", &None::<String>)?;
    let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
    let token = jwt.sum(
        None,
        &Token {
            uid: user.uid,
            act: Action::SignIn,
            nbf: nbf,
            exp: exp,
            roles: PolicyDao::all(db, &user.id)?
                .iter()
                .filter(|(_, it)| *it == None)
                .map(|(it, _)| it.to_string())
                .collect(),
        },
    )?;
    Ok(json!({ "token": token }))
}

#[derive(Debug, Validate, Deserialize)]
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

#[post("/users/sign-up", format = "json", data = "<form>")]
pub fn sign_up(
    form: Json<SignUp>,
    queue: State<Arc<RabbitMQ>>,
    db: Database,
    jwt: State<Arc<Jwt>>,
    i18n: I18n,
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    if let Ok(_) = UserDao::by_email(db, &form.email) {
        return Err(i18n.e(
            "nut.errors.already-exist.email",
            &Some(json!({"email":form.email})),
        ));
    }
    if let Ok(_) = UserDao::by_nick_name(db, &form.nick_name) {
        return Err(i18n.e(
            "nut.errors.already-exist.nick-name",
            &Some(json!({"name":&form.nick_name})),
        ));
    }

    let home = form.home.clone();
    let user = db.transaction::<_, Error, _>(move || {
        UserDao::sign_up::<Sodium>(
            db,
            &form.real_name,
            &form.nick_name,
            &form.email,
            &form.password,
        )?;
        let it = UserDao::by_email(db, &form.email)?;
        Ok(it)
    })?;
    i18n.l(&user.id, "nut.logs.user.sign-up", &None::<String>)?;
    send_email(&i18n, jwt, queue, &user, &Action::Confirm, &home)?;
    Ok(json!({}))
}

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
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    if let Some(_) = it.confirmed_at {
        return Err(i18n.e("nut.errors.user.already-confirm", &None::<String>));
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
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let it = UserDao::by_email(db, &form.email)?;
    if None == it.locked_at {
        return Err(i18n.e("nut.errors.user.is-not-lock", &None::<String>));
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
) -> Result<JsonValue> {
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
) -> Result<JsonValue> {
    form.validate()?;
    let token = jwt.parse::<Token>(&form.token)?.claims;
    if token.act != Action::ResetPassword {
        return Err(i18n.e("flashes.bad-action", &None::<String>));
    }

    let db = db.deref();
    let it = UserDao::by_uid(db, &token.uid)?;

    UserDao::password::<Sodium>(db, &it.id, &form.password)?;
    i18n.l(&it.id, "nut.logs.user.reset-password", &None::<String>)?;
    Ok(json!({}))
}

#[get("/users/logs")]
pub fn logs(user: CurrentUser, db: Database) -> Result<Json<Vec<Log>>> {
    let db = db.deref();
    let items = LogDao::all(db, &user.id, 1 << 10)?;
    Ok(Json(items))
}

#[get("/users/profile")]
pub fn get_profile(user: CurrentUser, db: Database) -> Result<Json<Profile>> {
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
pub fn post_profile(user: CurrentUser, form: Json<Profile>, db: Database) -> Result<Json<()>> {
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
) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    let user = UserDao::by_id(db, &user.id)?;
    user.auth::<Sodium>(&form.current_password)?;
    UserDao::password::<Sodium>(db, &user.id, &form.new_password)?;
    i18n.l(&user.id, "nut.logs.user.change-password", &None::<String>)?;
    Ok(Json(()))
}

#[delete("/users/sign-out")]
pub fn sign_out(user: CurrentUser, i18n: I18n) -> Result<Json<()>> {
    i18n.l(&user.id, "nut.logs.user.sign-out", &None::<String>)?;
    Ok(Json(()))
}

fn send_email(
    i18n: &I18n,
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
            roles: Vec::new(),
        },
    )?;

    let args =
        Some(json!({ "name": user.real_name, "home": home, "expire":expire, "token":token }));
    let subject = i18n.t(format!("nut.mailer.users.{}.subject", act), &args);
    let body = i18n.t(format!("nut.mailer.users.{}.body", act), &args);

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
