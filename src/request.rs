use std::fmt;
use std::ops::Deref;

use hyper::header::Header;
use language_tags::LanguageTag;
use rocket::{
    http::{
        hyper::header::{AcceptLanguage, Authorization, Bearer},
        RawStr, Status,
    },
    request::{self, FromRequest},
    Outcome, Request, State,
};

use super::{i18n::I18n, orm::Database};

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For
//     pub const X_FORWARDED_FOR: &'static str = "X-Forwarded-For";
//     pub const X_REAL_IP: &'static str = "X-Real-Ip";
//     pub const X_APPENGINE_REMOTE_ADDR: &'static str = "X-Appengine-Remote-Addr";

pub struct Token(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth) = req
            .headers()
            .get_one(Authorization::<Bearer>::header_name())
        {
            if let Ok(auth) = auth.parse::<Bearer>() {
                let header = "Bearer ";
                return Outcome::Success(Token(auth.token[header.len()..].to_string()));
            }
        }
        Outcome::Failure((Status::NonAuthoritativeInformation, ()))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

pub struct Locale(pub String);

impl fmt::Display for Locale {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self("en-US".to_string())
    }
}

impl<'a, 'r> Locale {
    fn detect(req: &'a Request<'r>) -> Option<String> {
        let key = "locale";

        // 1. Check URL arguments.
        if let Some(it) = req.get_query_value::<&'a RawStr>(key) {
            if let Ok(it) = it {
                return Some(it.to_string());
            }
        }

        // 2. Get language information from cookies.
        if let Some(it) = req.cookies().get(key) {
            return Some(it.value().to_string());
        }

        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(it) = req.headers().get_one(AcceptLanguage::header_name()) {
            if let Ok(it) = it.parse::<LanguageTag>() {
                if let Some(it) = it.language {
                    return Some(it);
                }
            }
        }

        None
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(it) = Self::detect(req) {
            let Database(db) = req.guard::<Database>()?;
            let db = db.deref();
            if db.exist(&it) {
                return Outcome::Success(Locale(it));
            }
        }
        return Outcome::Success(Locale::default());
    }
}
