use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*};

use super::super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::{ops_mail_aliases, ops_mail_domains, ops_mail_users};

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn get(&self, id: ID) -> Result<Item>;
    fn add(&self, name: &str) -> Result<()>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = ops_mail_domains::dsl::ops_mail_domains
            .order(ops_mail_domains::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn get(&self, id: ID) -> Result<Item> {
        let it = ops_mail_domains::dsl::ops_mail_domains
            .filter(ops_mail_domains::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn add(&self, name: &str) -> Result<()> {
        let name = name.trim().to_lowercase();
        let now = Utc::now().naive_local();
        insert_into(ops_mail_domains::dsl::ops_mail_domains)
            .values((
                ops_mail_domains::dsl::name.eq(name),
                ops_mail_domains::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&self, id: ID) -> Result<()> {
        delete(
            ops_mail_aliases::dsl::ops_mail_aliases.filter(ops_mail_aliases::dsl::domain_id.eq(id)),
        )
        .execute(self)?;
        delete(ops_mail_users::dsl::ops_mail_users.filter(ops_mail_users::dsl::domain_id.eq(id)))
            .execute(self)?;
        delete(ops_mail_domains::dsl::ops_mail_domains.filter(ops_mail_domains::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
