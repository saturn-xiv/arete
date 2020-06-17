use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*};

use super::super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::ops_mail_aliases;

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub domain_id: ID,
    pub source: String,
    pub destination: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self, domain: ID) -> Result<Vec<Item>>;
    fn bind(&self, domain: ID, source: &str, destination: &str) -> Result<()>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self, domain: ID) -> Result<Vec<Item>> {
        let items = ops_mail_aliases::dsl::ops_mail_aliases
            .filter(ops_mail_aliases::dsl::id.eq(domain))
            .order(ops_mail_aliases::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn bind(&self, domain: ID, source: &str, destination: &str) -> Result<()> {
        let source = source.trim().to_lowercase();
        let destination = destination.trim().to_lowercase();

        if ops_mail_aliases::dsl::ops_mail_aliases
            .filter(ops_mail_aliases::dsl::domain_id.eq(&domain))
            .filter(ops_mail_aliases::dsl::source.eq(&source))
            .filter(ops_mail_aliases::dsl::destination.eq(&destination))
            .first::<Item>(self)
            .is_ok()
        {
            return Ok(());
        }
        insert_into(ops_mail_aliases::dsl::ops_mail_aliases)
            .values((
                ops_mail_aliases::dsl::source.eq(&source),
                ops_mail_aliases::dsl::destination.eq(&destination),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&self, id: ID) -> Result<()> {
        delete(ops_mail_aliases::dsl::ops_mail_aliases.filter(ops_mail_aliases::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
