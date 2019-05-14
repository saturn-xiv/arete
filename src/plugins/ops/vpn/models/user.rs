use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};

use super::super::super::super::super::{
    crypto::Password,
    errors::{Error, Result},
    orm::{Connection, ID},
};
use super::super::schema::vpn_users;

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub password: Vec<u8>,
    pub online: bool,
    pub locked_at: Option<NaiveDateTime>,
    pub startup: NaiveDate,
    pub shutdown: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn auth<E: Password>(&self, password: &String) -> Result<()> {
        if E::verify(&self.password, password.as_bytes()) {
            return Ok(());
        }
        return Err(Error::UserBadPassword.into());
    }
}

#[derive(Insertable)]
#[table_name = "vpn_users"]
pub struct New<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a [u8],
    pub startup: &'a NaiveDate,
    pub shutdown: &'a NaiveDate,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: ID) -> Result<Item>;
    fn by_email(&self, email: &String) -> Result<Item>;
    fn online(&self, id: ID, on: bool) -> Result<()>;
    fn enable(&self, id: ID, startup: &NaiveDate, shutdown: &NaiveDate) -> Result<()>;
    fn add<T: Password>(
        &self,
        name: &String,
        email: &String,
        password: &String,
        startup: &NaiveDate,
        shutdown: &NaiveDate,
    ) -> Result<()>;
    fn lock(&self, id: ID, on: bool) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn update<T: Password>(
        &self,
        id: ID,
        name: &String,
        password: &String,
        startup: &NaiveDate,
        shutdown: &NaiveDate,
    ) -> Result<()>;
    fn password<T: Password>(&self, id: ID, password: &String) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = vpn_users::dsl::vpn_users
            .filter(vpn_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_email(&self, email: &String) -> Result<Item> {
        let it = vpn_users::dsl::vpn_users
            .filter(vpn_users::dsl::email.eq(&email.trim().to_lowercase()))
            .first(self)?;
        Ok(it)
    }

    fn add<T: Password>(
        &self,
        name: &String,
        email: &String,
        password: &String,
        startup: &NaiveDate,
        shutdown: &NaiveDate,
    ) -> Result<()> {
        let email = email.trim().to_lowercase();
        insert_into(vpn_users::dsl::vpn_users)
            .values(&New {
                name: name,
                email: &email,
                startup: startup,
                shutdown: shutdown,
                password: &T::sum(password.as_bytes())?,
                updated_at: &Utc::now().naive_utc(),
            })
            .execute(self)?;
        Ok(())
    }

    fn update<T: Password>(
        &self,
        id: ID,
        name: &String,
        password: &String,
        startup: &NaiveDate,
        shutdown: &NaiveDate,
    ) -> Result<()> {
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::name.eq(name),
                vpn_users::dsl::startup.eq(startup),
                vpn_users::dsl::shutdown.eq(shutdown),
                vpn_users::dsl::password.eq(&T::sum(password.as_bytes())?),
                vpn_users::dsl::updated_at.eq(&Utc::now().naive_utc()),
            ))
            .execute(self)?;
        Ok(())
    }

    fn lock(&self, id: ID, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::locked_at.eq(&if on { Some(now) } else { None }),
                vpn_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn online(&self, id: ID, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::online.eq(on),
                vpn_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn enable(&self, id: ID, startup: &NaiveDate, shutdown: &NaiveDate) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::startup.eq(startup),
                vpn_users::dsl::shutdown.eq(shutdown),
                vpn_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = vpn_users::dsl::vpn_users
            .order(vpn_users::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn password<T: Password>(&self, id: ID, password: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        let password = T::sum(password.as_bytes())?;
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::password.eq(&password),
                vpn_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}
