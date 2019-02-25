use std::ops::Deref;

use chrono::NaiveDateTime;

use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, BigSerial, Handler},
};
use super::super::{models::leave_word::Dao as LeaveWordDao, MediaType};

#[derive(GraphQLInputObject, Validate)]
pub struct Create {
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
        LeaveWordDao::add(
            db,
            &s.client_ip,
            &self.body,
            &self.media_type.parse::<MediaType>()?,
        )?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct LeaveWord {
    pub id: BigSerial,
    pub ip: Option<String>,
    pub body: String,
    pub media_type: String,
    pub created_at: NaiveDateTime,
}

#[derive(Validate)]
pub struct Index {
    pub limit: i64,
}

impl Handler for Index {
    type Item = Vec<LeaveWord>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;
        let items = LeaveWordDao::all(db, self.limit)?
            .into_iter()
            .map(|x| LeaveWord {
                id: BigSerial(x.id),
                ip: x.ip,
                body: x.body,
                media_type: x.media_type,
                created_at: x.created_at,
            })
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
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;
        LeaveWordDao::delete(db, &self.id)?;
        Ok(())
    }
}
