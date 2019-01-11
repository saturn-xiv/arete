use std::net::IpAddr;
use std::result::Result as StdResult;

use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*};
use ipnetwork::IpNetwork;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::super::super::super::{
    errors::Result,
    orm::{schema::leave_words, Connection},
};
use super::super::MediaType;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub ip: IpNetwork,
    pub body: String,
    pub media_type: String,
    pub created_at: NaiveDateTime,
}

// FIXME
impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("leaveWord", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("ip", &self.ip.to_string())?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("mediaType", &self.media_type)?;
        state.serialize_field("createdAt", &self.created_at)?;
        state.end()
    }
}

pub trait Dao {
    fn add(&self, ip: &IpAddr, body: &String, media_type: &MediaType) -> Result<()>;
    fn all(&self, limit: i64) -> Result<Vec<Item>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl Dao for Connection {
    fn add(&self, ip: &IpAddr, body: &String, media_type: &MediaType) -> Result<()> {
        let ip: IpNetwork = (*ip).into();
        insert_into(leave_words::dsl::leave_words)
            .values((
                leave_words::dsl::ip.eq(&ip),
                leave_words::dsl::body.eq(body),
                leave_words::dsl::media_type.eq(&media_type.to_string()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self, limit: i64) -> Result<Vec<Item>> {
        let items = leave_words::dsl::leave_words
            .order(leave_words::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: &i64) -> Result<()> {
        delete(leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
