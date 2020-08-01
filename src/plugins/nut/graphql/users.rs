use std::ops::Deref;

use chrono::Duration;
use diesel::Connection;
use failure::Error;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto,
    errors::Result,
    graphql::{context::Context, Pager, Pagination},
    i18n::I18n,
    jwt::Jwt,
    orm::Connection as Db,
    queue::{rabbitmq::RabbitMQ, Task},
};
use super::super::{
    models::{
        log::{Dao as LogDao, Log},
        policy::{Dao as PolicyDao, Policy},
        user::{Dao as UserDao, Item as User},
    },
    request::{Action, Token},
    tasks::send_email,
};

#[derive(GraphQLInputObject, Validate)]
pub struct SignIn {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl SignIn {
    pub fn execute(&self, ctx: &Context) -> Result<String> {
        self.validate()?;
        let db = ctx.db.deref();
        let user: Result<User> = self.select(db).ok_or(__i18n_e!(
            db,
            &ctx.locale,
            "nut.errors.user.is-not-exist",
            &json!({"id": self.id})
        ));
        let user = user?;
        if let Err(e) = user.auth::<Crypto>(&self.password) {
            __i18n_l!(
                db,
                user.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.sign-in.failed"
            )?;
            return Err(e);
        }
        user.available()?;

        let uid = user.uid.clone();
        let name = user.real_name.clone();
        db.transaction::<_, Error, _>(move || {
            UserDao::sign_in(db, user.id, &ctx.client_ip)?;
            __i18n_l!(
                db,
                user.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.sign-in.success"
            )?;
            Ok(())
        })?;
        let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
        let token = ctx.jwt.sum(
            None,
            &Token {
                uid,
                sub: name,
                act: Action::SignIn,
                nbf,
                exp,
            },
        )?;

        Ok(token)
    }

    fn select(&self, db: &Db) -> Option<User> {
        if let Ok(it) = UserDao::by_nick_name(db, &self.id) {
            return Some(it);
        }
        if let Ok(it) = UserDao::by_email(db, &self.id) {
            return Some(it);
        }
        None
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct SignUp {
    #[validate(length(min = 2, max = 32))]
    pub real_name: String,
    #[validate(length(min = 2, max = 32))]
    pub nick_name: String,
    #[validate(length(min = 1, max = 255), email)]
    pub email: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
    #[validate(length(min = 6))]
    pub home: String,
}

impl SignUp {
    pub async fn execute(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        if let Ok(_) = UserDao::by_email(db, &self.email) {
            return Err(__i18n_e!(
                db,
                &ctx.locale,
                "nut.errors.user.already-exist",
                &json!({"id": self.email})
            ));
        }
        if let Ok(_) = UserDao::by_nick_name(db, &self.nick_name) {
            return Err(__i18n_e!(
                db,
                &ctx.locale,
                "nut.errors.user.already-exist",
                &json!({"id": self.nick_name})
            ));
        }

        let user = db.transaction::<_, Error, _>(move || {
            UserDao::sign_up::<Crypto>(
                db,
                &self.real_name,
                &self.nick_name,
                &self.email,
                &self.password,
            )?;
            let it = UserDao::by_email(db, &self.email)?;
            __i18n_l!(
                db,
                it.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.sign-up"
            )?;
            Ok(it)
        })?;
        EmailForm::send_email(
            db,
            &ctx.locale,
            &ctx.jwt,
            &ctx.queue,
            &user,
            &Action::Confirm,
            &self.home,
        )
        .await?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct EmailForm {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub home: String,
}

impl EmailForm {
    pub async fn confirm(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        let it = UserDao::by_email(db, &self.email)?;
        if let Some(_) = it.confirmed_at {
            return Err(__i18n_e!(
                db,
                &ctx.locale,
                "nut.errors.user.already-confirm"
            ));
        }
        Self::send_email(
            db,
            &ctx.locale,
            &ctx.jwt,
            &ctx.queue,
            &it,
            &Action::Confirm,
            &self.home,
        )
        .await?;
        Ok(())
    }

    pub async fn unlock(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        let it = UserDao::by_email(db, &self.email)?;
        if None == it.locked_at {
            return Err(__i18n_e!(db, &ctx.locale, "nut.errors.user.is-not-lock"));
        }
        Self::send_email(
            &db,
            &ctx.locale,
            &ctx.jwt,
            &ctx.queue,
            &it,
            &Action::Unlock,
            &self.home,
        )
        .await?;
        Ok(())
    }

    pub async fn forgot_password(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        let it = UserDao::by_email(db, &self.email)?;
        Self::send_email(
            db,
            &ctx.locale,
            &ctx.jwt,
            &ctx.queue,
            &it,
            &Action::ResetPassword,
            &self.home,
        )
        .await?;
        Ok(())
    }

    async fn send_email(
        db: &Db,
        lang: &str,
        jwt: &Jwt,
        queue: &RabbitMQ,
        user: &User,
        act: &Action,
        home: &str,
    ) -> Result<()> {
        let expire = 1;
        let (nbf, exp) = Jwt::timestamps(Duration::hours(expire));
        let token = jwt.sum(
            None,
            &Token {
                uid: user.uid.clone(),
                sub: user.real_name.clone(),
                act: act.clone(),
                nbf: nbf,
                exp: exp,
            },
        )?;

        let args =
            Some(json!({ "name": user.real_name, "home": home, "expire":expire, "token":token }));
        let subject = I18n::t(db, lang, format!("nut.mailer.users.{}.subject", act), &args);
        let body = I18n::t(db, lang, format!("nut.mailer.users.{}.body", act), &args);

        queue
            .publish(
                send_email::NAME,
                Task::new(&send_email::Task {
                    email: user.email.clone(),
                    name: user.real_name.clone(),
                    subject: subject,
                    body: body,
                })?,
            )
            .await?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct TokenForm {
    #[validate(length(min = 1))]
    pub token: String,
}

impl TokenForm {
    pub fn confirm(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();

        let token = ctx.jwt.parse::<Token>(&self.token)?.claims;
        if token.act != Action::Confirm {
            return Err(__i18n_e!(db, &ctx.locale, "nut.errors.bad-action"));
        }

        let it = UserDao::by_uid(db, &token.uid)?;
        if let Some(_) = it.confirmed_at {
            return Err(__i18n_e!(
                db,
                &ctx.locale,
                "nut.errors.user.already-confirm"
            ));
        }

        db.transaction::<_, Error, _>(move || {
            UserDao::confirm(db, it.id)?;
            __i18n_l!(
                db,
                it.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.confirm"
            )?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn unlock(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        let token = ctx.jwt.parse::<Token>(&self.token)?.claims;
        if token.act != Action::Unlock {
            return Err(__i18n_e!(db, &ctx.locale, "nut.errors.bad-action"));
        }
        let it = UserDao::by_uid(db, &token.uid)?;
        if None == it.locked_at {
            return Err(__i18n_e!(db, &ctx.locale, "nut.errors.user.is-not-lock"));
        }
        db.transaction::<_, Error, _>(move || {
            UserDao::lock(db, it.id, false)?;
            __i18n_l!(
                db,
                it.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.unlock"
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct ResetPassword {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

impl ResetPassword {
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();

        let token = ctx.jwt.parse::<Token>(&self.token)?.claims;
        if token.act != Action::ResetPassword {
            return Err(__i18n_e!(db, &ctx.locale, "nut.errors.bad-action"));
        }

        let it = UserDao::by_uid(db, &token.uid)?;

        UserDao::password::<Crypto>(db, it.id, &self.password)?;
        __i18n_l!(
            db,
            it.id,
            &ctx.client_ip,
            &ctx.locale,
            "nut.logs.user.reset-password"
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 1))]
    pub current_password: String,
    #[validate(length(min = 6, max = 32))]
    pub new_password: String,
}

impl ChangePassword {
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;

        user.auth::<Crypto>(&self.current_password)?;
        db.transaction::<_, Error, _>(move || {
            UserDao::password::<Crypto>(db, user.id, &self.new_password)?;
            __i18n_l!(
                db,
                user.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.change-password"
            )?;
            Ok(())
        })?;
        Ok(())
    }
}

pub struct SignOut;

impl SignOut {
    pub fn execute(ctx: &Context) -> Result<()> {
        let db = ctx.db.deref();
        let user = ctx.current_user()?;

        __i18n_l!(
            db,
            user.id,
            &ctx.client_ip,
            &ctx.locale,
            "nut.logs.user.sign-out"
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Profile {
    #[validate(length(min = 2, max = 32))]
    pub real_name: String,
    #[validate(length(min = 1))]
    pub logo: String,
}

impl Profile {
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        let user = ctx.current_user()?;

        UserDao::set_profile(db, user.id, &self.real_name, &self.logo)?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct CurrentUser {
    pub nick_name: String,
    pub real_name: String,
    pub email: String,
    pub policies: Vec<Policy>,
}

impl CurrentUser {
    pub fn new(ctx: &Context) -> Result<Self> {
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        Ok(Self {
            nick_name: user.nick_name.clone(),
            real_name: user.real_name.clone(),
            email: user.email.clone(),
            policies: PolicyDao::all(db, user.id)?
                .into_iter()
                .map(|it| it.into())
                .collect::<_>(),
        })
    }
}

#[derive(GraphQLObject)]
pub struct Logs {
    pagination: Pagination,
    items: Vec<Log>,
}

impl Logs {
    pub fn new(ctx: &Context, pag: &Pager) -> Result<Self> {
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        let total = LogDao::count(db, user.id)?;
        Ok(Self {
            pagination: Pagination::new(total, pag),
            items: LogDao::all(db, user.id, pag.offset(total), pag.limit())?
                .into_iter()
                .map(|it| it.into())
                .collect::<_>(),
        })
    }
}
