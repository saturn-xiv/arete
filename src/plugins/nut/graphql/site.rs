use std::ops::Deref;

use diesel::Connection;
use failure::Error;
use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto, errors::Result, graphql::context::Context, orm::Connection as Db,
    settings::Dao as SettingDao,
};
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
        db.transaction::<_, Error, _>(|| {
            UpdateLocale::save(db, lang, Self::TITLE, &self.title)?;
            UpdateLocale::save(db, lang, Self::SUBHEAD, &self.subhead)?;
            UpdateLocale::save(db, lang, Self::DESCRIPTION, &self.description)?;
            UpdateLocale::save(db, lang, Self::COPYRIGHT, &self.copyright)?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Validate)]
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
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        SettingDao::set::<String, Author, Crypto>(
            db,
            &ctx.crypto,
            &Self::KEY.to_string(),
            self,
            false,
        )?;
        Ok(())
    }
}
