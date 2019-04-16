use std::ops::Deref;

use chrono::NaiveDateTime;
use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I64},
};
use super::super::models::leave_word::{Dao as LeaveWordDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

impl Handler for Create {
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        LeaveWordDao::add(db, &s.client_ip, &self.body, &self.media_type.parse()?)?;
        Ok(None)
    }
}

#[derive(GraphQLObject)]
pub struct LeaveWord {
    pub id: I64,
    pub ip: String,
    pub body: String,
    pub media_type: String,
    pub created_at: NaiveDateTime,
}

impl From<Item> for LeaveWord {
    fn from(it: Item) -> Self {
        Self {
            id: I64(it.id),
            ip: it.ip,
            body: it.body,
            media_type: it.media_type,
            created_at: it.created_at,
        }
    }
}

#[derive(Validate)]
pub struct Index {
    pub limit: i64,
}

impl Handler for Index {
    type Item = Vec<LeaveWord>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        let items = LeaveWordDao::all(db, self.limit)?
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
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        LeaveWordDao::delete(db, &self.id)?;
        Ok(None)
    }
}
