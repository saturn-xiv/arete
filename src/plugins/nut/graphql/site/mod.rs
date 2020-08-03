pub mod status;

use std::ops::Deref;

use diesel::Connection;
use failure::Error as FailureError;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    cache::Provider, crypto::Crypto, errors::Result, graphql::context::Context,
    orm::Connection as Db, queue::Task, settings::Dao as SettingDao,
};
use super::super::tasks::send_email;
use super::locales::Update as UpdateLocale;

#[derive(GraphQLInputObject, Validate)]
pub struct Info {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1, max = 12))]
    pub subhead: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(length(min = 1))]
    pub copyright: String,
}

impl Info {
    pub const TITLE: &'static str = "site.title";
    pub const SUBHEAD: &'static str = "site.subhead";
    pub const DESCRIPTION: &'static str = "site.description";
    pub const COPYRIGHT: &'static str = "site.copyright";
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        self.save(db, &ctx.locale)
    }
    pub fn save(&self, db: &Db, lang: &str) -> Result<()> {
        db.transaction::<_, FailureError, _>(|| {
            UpdateLocale::save(db, lang, Self::TITLE, &self.title)?;
            UpdateLocale::save(db, lang, Self::SUBHEAD, &self.subhead)?;
            UpdateLocale::save(db, lang, Self::DESCRIPTION, &self.description)?;
            UpdateLocale::save(db, lang, Self::COPYRIGHT, &self.copyright)?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Validate)]
pub struct Author {
    #[validate(length(min = 1), email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub name: String,
}

impl Default for Author {
    fn default() -> Self {
        Self {
            name: "who-am-i".to_string(),
            email: "change-me@gmail.com".to_string(),
        }
    }
}

impl Author {
    pub const KEY: &'static str = "site.author";
    pub fn get(ctx: &Context) -> Result<Self> {
        let db = ctx.db.deref();
        let enc = ctx.crypto.deref();
        let it: Self = match SettingDao::get(db, enc, &Author::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => Self::default(),
        };
        Ok(it)
    }
    pub fn set(ctx: &Context, email: &str, name: &str) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        let enc = ctx.crypto.deref();
        SettingDao::set::<String, Self, Crypto>(
            db,
            enc,
            &Self::KEY.to_string(),
            &Self {
                email: email.to_string(),
                name: name.to_string(),
            },
            false,
        )?;
        Ok(())
    }
}

#[derive(GraphQLObject, Default, Validate, Serialize, Deserialize)]
pub struct Seo {
    pub google: Option<Google>,
    pub baidu: Option<Baidu>,
    pub keywords: Vec<String>,
}

#[derive(GraphQLObject, Validate, Serialize, Deserialize)]
pub struct Google {
    #[validate(length(min = 1))]
    pub verify_id: String,
}

#[derive(GraphQLObject, Validate, Serialize, Deserialize)]
pub struct Baidu {
    #[validate(length(min = 1))]
    pub verify_id: String,
}

impl Seo {
    pub const KEY: &'static str = "site.seo";

    pub fn get(ctx: &Context) -> Result<Self> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        let enc = ctx.crypto.deref();
        let it: Self = match SettingDao::get(db, enc, &Self::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => Self::default(),
        };
        Ok(it)
    }

    pub fn set(&self, ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        let enc = ctx.crypto.deref();
        SettingDao::set::<String, Self, Crypto>(db, enc, &Self::KEY.to_string(), self, false)?;
        Ok(())
    }
}

impl send_email::Config {
    pub fn get(ctx: &Context) -> Result<Self> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        let enc = ctx.crypto.deref();
        let it: Self = match SettingDao::get(db, enc, &Self::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => Self::default(),
        };
        Ok(it)
    }

    pub fn set(&self, ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        let enc = ctx.crypto.deref();
        SettingDao::set::<String, Self, Crypto>(db, enc, &Self::KEY.to_string(), self, true)?;
        Ok(())
    }

    pub async fn test(ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        let user = ctx.administrator()?;
        let queue = ctx.queue.deref();
        queue
            .publish(
                send_email::NAME,
                Task::new(&send_email::Task {
                    email: user.email.clone(),
                    name: user.real_name.clone(),
                    subject: format!("Hi, {}", user.real_name),
                    body: "This is a test email.".to_string(),
                })?,
            )
            .await?;
        Ok(())
    }
}

pub struct ClearCache;
impl ClearCache {
    pub fn execute(ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        if let Ok(mut ch) = ctx.cache.lock() {
            ch.clear()?;
        }
        Ok(())
    }
}
