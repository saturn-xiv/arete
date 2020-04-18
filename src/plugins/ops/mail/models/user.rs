use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::super::{
    crypto::{Random, SSha512},
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::{ops_mail_aliases, ops_mail_users};

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub domain_id: ID,
    pub name: String,
    pub email: String,
    pub password: String,
    pub locked_at: Option<NaiveDateTime>,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub const SALT: usize = 8;
    pub fn auth<E: SSha512>(&self, password: &str) -> Result<()> {
        if !E::verify(&self.password, password.as_bytes()) {
            return Err(format_err!("bad password"));
        }
        Ok(())
    }
    pub fn sum<S: Random + SSha512>(password: &str) -> String {
        let buf = S::bytes(Self::SALT);
        S::sum(password.as_bytes(), &buf)
    }
}

pub trait Dao {
    fn all(&self, domain: ID) -> Result<Vec<Item>>;
    fn get(&self, id: ID) -> Result<Item>;
    fn add<S: Random + SSha512>(
        &self,
        domain: ID,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<()>;
    fn set_password<S: Random + SSha512>(&self, id: ID, password: &str) -> Result<()>;
    fn lock(&self, id: ID, on: bool) -> Result<()>;
    fn set_name(&self, id: ID, name: &str) -> Result<()>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self, domain: ID) -> Result<Vec<Item>> {
        let items = ops_mail_users::dsl::ops_mail_users
            .filter(ops_mail_users::dsl::domain_id.eq(domain))
            .order(ops_mail_users::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn get(&self, id: ID) -> Result<Item> {
        let it = ops_mail_users::dsl::ops_mail_users
            .filter(ops_mail_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn add<S: Random + SSha512>(
        &self,
        domain: ID,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<()> {
        let email = email.trim().to_lowercase();
        let password = Item::sum::<S>(password);
        let now = Utc::now().naive_local();
        insert_into(ops_mail_users::dsl::ops_mail_users)
            .values((
                ops_mail_users::dsl::domain_id.eq(domain),
                ops_mail_users::dsl::name.eq(name),
                ops_mail_users::dsl::email.eq(&email),
                ops_mail_users::dsl::password.eq(&password),
                ops_mail_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_password<S: Random + SSha512>(&self, id: ID, password: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let password = Item::sum::<S>(password);
        let it = ops_mail_users::dsl::ops_mail_users.filter(ops_mail_users::dsl::id.eq(id));
        update(it)
            .set((
                ops_mail_users::dsl::password.eq(&password),
                ops_mail_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn set_name(&self, id: ID, name: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = ops_mail_users::dsl::ops_mail_users.filter(ops_mail_users::dsl::id.eq(id));
        update(it)
            .set((
                ops_mail_users::dsl::name.eq(name),
                ops_mail_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&self, id: ID, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = ops_mail_users::dsl::ops_mail_users.filter(ops_mail_users::dsl::id.eq(id));
        update(it)
            .set((
                ops_mail_users::dsl::locked_at.eq(&if on { Some(now) } else { None }),
                ops_mail_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&self, id: ID) -> Result<()> {
        let email = ops_mail_users::dsl::ops_mail_users
            .select(ops_mail_users::dsl::email)
            .filter(ops_mail_users::dsl::id.eq(id))
            .first::<String>(self)?;
        delete(
            ops_mail_aliases::dsl::ops_mail_aliases
                .filter(ops_mail_aliases::dsl::source.eq(&email)),
        )
        .execute(self)?;
        delete(ops_mail_users::dsl::ops_mail_users.filter(ops_mail_users::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
