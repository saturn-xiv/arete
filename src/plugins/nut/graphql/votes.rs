use std::ops::Deref;

use chrono::NaiveDateTime;

use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, BigSerial, Handler},
};
use super::super::models::vote::{Dao as VoteDao, Item};

#[derive(GraphQLInputObject, Validate)]
pub struct Update {
    #[validate(length(min = "1"))]
    pub resource_type: String,
    pub resource_id: BigSerial,
    pub like: bool,
}

impl Handler for Update {
    type Item = ();
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.current_user()?;
        VoteDao::like(db, &self.resource_type, &self.resource_id.0, self.like)?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct Vote {
    pub id: BigSerial,
    pub point: BigSerial,
    pub resource_type: String,
    pub resource_id: BigSerial,
    pub updated_at: NaiveDateTime,
}

impl From<Item> for Vote {
    fn from(it: Item) -> Self {
        Self {
            id: BigSerial(it.id),
            point: BigSerial(it.id),
            resource_id: BigSerial(it.resource_id),
            resource_type: it.resource_type,
            updated_at: it.updated_at,
        }
    }
}

#[derive(Validate)]
pub struct Index {}

impl Handler for Index {
    type Item = Vec<Vote>;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        let items = VoteDao::all(db)?.into_iter().map(|x| x.into()).collect();
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
        VoteDao::delete(db, &self.id)?;
        Ok(())
    }
}
