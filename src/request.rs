use std::fmt;
use std::pin::Pin;

use actix_web::{
    dev::Payload,
    error::{Error, ErrorBadRequest, ErrorUnauthorized},
    http::header::{HeaderMap, LanguageTag, ACCEPT_LANGUAGE, AUTHORIZATION},
    FromRequest, HttpMessage, HttpRequest,
};
use futures::future::Future;

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
