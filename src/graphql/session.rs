use std::ops::Deref;

use hyper::Request;

use super::super::{
    i18n::I18n,
    orm::Connection,
    request::{FromRequest, Locale, Token},
};
use super::context::Context;

pub struct Session {
    pub user: Option<i64>,
    pub lang: String,
}

impl Session {
    const DEFAULT_LANG: &'static str = "en-US";
    pub fn new<S>(ctx: &Context, req: &Request<S>) -> Self {
        if let Ok(db) = ctx.db() {
            let db = db.deref();
            let lang = Self::locale(db, req);
            let user = match Token::from_request(req) {
                Some(_) => Some(0),
                None => None,
            };
            return Self {
                user: user,
                lang: lang,
            };
        }

        Self {
            user: None,
            lang: Self::DEFAULT_LANG.to_string(),
        }
    }

    fn locale<S>(db: &Connection, req: &Request<S>) -> String {
        if let Some(it) = Locale::from_request(req) {
            if I18n::exist(db, &it.0) {
                return it.0;
            }
        }
        Self::DEFAULT_LANG.to_string()
    }
}
