use std::net::IpAddr;
use std::result::Result as StdResult;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use ipnetwork::IpNetwork;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::super::super::super::{
    errors::Result,
    orm::{schema::logs, Connection},
};

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub ip: IpNetwork,
    pub message: String,
    pub created_at: NaiveDateTime,
}

// FIXME
impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("log", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("userId", &self.user_id)?;
        state.serialize_field("ip", &self.ip.to_string())?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("createdAt", &self.created_at)?;
        state.end()
    }
}

pub trait Dao {
    fn add<S: Into<String>>(&self, user: &i64, ip: &IpAddr, message: S) -> Result<()>;
    fn all(&self, user: &i64, limit: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add<S: Into<String>>(&self, user: &i64, ip: &IpAddr, message: S) -> Result<()> {
        let ip: IpNetwork = (*ip).into();
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(&ip),
                logs::dsl::message.eq(&message.into()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self, user: &i64, limit: i64) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .order(logs::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
}
