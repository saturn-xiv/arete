use std::ops::Deref;

use chrono::NaiveDateTime;
use rocket::http::Status;
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, session::Session, Handler, I64},
};
use super::super::models::{
    attachment::{Dao as AttachmentDao, Item},
    policy::{Dao as PolicyDao, Role},
};

#[derive(GraphQLObject)]
pub struct Attachment {
    pub id: I64,
    pub title: String,
    pub mime_type: String,
    pub url: String,
    pub size: I64,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Attachment {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            title: it.title,
            mime_type: it.mime_type,
            url: it.url,
            size: I64(it.size),
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Show {
    pub id: i64,
}

impl Handler for Show {
    type Item = Attachment;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let it = AttachmentDao::by_id(db, &self.id)?;
        Ok(it.into())
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Attachment>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let user = s.current_user()?;
        let items = if PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            AttachmentDao::all(db)?
        } else {
            AttachmentDao::by_user(db, &user.id)?
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
        let db = c.db.deref();
        let user = s.current_user()?;
        let it = AttachmentDao::by_id(db, &self.id)?;
        if it.user_id == self.id || PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            AttachmentDao::delete(db, &self.id)?;
            return Ok(());
        }
        Err(Error::Http(Status::Forbidden).into())
    }
}
