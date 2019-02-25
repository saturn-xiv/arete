use std::fmt;
use std::ops::Deref;

use chrono::Duration;
use diesel::prelude::*;
use failure::Error;
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    errors::Result,
    graphql::{context::Context, session::Session, Handler},
    i18n::I18n,
    jwt::Jwt,
    orm::Connection as Db,
    queue::{rabbitmq::RabbitMQ, Queue},
};
use super::super::super::{
    models::{
        log::{Dao as LogDao, Item as Log},
        policy::Dao as PolicyDao,
        user::{Dao as UserDao, Item as User, Show as UserInfo},
    },
    tasks::send_email,
};

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
