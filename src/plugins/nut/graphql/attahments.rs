use std::ops::Deref;

use actix_web::http::StatusCode;
use chrono::NaiveDateTime;
use juniper::GraphQLObject;

use super::super::super::super::{
    errors::{Error, Result},
    graphql::{context::Context, I64, ID},
    orm::{Connection as Db, ID as RID},
};
use super::super::models::{
    attachment::{Dao as AttachmentDao, Item},
    policy::{Dao as PolicyDao, Role},
};

#[derive(GraphQLObject)]
pub struct Attachment {
    pub id: ID,
    pub title: String,
    pub size: I64,
    pub mime_type: String,
    pub url: String,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Attachment {
    fn from(it: Item) -> Self {
        Self {
            id: it.id.into(),
            title: it.title,
            size: it.size.into(),
            mime_type: it.mime_type,
            url: it.url,
            updated_at: it.updated_at,
        }
    }
}

impl Attachment {
    pub fn index(ctx: &Context) -> Result<Vec<Self>> {
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        let items = if PolicyDao::is(db, user.id, &Role::Admin) {
            AttachmentDao::all(db)?
        } else {
            AttachmentDao::by_user(db, user.id)?
        };
        Ok(items.into_iter().map(|it| it.into()).collect::<_>())
    }

    pub fn can(db: &Db, user: RID, id: RID) -> Result<Item> {
        let it = AttachmentDao::by_id(db, id)?;
        if it.user_id == user || PolicyDao::is(db, user, &Role::Admin) {
            return Ok(it);
        }
        Err(Error::Http(StatusCode::FORBIDDEN).into())
    }
}

pub struct Destory;

impl Destory {
    pub fn execute(ctx: &Context, id: ID) -> Result<()> {
        let user = ctx.current_user()?;
        let db = ctx.db.deref();
        Attachment::can(db, user.id, id.0)?;
        AttachmentDao::delete(db, id.0)?;
        Ok(())
    }
}
