use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::Connection as DieselConnection;
use failure::Error as FailueError;
use hyper::StatusCode;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, session::Session, Handler, I64},
    orm::Connection,
};
use super::super::super::nut::models::policy::{Dao as PolicyDao, Role};
use super::super::models::topic::{Dao as TopicDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    pub tags: Vec<I64>,
    pub categories: Vec<I64>,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        db.transaction::<_, FailueError, _>(|| {
            TopicDao::add(
                db,
                &user.id,
                &self.title,
                &self.body,
                &self.media_type.parse()?,
                &self.tags.iter().map(|x| x.0).collect(),
                &self.categories.iter().map(|x| x.0).collect(),
            )
        })?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    pub id: I64,
    #[validate(length(min = "1"))]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
    pub tags: Vec<I64>,
    pub categories: Vec<I64>,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        can_edit(db, user.id, self.id.0)?;
        db.transaction::<_, FailueError, _>(|| {
            TopicDao::update(
                db,
                &self.id.0,
                &self.title,
                &self.body,
                &self.media_type.parse()?,
                &self.tags.iter().map(|x| x.0).collect(),
                &self.categories.iter().map(|x| x.0).collect(),
            )
        })?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct Topic {
    pub id: I64,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Topic {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            title: it.title,
            body: it.body,
            media_type: it.media_type,
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Topic;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let it = TopicDao::get(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Topic>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        let items = if PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            TopicDao::latest(db)?
        } else {
            TopicDao::by_user(db, &user.id)?
        };

        Ok(items.into_iter().map(|x| x.into()).collect())
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
        let user = s.current_user()?;
        can_edit(db, user.id, self.id)?;
        db.transaction::<_, FailueError, _>(|| TopicDao::delete(db, &self.id))?;
        Ok(())
    }
}

fn can_edit(db: &Connection, user: i64, topic: i64) -> Result<()> {
    let it = TopicDao::get(db, &topic)?;
    if it.user_id == user || PolicyDao::can(db, &user, &Role::Admin, &None) {
        return Ok(());
    }
    Err(Error::Http(StatusCode::FORBIDDEN).into())
}
