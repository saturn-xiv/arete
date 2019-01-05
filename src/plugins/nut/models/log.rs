use std::net::IpAddr;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use ipnetwork::IpNetwork;

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

#[derive(Insertable)]
#[table_name = "logs"]
pub struct New<'a> {
    pub user_id: &'a i64,
    pub ip: &'a IpNetwork,
    pub message: &'a str,
}

pub trait Dao {
    fn add<S: Into<String>>(&self, user: &i64, ip: &IpAddr, message: S) -> Result<()>;
    fn all(&self, user: &i64, limit: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add<S: Into<String>>(&self, user: &i64, ip: &IpAddr, message: S) -> Result<()> {
        let ip: IpNetwork = (*ip).into();
        insert_into(logs::dsl::logs)
            .values(&New {
                user_id: user,
                ip: &ip,
                message: &message.into(),
            })
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
