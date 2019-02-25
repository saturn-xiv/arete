use std::fmt;
use std::ops::Deref;

use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;
use failure::Error;
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    errors::Result,
    graphql::{context::Context, session::Session, BigSerial, Handler},
    i18n::I18n,
    jwt::Jwt,
    orm::Connection as Db,
    queue::{rabbitmq::RabbitMQ, Queue},
};
use super::super::{
    models::{
        log::Dao as LogDao,
        user::{Dao as UserDao, Item as User, Show as UserInfo},
    },
    tasks::send_email,
};

#[derive(Validate)]
pub struct Current;

impl Handler for Current {
    type Item = Option<UserInfo>;
    fn handle(&self, _: &Context, s: &Session) -> Result<Self::Item> {
        if let Some(ref v) = s.user {
            return Ok(Some((*v).clone().into()));
        }
        Ok(None)
    }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub uid: String,
    pub act: Action,
    pub nbf: i64,
    pub exp: i64,
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(length(min = "1"))]
    pub login: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

impl Handler for SignIn {
    type Item = String;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let user: Result<User> = match UserDao::by_email(db, &self.login) {
            Ok(v) => Ok(v),
            Err(_) => match UserDao::by_nick_name(db, &self.login) {
                Ok(v) => Ok(v),
                Err(_) => __i18n_e!(
                    db,
                    &s.lang,
                    "nut.errors.user.is-not-exist",
                    &Some(json!({"login": self.login}))
                ),
            },
        };
        let user = user?;

        if let Err(e) = user.auth::<Sodium>(&self.password) {
            __i18n_l!(
                db,
                &user.id,
                &s.client_ip,
                &s.lang,
                "nut.logs.user.sign-in.failed"
            )?;
            return Err(e.into());
        }
        user.available()?;
        UserDao::sign_in(db, &user.id, &s.client_ip)?;
        __i18n_l!(
            db,
            &user.id,
            &s.client_ip,
            &s.lang,
            "nut.logs.user.sign-in.success"
        )?;
        let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
        let token = c.jwt.sum(
            None,
            &Token {
                uid: user.uid.clone(),
                act: Action::SignIn,
                nbf: nbf,
                exp: exp,
            },
        )?;
        Ok(token)
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
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

impl Handler for SignUp {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        if let Ok(_) = UserDao::by_email(db, &self.email) {
            return __i18n_e!(
                db,
                &s.lang,
                "nut.errors.already-exist.email",
                &Some(json!({"email": self.email}))
            );
        }
        if let Ok(_) = UserDao::by_nick_name(db, &self.nick_name) {
            return __i18n_e!(
                db,
                &s.lang,
                "nut.errors.already-exist.nick-name",
                &Some(json!({"name": &self.nick_name}))
            );
        }

        let user = db.transaction::<_, Error, _>(move || {
            UserDao::sign_up::<Sodium>(
                db,
                &self.real_name,
                &self.nick_name,
                &self.email,
                &self.password,
            )?;
            let it = UserDao::by_email(db, &self.email)?;
            __i18n_l!(db, &it.id, &s.client_ip, &s.lang, "nut.logs.user.sign-up")?;
            Ok(it)
        })?;

        send_email(
            db,
            &s.lang,
            &c.jwt,
            &c.queue,
            &user,
            &Action::Confirm,
            &self.home,
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Confirm {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "1"))]
    pub home: String,
}

impl Handler for Confirm {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let it = UserDao::by_email(db, &self.email)?;
        if let Some(_) = it.confirmed_at {
            return __i18n_e!(db, &s.lang, "nut.errors.user.already-confirm");
        }
        send_email(
            db,
            &s.lang,
            &c.jwt,
            &c.queue,
            &it,
            &Action::Confirm,
            &self.home,
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmToken {
    #[validate(length(min = "1"))]
    pub token: String,
}

impl Handler for ConfirmToken {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let token = c.jwt.parse::<Token>(&self.token)?.claims;
        if token.act != Action::Confirm {
            return __i18n_e!(db, &s.lang, "flashes.bad-action");
        }

        let it = UserDao::by_uid(db, &token.uid)?;
        if let Some(_) = it.confirmed_at {
            return __i18n_e!(db, &s.lang, "nut.errors.user.already-confirm");
        }

        db.transaction::<_, Error, _>(move || {
            UserDao::confirm(db, &it.id)?;
            __i18n_l!(db, &it.id, &s.client_ip, &s.lang, "nut.logs.user.confirm")?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unlock {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "1"))]
    pub home: String,
}

impl Handler for Unlock {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let it = UserDao::by_email(db, &self.email)?;
        if None == it.locked_at {
            return __i18n_e!(db, &s.lang, "nut.errors.user.is-not-lock");
        }
        send_email(
            &db,
            &s.lang,
            &c.jwt,
            &c.queue,
            &it,
            &Action::Unlock,
            &self.home,
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockToken {
    #[validate(length(min = "1"))]
    pub token: String,
}

impl Handler for UnlockToken {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let token = c.jwt.parse::<Token>(&self.token)?.claims;
        if token.act != Action::Unlock {
            return __i18n_e!(db, &s.lang, "flashes.bad-action");
        }

        let it = UserDao::by_uid(db, &token.uid)?;
        if None == it.locked_at {
            return __i18n_e!(db, &s.lang, "nut.errors.user.is-not-lock");
        }
        db.transaction::<_, Error, _>(move || {
            UserDao::unlock(db, &it.id)?;
            __i18n_l!(db, &it.id, &s.client_ip, &s.lang, "nut.logs.user.unlock")?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPassword {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "1"))]
    pub home: String,
}

impl Handler for ForgotPassword {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let it = UserDao::by_email(db, &self.email)?;
        send_email(
            db,
            &s.lang,
            &c.jwt,
            &c.queue,
            &it,
            &Action::ResetPassword,
            &self.home,
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPassword {
    #[validate(length(min = "1"))]
    pub token: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

impl Handler for ResetPassword {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();

        let token = c.jwt.parse::<Token>(&self.token)?.claims;
        if token.act != Action::ResetPassword {
            return __i18n_e!(db, &s.lang, "flashes.bad-action");
        }

        let db = db.deref();
        let it = UserDao::by_uid(db, &token.uid)?;

        UserDao::password::<Sodium>(db, &it.id, &self.password)?;
        __i18n_l!(
            db,
            &it.id,
            &s.client_ip,
            &s.lang,
            "nut.logs.user.reset-password"
        )?;
        Ok(())
    }
}

#[derive(GraphQLObject, Serialize)]
pub struct Log {
    pub id: BigSerial,
    pub ip: Option<String>,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    #[validate(range(min = "1", max = "10240"))]
    pub limit: i64,
}

impl Handler for Logs {
    type Item = Vec<Log>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        let items = LogDao::all(db, &user.id, self.limit)?
            .iter()
            .map(|it| Log {
                id: BigSerial(it.id),
                ip: it.ip.clone(),
                message: it.message.clone(),
                created_at: it.created_at,
            })
            .collect();
        Ok(items)
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
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

impl Handler for Profile {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        UserDao::set_profile(db, &user.id, &self.real_name, &self.logo)?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    #[validate(length(min = "1"))]
    pub current_password: String,
    #[validate(length(min = "6", max = "32"))]
    pub new_password: String,
}

impl Handler for ChangePassword {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;

        user.auth::<Sodium>(&self.current_password)?;
        db.transaction::<_, Error, _>(move || {
            UserDao::password::<Sodium>(db, &user.id, &self.new_password)?;
            __i18n_l!(
                db,
                &user.id,
                &s.client_ip,
                &s.lang,
                "nut.logs.user.change-password"
            )?;
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOut {}

impl Handler for SignOut {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;

        __i18n_l!(
            db,
            &user.id,
            &s.client_ip,
            &s.lang,
            "nut.logs.user.sign-out"
        )?;

        Ok(())
    }
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
