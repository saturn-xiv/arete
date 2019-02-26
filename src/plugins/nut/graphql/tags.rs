use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use failure::Error;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I64},
};
use super::super::models::tag::{Dao as TagDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub icon: String,
    #[validate(length(min = "1"))]
    pub color: String,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;
        TagDao::create(db, &self.name, &self.icon, &self.color)?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    pub id: I64,
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub icon: String,
    #[validate(length(min = "1"))]
    pub color: String,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;
        TagDao::update(db, &self.id.0, &self.name, &self.icon, &self.color)?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct Tag {
    pub id: I64,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Tag {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            name: it.name,
            icon: it.icon,
            color: it.color,
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Tag;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let it = TagDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Tag>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let items = TagDao::all(db)?.into_iter().map(|x| x.into()).collect();
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
        db.transaction::<_, Error, _>(|| {
            TagDao::delete(db, &self.id)?;
            Ok(())
        })?;
        Ok(())
    }
}
