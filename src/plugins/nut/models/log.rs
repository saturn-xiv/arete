use std::net::IpAddr;
use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use ipnetwork::IpNetwork;
use serde::ser::Serialize;

use super::super::super::super::{
    errors::Result,
    i18n::I18n,
    orm::{schema::logs, Connection},
};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub ip: IpNetwork,
    pub message: String,
    pub created_at: NaiveDateTime,
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

impl I18n {
    pub fn l<C: Into<String>, S: Serialize>(
        &self,
        user: &i64,
        code: C,
        args: &Option<S>,
    ) -> Result<()> {
        let db = self.db.deref();
        Dao::add(db, user, &self.ip, self.t(code, args))
    }
}
