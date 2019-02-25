use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use md5;
use uuid::Uuid;

use super::super::super::super::{
    crypto::Encryptor,
    errors::{Error, Result},
    orm::Connection,
};
use super::super::schema::users;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    Google,
    Facebook,
    Line,
    Github,
    WeChat,
    Email,
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Google => fmt.write_str("google"),
            Type::Facebook => fmt.write_str("facebook"),
            Type::Github => fmt.write_str("github"),
            Type::WeChat => fmt.write_str("wechat"),
            Type::Line => fmt.write_str("line"),
            Type::Email => fmt.write_str("email"),
        }
    }
}

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub real_name: String,
    pub nick_name: String,
    pub email: String,
    pub password: Option<Vec<u8>>,
    pub uid: String,
    pub provider_type: String,
    pub provider_id: String,
    pub logo: String,
    pub sign_in_count: i64,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn available(&self) -> Result<()> {
        if let Some(_) = self.deleted_at {
            return Err(Error::UserIsDeleted.into());
        }
        if let Some(_) = self.locked_at {
            return Err(Error::UserIsLocked.into());
        }
        if None == self.confirmed_at {
            return Err(Error::UserIsNotConfirmed.into());
        }
        Ok(())
    }
    pub fn auth<E: Encryptor>(&self, password: &String) -> Result<()> {
        if let Some(ref v) = self.password {
            if E::verify(v, password.as_bytes()) {
                return Ok(());
            }
        }
        return Err(Error::UserBadPassword.into());
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct New<'a> {
    pub real_name: &'a str,
    pub nick_name: &'a str,
    pub email: &'a str,
    pub password: Option<&'a [u8]>,
    pub uid: &'a str,
    pub provider_type: &'a str,
    pub provider_id: &'a str,
    pub logo: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: &i64) -> Result<Item>;
    fn by_uid(&self, uid: &String) -> Result<Item>;
    fn by_email(&self, email: &String) -> Result<Item>;
    fn by_nick_name(&self, nick_name: &String) -> Result<Item>;
    fn set_profile(&self, id: &i64, real_name: &String, logo: &String) -> Result<()>;
    fn sign_in(&self, id: &i64, ip: &Option<String>) -> Result<()>;
    fn sign_up<T: Encryptor>(
        &self,
        real_name: &String,
        nick_name: &String,
        email: &String,
        password: &String,
    ) -> Result<()>;
    fn lock(&self, id: &i64, on: bool) -> Result<()>;
    fn confirm(&self, id: &i64) -> Result<()>;
    fn unlock(&self, id: &i64) -> Result<()>;
    fn count(&self) -> Result<i64>;
    fn all(&self) -> Result<Vec<Item>>;
    fn password<T: Encryptor>(&self, id: &i64, password: &String) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: &i64) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first(self)?;
        Ok(it)
    }

    fn by_uid(&self, uid: &String) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::uid.eq(uid))
            .first(self)?;
        Ok(it)
    }

    fn by_email(&self, email: &String) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::email.eq(&email.trim().to_lowercase()))
            .first(self)?;
        Ok(it)
    }

    fn by_nick_name(&self, nick_name: &String) -> Result<Item> {
        let it = users::dsl::users
            .filter(users::dsl::nick_name.eq(nick_name.trim()))
            .first(self)?;
        Ok(it)
    }

    fn sign_in(&self, id: &i64, ip: &Option<String>) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        let (current_sign_in_at, current_sign_in_ip, sign_in_count) = users::dsl::users
            .select((
                users::dsl::current_sign_in_at,
                users::dsl::current_sign_in_ip,
                users::dsl::sign_in_count,
            ))
            .filter(users::dsl::id.eq(id))
            .first::<(Option<NaiveDateTime>, Option<String>, i64)>(self)?;
        update(it)
            .set((
                users::dsl::current_sign_in_at.eq(&now),
                users::dsl::current_sign_in_ip.eq(ip),
                users::dsl::last_sign_in_at.eq(&current_sign_in_at),
                users::dsl::last_sign_in_ip.eq(&current_sign_in_ip),
                users::dsl::sign_in_count.eq(&(sign_in_count + 1)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn sign_up<T: Encryptor>(
        &self,
        real_name: &String,
        nick_name: &String,
        email: &String,
        password: &String,
    ) -> Result<()> {
        let email = email.trim().to_lowercase();
        let nick_name = nick_name.trim();
        insert_into(users::dsl::users)
            .values(&New {
                real_name: real_name,
                nick_name: nick_name,
                email: &email,
                password: Some(&T::sum(password.as_bytes())?),
                provider_type: &Type::Email.to_string(),
                provider_id: &email,
                logo: &format!(
                    "https://www.gravatar.com/avatar/{}.jpg",
                    gravatar_hash(&email)
                ),
                uid: &Uuid::new_v4().to_string(),
                updated_at: &Utc::now().naive_utc(),
            })
            .execute(self)?;
        Ok(())
    }

    fn lock(&self, id: &i64, on: bool) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&if on { Some(now) } else { None }),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn set_profile(&self, id: &i64, real_name: &String, logo: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(users::dsl::users.filter(users::dsl::id.eq(id)))
            .set((
                users::dsl::real_name.eq(real_name),
                users::dsl::logo.eq(logo),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn confirm(&self, id: &i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::confirmed_at.eq(&Some(now)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn unlock(&self, id: &i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::locked_at.eq(&None::<NaiveDateTime>),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn count(&self) -> Result<i64> {
        let cnt: i64 = users::dsl::users.count().get_result(self)?;
        Ok(cnt)
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = users::dsl::users
            .order(users::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn password<T: Encryptor>(&self, id: &i64, password: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        let password = T::sum(password.as_bytes())?;
        let it = users::dsl::users.filter(users::dsl::id.eq(id));
        update(it)
            .set((
                users::dsl::password.eq(&Some(password)),
                users::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
}

// https://en.gravatar.com/site/implement/hash/
pub fn gravatar_hash<S: AsRef<str>>(email: &S) -> String {
    format!(
        "{:x}",
        md5::compute(email.as_ref().to_lowercase().trim().as_bytes())
    )
}
