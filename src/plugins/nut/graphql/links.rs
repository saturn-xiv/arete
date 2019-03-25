use std::ops::Deref;

use chrono::NaiveDateTime;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I16, I64},
};
use super::super::models::link::{Dao as LinkDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub label: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub x: I16,
    pub y: I16,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        LinkDao::create(
            db,
            &self.lang,
            &self.label,
            &self.href,
            &self.loc,
            &self.x.0,
            &self.y.0,
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
    pub label: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub x: I16,
    pub y: I16,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        LinkDao::update(
            db,
            &self.id.0,
            &self.lang,
            &self.label,
            &self.href,
            &self.loc,
            &self.x.0,
            &self.y.0,
        )?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct Link {
    pub id: I64,
    pub lang: String,
    pub label: String,
    pub href: String,
    pub loc: String,
    pub x: I16,
    pub y: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Link {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            lang: it.lang,
            label: it.label,
            href: it.href,
            loc: it.loc,
            x: I16(it.x),
            y: I16(it.y),
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Link;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let it = LinkDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Link>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let items = LinkDao::all(db)?.into_iter().map(|x| x.into()).collect();
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
        LinkDao::delete(db, &self.id)?;
        Ok(())
    }
}
