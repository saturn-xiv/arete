use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::{
    errors::Result,
    orm::{schema::locales, Connection},
};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i64,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "locales"]
pub struct New<'a> {
    pub lang: &'a str,
    pub code: &'a str,
    pub message: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn languages(&self) -> Result<Vec<String>>;
    fn count(&self, lang: &String) -> Result<i64>;
    fn all(&self, lang: &String) -> Result<Vec<Item>>;
    fn get(&self, lang: &String, code: &String) -> Result<String>;
    fn set(&self, lang: &String, code: &String, message: &String) -> Result<()>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl Dao for Connection {
    fn languages(&self) -> Result<Vec<String>> {
        Ok(locales::dsl::locales
            .select(locales::dsl::lang)
            .distinct()
            .load::<String>(self)?)
    }

    fn count(&self, lang: &String) -> Result<i64> {
        let cnt: i64 = locales::dsl::locales
            .count()
            .filter(locales::dsl::lang.eq(lang))
            .get_result(self)?;
        Ok(cnt)
    }

    fn all(&self, lang: &String) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .order(locales::dsl::code.asc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn get(&self, lang: &String, code: &String) -> Result<String> {
        let it = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<Item>(self)?;
        Ok(it.message)
    }
    fn set(&self, lang: &String, code: &String, message: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        match locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<Item>(self)
        {
            Ok(it) => {
                let it = locales::dsl::locales.filter(locales::dsl::id.eq(&it.id));
                update(it)
                    .set((
                        locales::dsl::message.eq(message),
                        locales::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(locales::dsl::locales)
                    .values(&New {
                        lang: lang,
                        code: code,
                        message: message,
                        updated_at: &now,
                    })
                    .execute(self)?;
            }
        }
        Ok(())
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(locales::dsl::locales.filter(locales::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
