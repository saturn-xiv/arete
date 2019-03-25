use std::ops::Deref;

use chrono::NaiveDateTime;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I16, I64},
};
use super::super::models::friend_link::{Dao as FriendLinkDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub home: String,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    pub position: I16,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        FriendLinkDao::create(db, &self.title, &self.home, &self.logo, &self.position.0)?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    pub id: I64,
    #[validate(length(min = "1"))]
    pub home: String,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub logo: String,
    pub position: I16,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        FriendLinkDao::update(
            db,
            &self.id.0,
            &self.title,
            &self.home,
            &self.logo,
            &self.position.0,
        )?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct FriendLink {
    pub id: I64,
    pub title: String,
    pub home: String,
    pub logo: String,
    pub position: I16,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for FriendLink {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            home: it.home,
            title: it.title,
            logo: it.logo,
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
    type Item = FriendLink;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let it = FriendLinkDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<FriendLink>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let items = FriendLinkDao::all(db)?
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
        FriendLinkDao::delete(db, &self.id)?;
        Ok(())
    }
}
