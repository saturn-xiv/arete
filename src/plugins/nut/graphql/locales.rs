use std::ops::Deref;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, ID as RID},
    i18n::locale::{Dao as LocaleDao, Item as LocaleItem},
    orm::Connection as Db,
};

#[derive(GraphQLObject)]
pub struct Locale {
    pub id: RID,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

impl From<LocaleItem> for Locale {
    fn from(it: LocaleItem) -> Self {
        Self {
            id: it.id.into(),
            message: it.message,
            code: it.code,
            updated_at: it.updated_at,
        }
    }
}

impl Locale {
    pub fn index(ctx: &Context) -> Result<Vec<Self>> {
        let db = ctx.db.deref();
        Ok(LocaleDao::by_lang(db, &ctx.locale)?
            .into_iter()
            .map(|it| it.into())
            .collect::<_>())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub message: String,
}

impl Update {
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        ctx.administrator()?;

        let db = ctx.db.deref();
        Self::save(db, &ctx.locale, &self.code, &self.message)
    }
    pub fn save(db: &Db, lang: &str, code: &str, message: &str) -> Result<()> {
        match LocaleDao::by_lang_and_code(db, lang, code) {
            Ok(it) => {
                LocaleDao::update(db, it.id, code, message)?;
            }
            Err(_) => {
                LocaleDao::create(db, lang, code, message)?;
            }
        };
        Ok(())
    }
}

pub struct Destory;

impl Destory {
    pub fn execute(ctx: &Context, id: RID) -> Result<()> {
        ctx.administrator()?;
        let db = ctx.db.deref();
        LocaleDao::delete(db, id.0)
    }
}
