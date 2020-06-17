use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::super::{
    crypto::Password,
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::{vpn_logs, vpn_users};

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub password: Vec<u8>,
    pub online: bool,
    pub fixed_ip: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub startup: NaiveDate,
    pub shutdown: NaiveDate,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn auth<E: Password>(&self, password: &str) -> Result<()> {
        if !E::verify(&self.password, password.as_bytes()) {
            return Err(format_err!("bad password"));
        }
        Ok(())
    }
    pub fn enable(&self) -> Result<()> {
        if self.locked_at != None {
            return Err(format_err!("user is disable"));
        }
        let now = Utc::now().naive_utc().date();
        if now < self.startup && now > self.shutdown {
            return Err(format_err!("user is expired"));
        }
        Ok(())
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
    fn by_email(&self, email: &str) -> Result<Item>;
    fn online(&self, id: ID, on: bool) -> Result<()>;
    fn enable(&self, id: ID, startup: NaiveDate, shutdown: NaiveDate) -> Result<()>;
    fn add<T: Password>(
        &self,
        name: &str,
        email: &str,
        password: &str,
        startup: NaiveDate,
        shutdown: NaiveDate,
    ) -> Result<()>;
    fn lock(&self, id: ID, on: bool) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn update(&self, id: ID, name: &str, startup: NaiveDate, shutdown: NaiveDate) -> Result<()>;
    fn password<T: Password>(&self, id: ID, password: &str) -> Result<()>;
    fn bind(&self, id: ID, ip: &Option<String>) -> Result<()>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = vpn_users::dsl::vpn_users
            .filter(vpn_users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_email(&self, email: &str) -> Result<Item> {
        let it = vpn_users::dsl::vpn_users
            .filter(vpn_users::dsl::email.eq(&email.trim().to_lowercase()))
            .first(self)?;
        Ok(it)
    }

    fn add<T: Password>(
        &self,
        name: &str,
        email: &str,
        password: &str,
        startup: NaiveDate,
        shutdown: NaiveDate,
    ) -> Result<()> {
        let email = email.trim().to_lowercase();
        insert_into(vpn_users::dsl::vpn_users)
            .values(&New {
                name,
                email: &email,
                startup: &startup,
                shutdown: &shutdown,
                password: &T::sum(password.as_bytes())?,
                updated_at: &Utc::now().naive_utc(),
            })
            .execute(self)?;
        Ok(())
    }

    fn update(&self, id: ID, name: &str, startup: NaiveDate, shutdown: NaiveDate) -> Result<()> {
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::name.eq(name),
                vpn_users::dsl::startup.eq(startup),
                vpn_users::dsl::shutdown.eq(shutdown),
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

    fn enable(&self, id: ID, startup: NaiveDate, shutdown: NaiveDate) -> Result<()> {
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

    fn password<T: Password>(&self, id: ID, password: &str) -> Result<()> {
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

    fn bind(&self, id: ID, ip: &Option<String>) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id));
        update(it)
            .set((
                vpn_users::dsl::fixed_ip.eq(ip),
                vpn_users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn delete(&self, id: ID) -> Result<()> {
        delete(vpn_logs::dsl::vpn_logs.filter(vpn_logs::dsl::user_id.eq(id))).execute(self)?;
        delete(vpn_users::dsl::vpn_users.filter(vpn_users::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
