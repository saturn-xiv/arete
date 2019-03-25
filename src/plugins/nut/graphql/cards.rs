use std::ops::Deref;

use chrono::NaiveDateTime;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I16, I64},
};
use super::super::models::card::{Dao as CardDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub action: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub position: I16,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        CardDao::create(
            db,
            &self.lang,
            &self.title,
            &self.logo,
            &self.body,
            &self.media_type.parse()?,
            &self.href,
            &self.action,
            &self.loc,
            &self.position.0,
        )?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    pub id: I64,
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub action: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub position: I16,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        CardDao::update(
            db,
            &self.id.0,
            &self.lang,
            &self.title,
            &self.logo,
            &self.body,
            &self.media_type.parse()?,
            &self.href,
            &self.action,
            &self.loc,
            &self.position.0,
        )?;
        Ok(())
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Card;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let it = CardDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(GraphQLObject)]
pub struct Card {
    pub id: I64,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub action: String,
    pub href: String,
    pub logo: String,
    pub loc: String,
    pub lang: String,
    pub position: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Card {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            title: it.title,
            body: it.body,
            media_type: it.media_type,
            action: it.action,
            href: it.href,
            logo: it.logo,
            loc: it.loc,
            lang: it.lang,
            position: I16(it.position),
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Card>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let items = CardDao::all(db)?.into_iter().map(|x| x.into()).collect();
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
        let db = c.db.deref();
        s.administrator(db)?;
        CardDao::delete(db, &self.id)?;
        Ok(())
    }
}
