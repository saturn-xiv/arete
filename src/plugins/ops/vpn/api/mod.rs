pub mod logs;
pub mod settings;
pub mod users;

use std::ops::Deref;
use std::sync::Arc;

use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};
use uuid::Uuid;

use super::super::super::super::{
    crypto::{Crypto, Secret},
    errors::Result,
    orm::{Connection, Database},
    request::Token as Bearer,
    settings::Dao as SettingDao,
};

pub struct Token;

impl Token {
    pub const KEY: &'static str = "vpn.token";

    pub fn new<S: Secret>(db: &Connection, enc: &S) -> Result<String> {
        let token: String = match SettingDao::get(db, enc, &Token::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => {
                let v = Uuid::new_v4().to_string();
                SettingDao::set(db, enc, &Token::KEY.to_string(), &v, true)?;
                v
            }
        };
        Ok(token)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let Bearer(token) = req.guard::<Bearer>()?;
        let Database(db) = req.guard::<Database>()?;
        let db = db.deref();
        let enc = req.guard::<State<Arc<Crypto>>>()?;
        let enc = enc.deref();
        let enc = enc.deref();

        if let Ok(val) = SettingDao::get::<String, String, Crypto>(db, enc, &Token::KEY.to_string())
        {
            if val == token {
                return Outcome::Success(Token);
            }
        }

        Outcome::Failure((Status::NonAuthoritativeInformation, ()))
    }
}
