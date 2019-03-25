use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use failure::Error;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I16, I64},
};
use super::super::models::category::{Dao as CategoryDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub icon: String,
    #[validate(length(min = "1"))]
    pub color: String,
    pub position: I16,
    pub parent_id: Option<I64>,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        CategoryDao::create(
            db,
            &match self.parent_id {
                Some(ref v) => Some(v.0),
                None => None,
            },
            &self.name,
            &self.icon,
            &self.color,
            self.position.0,
        )?;
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
    pub position: I16,
    pub parent_id: Option<I64>,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        CategoryDao::update(
            db,
            &self.id.0,
            &match self.parent_id {
                Some(ref v) => Some(v.0),
                None => None,
            },
            &self.name,
            &self.icon,
            &self.color,
            self.position.0,
        )?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct Category {
    pub id: I64,
    pub parent: Option<I64>,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub position: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Category {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            name: it.name,
            icon: it.icon,
            color: it.color,
            parent: match it.parent_id {
                Some(v) => Some(I64(v)),
                None => None,
            },
            position: I16(it.position),
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Category;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let it = CategoryDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Category>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let items = CategoryDao::all(db)?
            .into_iter()
            .map(|x| x.into())
            .collect();
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
        db.transaction::<_, Error, _>(|| {
            CategoryDao::delete(db, &self.id)?;
            Ok(())
        })?;
        Ok(())
    }
}
