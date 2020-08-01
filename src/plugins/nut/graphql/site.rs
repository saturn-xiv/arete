use std::ops::Deref;

use diesel::Connection;
use failure::Error;
use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::super::{
    errors::Result, graphql::context::Context, orm::Connection as Db,
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
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        self.save(db, &ctx.locale)
    }
    pub fn save(&self, db: &Db, lang: &str) -> Result<()> {
        db.transaction::<_, Error, _>(|| {
            UpdateLocale::save(db, lang, "site.title", &self.title)?;
            UpdateLocale::save(db, lang, "site.subhead", &self.title)?;
            UpdateLocale::save(db, lang, "site.description", &self.title)?;
            UpdateLocale::save(db, lang, "site.copyright", &self.title)?;
            Ok(())
        })?;
        Ok(())
    }
}
