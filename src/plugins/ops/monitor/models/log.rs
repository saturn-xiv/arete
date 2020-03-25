use std::fmt::Debug;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::monitor_logs;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Log<T> {
    pub name: String,
    pub code: String,
    pub value: T,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub code: String,
    pub value: Vec<u8>,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn all<V: DeserializeOwned>(&self, name: &str, code: &str, limit: i64) -> Result<Vec<Log<V>>>;
    fn add<V: Serialize>(&self, name: &str, code: &str, v: &V) -> Result<()>;
}

impl Dao for Connection {
    fn all<V: DeserializeOwned>(&self, name: &str, code: &str, limit: i64) -> Result<Vec<Log<V>>> {
        let mut logs = Vec::new();
        for it in monitor_logs::dsl::monitor_logs
            .filter(monitor_logs::dsl::code.eq(code))
            .filter(monitor_logs::dsl::name.eq(name))
            .order(monitor_logs::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?
        {
            logs.push(Log {
                name: it.name,
                code: it.code,
                value: serde_json::from_slice(&it.value)?,
                created_at: it.created_at,
            });
        }
        Ok(logs)
    }
    fn add<V: Serialize>(&self, name: &str, code: &str, v: &V) -> Result<()> {
        insert_into(monitor_logs::dsl::monitor_logs)
            .values((
                monitor_logs::dsl::name.eq(name),
                monitor_logs::dsl::code.eq(code),
                monitor_logs::dsl::value.eq(&serde_json::to_vec(v)?),
            ))
            .execute(self)?;
        Ok(())
    }
}
