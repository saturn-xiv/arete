use std::ops::Deref;

use chrono::NaiveDateTime;
use serde::Serialize;
use validator::Validate;

use super::super::{
    errors::Result,
    graphql::context::{Context, Handler},
};
use super::locale::Dao as LocaleDao;

#[derive(GraphQLObject, Serialize)]
pub struct Item {
    pub id: String,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject, Validate)]
pub struct Save {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub message: String,
}

impl Handler for Save {
    type Item = ();
    fn handle(&self, ctx: &Context) -> Result<Self::Item> {
        let db = ctx.db()?;
        let db = db.deref();
        match LocaleDao::by_lang_and_code(db, &self.lang, &self.code) {
            Ok(it) => {
                LocaleDao::update(db, &it.id, &self.code, &self.message)?;
            }
            Err(_) => {
                LocaleDao::create(db, &self.lang, &self.code, &self.message)?;
            }
        }

        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct ByLang {
    #[validate(length(min = "1"))]
    pub lang: String,
}

impl Handler for ByLang {
    type Item = Vec<Item>;
    fn handle(&self, ctx: &Context) -> Result<Self::Item> {
        let db = ctx.db()?;
        let db = db.deref();
        let items = LocaleDao::by_lang(db, &self.lang)?
            .iter()
            .map(|x| Item {
                id: x.id.to_string(),
                lang: x.lang.clone(),
                code: x.code.clone(),
                message: x.message.clone(),
                updated_at: x.updated_at.clone(),
            })
            .collect();

        Ok(items)
    }
}
