use std::fmt;

use actix_web::{
    error::{ErrorBadRequest, ErrorUnauthorized},
    http::header::{LanguageTag, ACCEPT_LANGUAGE, AUTHORIZATION},
    Error, FromRequest, HttpRequest, Result,
};

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For
pub struct ClientIp(pub String);

impl fmt::Display for ClientIp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<S> FromRequest<S> for ClientIp {
    type Config = ();
    type Result = Self;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        let headers = req.headers();
        if let Some(it) = headers.get("X-Forwarded-For") {
            if let Ok(it) = it.to_str() {
                if let Some(it) = it.split(",").next() {
                    let it = it.trim();
                    if !it.is_empty() {
                        return Self(it.to_string());
                    }
                }
            }
        }
        if let Some(it) = headers.get("X-Real-Ip") {
            if let Ok(it) = it.to_str() {
                if !it.is_empty() {
                    return Self(it.to_string());
                }
            }
        }
        if let Some(it) = headers.get("X-Appengine-Remote-Addr") {
            if let Ok(it) = it.to_str() {
                if !it.is_empty() {
                    return Self(it.to_string());
                }
            }
        }

        Self(req.connection_info().host().to_string())
    }
}

pub struct Home(pub String);

impl Home {
    pub const KEY: &'static str = "home";
}

impl fmt::Display for Home {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<S> FromRequest<S> for Home {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        Ok(Self(req.url_for_static(Self::KEY)?.to_string()))
    }
}

pub struct Token(pub String);

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<S> FromRequest<S> for Token {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        let headers = req.headers();
        if let Some(it) = headers.get(AUTHORIZATION) {
            if let Ok(it) = it.to_str() {
                let bearer = "Bearer ";
                if it.starts_with(bearer) {
                    return Ok(Self(it[bearer.len()..].to_string()));
                }
            }
        }
        Err(ErrorUnauthorized("bad token"))
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
impl<S> FromRequest<S> for Locale {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        let key = "locale";

        // 1. Check URL arguments.
        if let Some(it) = req.query().get(key) {
            return Ok(Self(it.to_string()));
        }

        // 2. Get language information from cookies.
        if let Some(it) = req.cookie(key) {
            return Ok(Self(it.value().to_string()));
        }

        let headers = req.headers();
        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(it) = headers.get(ACCEPT_LANGUAGE) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = it.parse::<LanguageTag>() {
                    if let Some(it) = it.language {
                        return Ok(Self(it));
                    }
                }
            }
        }
        Err(ErrorBadRequest("can't find language tag"))
    }
}
