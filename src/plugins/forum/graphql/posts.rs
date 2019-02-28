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
use super::super::models::post::{Dao as PostDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    pub topic: I64,
    pub post: Option<I64>,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

impl Handler for Create {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        db.transaction::<_, FailueError, _>(|| {
            PostDao::add(
                db,
                &user.id,
                &self.topic.0,
                &match self.post {
                    Some(ref v) => Some(v.0),
                    None => None,
                },
                &self.body,
                &self.media_type.parse()?,
            )
        })?;
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    pub id: I64,
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        can_edit(db, user.id, self.id.0)?;
        db.transaction::<_, FailueError, _>(|| {
            PostDao::update(db, &self.id.0, &self.body, &self.media_type.parse()?)
        })?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct Post {
    pub id: I64,
    pub topic_id: I64,
    pub post_id: Option<I64>,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Post {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            topic_id: I64(it.topic_id),
            post_id: match it.post_id {
                Some(v) => Some(I64(v)),
                None => None,
            },
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
    type Item = Post;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let it = PostDao::get(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Post>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let user = s.current_user()?;
        let items = if PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            PostDao::latest(db)?
        } else {
            PostDao::by_user(db, &user.id)?
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
        db.transaction::<_, FailueError, _>(|| PostDao::delete(db, &self.id))?;
        Ok(())
    }
}

fn can_edit(db: &Connection, user: i64, id: i64) -> Result<()> {
    let it = PostDao::get(db, &id)?;
    if it.user_id == user || PolicyDao::can(db, &user, &Role::Admin, &None) {
        return Ok(());
    }
    Err(Error::Http(StatusCode::FORBIDDEN).into())
}
