use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};

use super::super::super::super::{errors::Result, orm::Connection};
use super::super::schema::logs;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub ip: Option<String>,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add<S: Into<String>>(&self, user: &i64, ip: &Option<String>, message: S) -> Result<()>;
    fn all(&self, user: &i64, limit: i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add<S: Into<String>>(&self, user: &i64, ip: &Option<String>, message: S) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(ip),
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
