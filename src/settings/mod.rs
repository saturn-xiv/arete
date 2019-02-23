pub mod schema;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use super::{
    crypto::Encryptor,
    errors::Result,
    orm::{migration::New as Migration, Connection},
};

use self::schema::settings;

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-settings",
        version: "20190101053042",
        up: include_str!("up.sql"),
        down: include_str!("down.sql"),
    };
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub key: String,
    pub value: Vec<u8>,
    pub salt: Option<Vec<u8>>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "settings"]
pub struct New<'a> {
    pub key: &'a str,
    pub value: &'a [u8],
    pub salt: Option<&'a [u8]>,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn get<K: Serialize, V: DeserializeOwned, E: Encryptor>(&self, e: &E, key: &K) -> Result<V>;
    fn set<K: Serialize, V: Serialize, E: Encryptor>(
        &self,
        e: &E,
        k: &K,
        v: &V,
        f: bool,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn get<K: Serialize, V: DeserializeOwned, E: Encryptor>(&self, e: &E, k: &K) -> Result<V> {
        let key = serde_json::to_string(k)?;
        let it = settings::dsl::settings
            .filter(settings::dsl::key.eq(&key))
            .first::<Item>(self)?;

        let val = match it.salt {
            Some(salt) => e.decrypt(&it.value, &salt)?,
            None => it.value,
        };
        Ok(serde_json::from_slice(val.as_slice())?)
    }

    fn set<K: Serialize, V: Serialize, E: Encryptor>(
        &self,
        e: &E,
        k: &K,
        v: &V,
        f: bool,
    ) -> Result<()> {
        let key = serde_json::to_string(k)?;
        let buf = serde_json::to_vec(v)?;

        let (val, salt) = if f {
            let (val, salt) = e.encrypt(&buf);
            (val, Some(salt))
        } else {
            (buf, None)
        };

        let now = Utc::now().naive_utc();

        match settings::dsl::settings
            .filter(settings::dsl::key.eq(&key))
            .first::<Item>(self)
        {
            Ok(it) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&it.id));
                update(it)
                    .set((
                        settings::dsl::value.eq(&val),
                        settings::dsl::salt.eq(&salt),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(settings::dsl::settings)
                    .values(&New {
                        key: &key,
                        value: &val,
                        salt: match salt {
                            Some(ref v) => Some(v),
                            None => None,
                        },
                        updated_at: &now,
                    })
                    .execute(self)?;
            }
        };
        Ok(())
    }
}
