use std::ops::Deref;

use chrono::NaiveDateTime;
use validator::Validate;

use super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I64},
};
use super::locale::{Dao as LocaleDao, Item};

#[derive(GraphQLObject)]
pub struct Locale {
    pub id: I64,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Locale {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            lang: it.lang,
            code: it.code,
            message: it.message,
            updated_at: it.updated_at,
        }
    }
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
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;
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

#[derive(Validate)]
pub struct ByLang {
    #[validate(length(min = "1"))]
    pub lang: String,
}

impl Handler for ByLang {
    type Item = Vec<Locale>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let items = LocaleDao::by_lang(db, &self.lang)?
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(items)
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Locale;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let it = LocaleDao::by_id(db, &self.id)?;

        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Languages;

impl Handler for Languages {
    type Item = Vec<String>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let items = LocaleDao::languages(db)?;
        Ok(items)
    }
}

#[derive(Validate)]
pub struct Destroy {
    pub id: i64,
}

impl Handler for Destroy {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;
        LocaleDao::delete(db, &self.id)?;
        Ok(())
    }
}
