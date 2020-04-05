use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};

use super::super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::monitor_logs;

#[derive(Queryable, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub uid: String,
    pub code: String,
    pub value: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn by_name_and_code(&self, name: &str, code: &str, limit: i64) -> Result<Vec<Item>>;
    fn by_name(&self, name: &str, limit: i64) -> Result<Vec<Item>>;
    fn by_code(&self, code: &str, limit: i64) -> Result<Vec<Item>>;
    fn add(&self, name: &str, uid: &str, code: &str, v: &str) -> Result<()>;
}

impl Dao for Connection {
    fn by_name_and_code(&self, name: &str, code: &str, limit: i64) -> Result<Vec<Item>> {
        let items = monitor_logs::dsl::monitor_logs
            .filter(monitor_logs::dsl::code.eq(code))
            .filter(monitor_logs::dsl::name.eq(name))
            .order(monitor_logs::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_name(&self, name: &str, limit: i64) -> Result<Vec<Item>> {
        let items = monitor_logs::dsl::monitor_logs
            .filter(monitor_logs::dsl::name.eq(name))
            .order(monitor_logs::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_code(&self, code: &str, limit: i64) -> Result<Vec<Item>> {
        let items = monitor_logs::dsl::monitor_logs
            .filter(monitor_logs::dsl::code.eq(code))
            .order(monitor_logs::dsl::created_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn add(&self, name: &str, uid: &str, code: &str, value: &str) -> Result<()> {
        insert_into(monitor_logs::dsl::monitor_logs)
            .values((
                monitor_logs::dsl::name.eq(name),
                monitor_logs::dsl::uid.eq(uid),
                monitor_logs::dsl::code.eq(code),
                monitor_logs::dsl::value.eq(value),
            ))
            .execute(self)?;
        Ok(())
    }
}
