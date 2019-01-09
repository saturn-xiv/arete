use std::fmt;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;

use chrono::{Duration, Utc};
use diesel::{prelude::*, update};
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
        user::{Dao as UserDao, Item as User},
    },
    request::{CurrentUser, Host, Locale},
    tasks::send_email,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub uid: String,
    pub act: Action,
    pub nbf: i64,
    pub exp: i64,
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
    pub id: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

#[post("/sign-in", format = "json", data = "<form>")]
pub fn sign_in(
    db: Database,
    jwt: State<Arc<Jwt>>,
    remote: SocketAddr,
    form: Json<SignIn>,
) -> Result<JsonValue> {
    form.validate()?;
    let ip = remote.ip();
    let db = db.deref();
    let user: Result<User> = match UserDao::by_email(db, &form.id) {
        Ok(v) => Ok(v),
        Err(_) => match UserDao::by_nick_name(db, &form.id) {
            Ok(v) => Ok(v),
            Err(_) => Err(format!("User {} not exist", form.id).into()),
        },
    };
    let user = user?;

    if let Err(e) = user.auth::<Sodium>(&form.password) {
        LogDao::add(db, &user.id, &ip, "Sign in failed")?;
        return Err(e);
    }
    user.available()?;
    UserDao::sign_in(db, &user.id, &ip)?;
    LogDao::add(db, &user.id, &ip, "Sign in success")?;
    let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
    let token = jwt.sum(
        None,
        &Token {
            uid: user.uid,
            act: Action::SignIn,
            nbf: nbf,
            exp: exp,
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
}

#[post("/sign-up", format = "json", data = "<form>")]
pub fn sign_up(
    form: Json<SignUp>,
    queue: State<Arc<RabbitMQ>>,
    db: Database,
    jwt: State<Arc<Jwt>>,
    remote: SocketAddr,
    host: Host,
    locale: Locale,
    i18n: I18n,
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let ip = remote.ip();
    let Locale(locale) = locale;
    let jwt = jwt.deref();

    if let Ok(_) = UserDao::by_email(db, &form.email) {
        return Err(format!("Email {} already exist", form.email).into());
    }
    if let Ok(_) = UserDao::by_nick_name(db, &form.nick_name) {
        return Err(format!("Nick name {} already exist", form.nick_name).into());
    }
    UserDao::sign_up::<Sodium>(
        db,
        &form.real_name,
        &form.nick_name,
        &form.email,
        &form.password,
    )?;
    let it = UserDao::by_email(db, &form.email)?;
    LogDao::add(db, &it.id, &ip, "Sign up")?;
    send_email(
        &i18n,
        jwt,
        queue,
        &it,
        &Action::Confirm,
        &locale,
        &host.hostname,
    )?;
    Ok(json!({}))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    #[validate(email)]
    pub email: String,
}

#[post("/confirm", format = "json", data = "<form>")]
pub fn confirm(
    form: Json<Email>,
    host: Host,
    locale: Locale,
    queue: State<Arc<RabbitMQ>>,
    db: Database,
    jwt: State<Arc<Jwt>>,
    i18n: I18n,
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();

    let Locale(locale) = locale;
    let it = UserDao::by_email(db, &form.email)?;
    if let Some(_) = it.confirmed_at {
        return Err("User already confirmed".into());
    }
    send_email(
        &i18n,
        jwt,
        queue,
        &it,
        &Action::Confirm,
        &locale,
        &host.hostname,
    )?;
    Ok(json!({}))
}

#[put("/confirm/<token>")]
pub fn confirm_token(
    token: String,
    remote: SocketAddr,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> Result<JsonValue> {
    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Confirm {
        return Err("bad action".into());
    }

    let db = db.deref();
    let ip = remote.ip();
    let it = UserDao::by_uid(db, &token.uid)?;
    if let Some(_) = it.confirmed_at {
        return Err("User already confirmed".into());
    }
    UserDao::confirm(db, &it.id)?;
    LogDao::add(db, &it.id, &ip, "Confirmed")?;
    Ok(json!({}))
}

#[post("/unlock", format = "json", data = "<form>")]
pub fn unlock(
    form: Json<Email>,
    host: Host,
    locale: Locale,
    queue: State<Arc<RabbitMQ>>,
    i18n: I18n,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();
    let Locale(locale) = locale;
    let it = UserDao::by_email(db, &form.email)?;
    if None == it.locked_at {
        return Err("User isn't locked".into());
    }
    send_email(
        &i18n,
        jwt,
        queue,
        &it,
        &Action::Unlock,
        &locale,
        &host.hostname,
    )?;
    Ok(json!({}))
}

#[put("/unlock/<token>")]
pub fn unlock_token(
    token: String,
    remote: SocketAddr,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> Result<JsonValue> {
    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Unlock {
        return Err("bad action".into());
    }

    let db = db.deref();
    let ip = remote.ip();
    let it = UserDao::by_uid(db, &token.uid)?;
    if None == it.locked_at {
        return Err("User already isn't locked".into());
    }
    UserDao::unlock(db, &it.id)?;
    LogDao::add(db, &it.id, &ip, "Unlock")?;
    Ok(json!({}))
}

#[post("/forgot-password", format = "json", data = "<form>")]
pub fn forgot_password(
    form: Json<Email>,
    queue: State<Arc<RabbitMQ>>,
    host: Host,
    locale: Locale,
    db: Database,
    i18n: I18n,
    jwt: State<Arc<Jwt>>,
) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let queue = queue.deref();
    let jwt = jwt.deref();
    let Locale(locale) = locale;
    let it = UserDao::by_uid(db, &form.email)?;
    send_email(
        &i18n,
        jwt,
        queue,
        &it,
        &Action::ResetPassword,
        &locale,
        &host.hostname,
    )?;
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

#[post("/reset-password", format = "json", data = "<form>")]
pub fn reset_password(
    form: Json<ResetPassword>,
    remote: SocketAddr,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> Result<JsonValue> {
    form.validate()?;
    let token = jwt.parse::<Token>(&form.token)?.claims;
    if token.act != Action::ResetPassword {
        return Err("bad action".into());
    }

    let db = db.deref();
    let ip = remote.ip();
    let it = UserDao::by_uid(db, &token.uid)?;

    UserDao::password::<Sodium>(db, &it.id, &form.password)?;
    LogDao::add(db, &it.id, &ip, "Reset password")?;
    Ok(json!({}))
}

#[get("/logs")]
pub fn logs(user: CurrentUser, db: Database) -> Result<Json<Vec<Log>>> {
    let db = db.deref();
    let items = LogDao::all(db, &user.id, 1 << 10)?;
    Ok(Json(items))
}

#[get("/profile")]
pub fn get_profile(user: CurrentUser, db: Database) -> Result<JsonValue> {
    let db = db.deref();
    let it = UserDao::by_id(db, &user.id)?;
    Ok(
        json!({"email":it.email, "nick_name":it.nick_name, "real_name":it.real_name, "logo":it.logo}),
    )
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[validate(length(min = "1"))]
    pub real_name: String,
    #[validate(length(min = "1"))]
    pub logo: String,
}

#[post("/profile", format = "json", data = "<form>")]
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

#[post("/change-password", format = "json", data = "<form>")]
pub fn change_password(
    db: Database,
    form: Json<ChangePassword>,
    user: CurrentUser,
    remote: SocketAddr,
) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    let ip = remote.ip();
    let user = UserDao::by_id(db, &user.id)?;
    user.auth::<Sodium>(&form.current_password)?;
    UserDao::password::<Sodium>(db, &user.id, &form.new_password)?;
    LogDao::add(db, &user.id, &ip, "Change password")?;
    Ok(Json(()))
}

#[delete("/sign-out")]
pub fn sign_out(db: Database, user: CurrentUser, remote: SocketAddr) -> Result<Json<()>> {
    let db = db.deref();
    let ip = remote.ip();
    LogDao::add(db, &user.id, &ip, "Sign out")?;
    Ok(Json(()))
}

fn send_email(
    i18n: &I18n,
    jwt: &Jwt,
    queue: &RabbitMQ,
    user: &User,
    act: &Action,
    locale: &String,
    host: &String,
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
    let subject = i18n.t(
        locale,
        &format!("nut.emails.users.{}.subject", act),
        &Some(json!({})),
    );
    let body = i18n.t(
        locale,
        &format!("nut.emails.users.{}.body", act),
        &Some(json!({ "host": host, "expire":expire, "token":token })),
    );

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
