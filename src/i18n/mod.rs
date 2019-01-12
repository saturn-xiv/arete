pub mod locale;

use std::net::IpAddr;
use std::ops::Deref;
use std::time::Duration;

use mustache;
use serde::ser::Serialize;

use super::{
    cache::Cache,
    errors::{Error, Result},
    orm::PooledConnection as DbConnection,
    redis::PooledConnection as RedisConnection,
};

use self::locale::Dao;

pub struct I18n {
    pub db: DbConnection,
    pub cache: RedisConnection,
    pub locale: String,
    pub ip: IpAddr,
}

impl I18n {
    pub fn languages(&self) -> Result<Vec<String>> {
        self.cache.get(
            &"languages".to_string(),
            Duration::from_secs(60 * 60 * 24 * 7),
            || -> Result<Vec<String>> { self.db.deref().languages() },
        )
    }

    pub fn exist(&self, lang: &String) -> bool {
        if let Ok(items) = self.languages() {
            return items.contains(lang);
        }
        false
    }

    fn get(&self, lang: &String, code: &String) -> Result<Option<String>> {
        self.cache.get(
            &format!("locales.{}.{}", lang, code),
            Duration::from_secs(60 * 60 * 24 * 7),
            || -> Result<Option<String>> {
                if let Ok(it) = self.db.deref().by_lang_and_code(lang, code) {
                    return Ok(Some(it.message));
                }
                Ok(None)
            },
        )
    }

    pub fn tr<S: Serialize>(&self, code: &String, args: &Option<S>) -> Result<Option<String>> {
        match self.get(&self.locale, &code)? {
            Some(msg) => match args {
                Some(args) => Ok(Some(mustache::compile_str(&msg)?.render_to_string(args)?)),
                None => Ok(Some(msg)),
            },
            None => Ok(None),
        }
    }

    pub fn e<C: Into<String>, S: Serialize>(&self, code: C, args: &Option<S>) -> Error {
        let code = code.into();
        match self.tr(&code, args) {
            Ok(msg) => match msg {
                Some(msg) => msg.into(),
                None => format!("{}.{}", self.locale, code).into(),
            },
            Err(e) => e,
        }
    }

    pub fn t<C: Into<String>, S: Serialize>(&self, code: C, args: &Option<S>) -> String {
        let code = code.into();
        if let Ok(Some(msg)) = self.tr(&code, args) {
            return msg;
        }
        format!("{}.{}", self.locale, code)
    }
}
