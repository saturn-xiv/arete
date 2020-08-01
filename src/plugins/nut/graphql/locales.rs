use std::ops::Deref;

use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::super::{
    errors::Result, graphql::context::Context, i18n::locale::Dao as LocaleDao,
    orm::Connection as Db,
};

#[derive(GraphQLObject)]
pub struct Show {
    pub id: String,
    pub code: String,
    pub message: String,
}

impl Show {
    pub fn load(ctx: &Context) -> Result<Vec<Self>> {
        let db = ctx.db.deref();
        Ok(LocaleDao::by_lang(db, &ctx.locale)?
            .into_iter()
            .map(|it| Self {
                id: it.id.to_string(),
                code: it.code,
                message: it.message,
            })
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
