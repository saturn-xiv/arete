use cookie::Cookie;
use hyper::{
    header::{ACCEPT_LANGUAGE, AUTHORIZATION, COOKIE},
    Request,
};
use language_tags::LanguageTag;
use url::Url;

pub trait FromRequest: Sized {
    fn from_request<S>(req: &Request<S>) -> Option<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Home {
    pub host: String,
    pub schema: String,
    pub port: Option<u16>,
}

impl FromRequest for Home {
    fn from_request<S>(req: &Request<S>) -> Option<Self> {
        let u = req.uri();
        if let Some(s) = u.scheme_str() {
            if let Some(h) = u.host() {
                return Some(Self {
                    host: h.to_string(),
                    schema: s.to_string(),
                    port: u.port_u16(),
                });
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token(pub String);

impl FromRequest for Token {
    fn from_request<S>(req: &Request<S>) -> Option<Self> {
        let headers = req.headers();
        if let Some(it) = headers.get(AUTHORIZATION) {
            if let Ok(it) = it.to_str() {
                let bearer = "Bearer ";
                if it.starts_with(bearer) {
                    return Some(Self(it[bearer.len()..].to_string()));
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locale(pub String);

impl FromRequest for Locale {
    fn from_request<S>(req: &Request<S>) -> Option<Self> {
        let key = "locale";

        // 1. Check URL arguments.
        if let Ok(it) = Url::parse(&req.uri().to_string()) {
            if let Some((_, v)) = it.query_pairs().find(|(k, _)| k == key) {
                return Some(Self(v.to_string()));
            }
        }

        let headers = req.headers();

        // 2. Get language information from cookies.
        if let Some(it) = headers.get(COOKIE) {
            if let Ok(it) = it.to_str() {
                for it in it.split(";") {
                    if let Ok(it) = Cookie::parse(it) {
                        if it.name() == key {
                            return Some(Self(it.value().to_string()));
                        }
                    }
                }
            }
        }

        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4
        if let Some(it) = headers.get(ACCEPT_LANGUAGE) {
            if let Ok(it) = it.to_str() {
                if let Ok(it) = it.parse::<LanguageTag>() {
                    if let Some(it) = it.language {
                        return Some(Self(it));
                    }
                }
            }
        }
        None
    }
}
