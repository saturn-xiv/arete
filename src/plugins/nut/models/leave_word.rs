use std::net::IpAddr;

use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*};
use ipnetwork::IpNetwork;

use super::super::super::super::{
    errors::Result,
    orm::{schema::leave_words, Connection},
};
use super::super::MediaType;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub ip: IpNetwork,
    pub body: String,
    pub media_type: String,
    pub created_at: NaiveDateTime,
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
