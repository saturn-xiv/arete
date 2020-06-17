use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::logs;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub ip: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add<S: Into<String>>(&self, user: ID, ip: &str, message: S) -> Result<()>;
    fn all(&self, user: ID, offset: i64, limit: i64) -> Result<Vec<Item>>;
    fn count(&self, user: ID) -> Result<i64>;
}

impl Dao for Connection {
    fn add<S: Into<String>>(&self, user: ID, ip: &str, message: S) -> Result<()> {
        insert_into(logs::dsl::logs)
            .values((
                logs::dsl::user_id.eq(user),
                logs::dsl::ip.eq(ip),
                logs::dsl::message.eq(&message.into()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self, user: ID, offset: i64, limit: i64) -> Result<Vec<Item>> {
        let items = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .order(logs::dsl::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn count(&self, user: ID) -> Result<i64> {
        let it = logs::dsl::logs
            .filter(logs::dsl::user_id.eq(user))
            .count()
            .first(self)?;
        Ok(it)
    }
}
