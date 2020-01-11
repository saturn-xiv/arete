use std::fmt;
use std::fmt::Debug;
use std::pin::Pin;

use actix_web::{
    dev::Payload,
    error::{Error, ErrorBadRequest, ErrorUnauthorized},
    http::header::{HeaderMap, LanguageTag, ACCEPT_LANGUAGE, AUTHORIZATION},
    FromRequest, HttpMessage, HttpRequest,
};
use futures::future::Future;
use serde::ser::Serialize;

#[derive(Serialize)]
pub struct Pagination<T> {
    pub size: i64,
    pub page: i64,
    pub total: i64,
    pub items: Vec<T>,
}

impl<T: Serialize> Pagination<T> {
    pub fn new(total: i64, size: i64, page: i64, items: Vec<T>) -> Self {
        let size = if size < Pager::MIN_SIZE {
            Pager::MAX_SIZE
        } else {
            size
        };

        let pages = total / size;
        let pages = if pages % size == 0 { pages } else { pages + 1 };
        let pages = if pages < 1 { 1 } else { pages };

        let page = if page < 1 {
            1
        } else if page * size >= total {
            pages
        } else {
            page
        };
        Self {
            size: size,
            page: page,
            total: pages,
            items: items,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pager {
    pub size: i64,
    pub page: i64,
}

impl Pager {
    pub const MIN_SIZE: i64 = 5;
    pub const MAX_SIZE: i64 = 120;
    pub fn offset(&self, total: i64) -> i64 {
        let v = (self.page - 1) * self.size;
        if v < 0 {
            return 0;
        }
        if v >= total {
            return total - self.size;
        }
        v
    }

    pub fn limit(&self) -> i64 {
        let v = self.size;
        if v < Self::MIN_SIZE {
            Self::MAX_SIZE
        } else {
            v
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Locale(pub String);

impl Locale {
    fn parse(req: &HttpRequest) -> Option<String> {
        let key = "locale";

        // 1. Check URL arguments.
        let it = req.match_info().query(key);
        if !it.is_empty() {
            return Some(it.to_string());
        }

        // 2. Get language information from cookies.
        if let Some(it) = req.cookie(key) {
            let it = it.value();
            if !it.is_empty() {
                return Some(it.to_string());
            }
        }

        let headers = req.headers();

        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(it) = headers.get(ACCEPT_LANGUAGE) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = it.parse::<LanguageTag>() {
                    if let Some(it) = it.language {
                        if !it.is_empty() {
                            return Some(it);
                        }
                    }
                }
            }
        }

        None
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl FromRequest for Locale {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let it = Self::parse(req);

        Box::pin(async move {
            Ok(Self(match it {
                Some(it) => it,
                None => "en-US".to_string(),
            }))
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientIp(pub String);

impl ClientIp {
    fn parse(headers: &HeaderMap) -> Option<String> {
        if let Some(it) = headers.get("X-Forwarded-For") {
            if let Ok(it) = it.to_str() {
                let items: Vec<&str> = it.split(',').collect();
                if let Some(it) = items.first() {
                    let it = it.trim();
                    if !it.is_empty() {
                        return Some(it.to_string());
                    }
                }
            }
        }
        if let Some(it) = headers.get("X-Real-Ip") {
            if let Ok(it) = it.to_str() {
                let it = it.trim();
                if !it.is_empty() {
                    return Some(it.to_string());
                }
            }
        }
        if let Some(it) = headers.get("X-Appengine-Remote-Addr") {
            if let Ok(it) = it.to_str() {
                let it = it.trim();
                if !it.is_empty() {
                    return Some(it.to_string());
                }
            }
        }
        None
    }
}

impl fmt::Display for ClientIp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl FromRequest for ClientIp {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let it = Self::parse(req.headers());

        Box::pin(async move {
            match it {
                Some(it) => Ok(Self(it)),
                None => Err(ErrorBadRequest("cant't detect client ip")),
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token(pub String);

impl Token {
    fn parse(headers: &HeaderMap) -> Option<String> {
        let bearer = "Bearer ";
        if let Some(it) = headers.get(AUTHORIZATION) {
            if let Ok(it) = it.to_str() {
                if it.starts_with(bearer) {
                    return Some(it[bearer.len()..].to_string());
                }
            }
        }
        None
    }
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl FromRequest for Token {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let it = Self::parse(req.headers());

        Box::pin(async move {
            match it {
                Some(it) => Ok(Self(it)),
                None => Err(ErrorUnauthorized("cant't detect auth token")),
            }
        })
    }
}
