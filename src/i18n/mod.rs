pub mod locale;

use std::net::IpAddr;
use std::net::SocketAddr;
use std::ops::Deref;
use std::time::Duration;

use hyper::header::Header as HyperHeader;
use mustache;
use rocket::{
    http::{hyper::header::AcceptLanguage, Cookies},
    request::{self, FromRequest},
    Outcome, Request,
};
use serde::ser::Serialize;

use super::{
    cache::Cache,
    errors::{Error, Result},
    orm::{Database, PooledConnection as DbConnection},
    redis::{PooledConnection as RedisConnection, Redis},
};

use self::locale::Dao;

pub struct I18n {
    pub db: DbConnection,
    pub cache: RedisConnection,
    pub locale: String,
    pub ip: IpAddr,
}

const TTL: u64 = 60 * 60 * 24 * 7;

impl I18n {
    pub fn languages(&self) -> Result<Vec<String>> {
        self.cache.get(
            &"languages".to_string(),
            Duration::from_secs(TTL),
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
            Duration::from_secs(TTL),
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

impl I18n {
    fn parse(&self, req: &Request) -> Option<String> {
        let key = "locale";
        // 1. Check URL arguments.
        // 2. Get language information from cookies.
        if let Outcome::Success(cookies) = req.guard::<Cookies>() {
            if let Some(it) = cookies.get(key) {
                return Some(it.value().to_string());
            }
        }
        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4

        if let Ok(AcceptLanguage(al)) = AcceptLanguage::parse_header(
            &req.headers()
                .get(AcceptLanguage::header_name())
                .map(|x| x.as_bytes().to_vec())
                .collect::<Vec<Vec<u8>>>(),
        ) {
            for it in al {
                if let Some(lng) = it.item.language {
                    return Some(lng);
                }
            }
        }
        None
    }
    fn detect(&mut self, req: &Request) {
        if let Some(lang) = self.parse(req) {
            if self.exist(&lang) {
                self.locale = lang;
            }
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for I18n {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let remote = req.guard::<SocketAddr>().unwrap();
        let Database(db) = req.guard::<Database>()?;
        let Redis(cache) = req.guard::<Redis>()?;
        let mut it = I18n {
            db: db,
            cache: cache,
            locale: "en-US".to_string(),
            ip: remote.ip(),
        };
        it.detect(req);
        Outcome::Success(it)
    }
}
