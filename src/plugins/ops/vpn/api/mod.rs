pub mod logs;
pub mod users;

use std::ops::Deref;
use std::sync::Arc;

use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};
use rocket_contrib::json::Json;
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto, errors::JsonResult, orm::Database, request::Token as Bearer,
    settings::Dao as SettingDao,
};
use super::super::super::nut::api::users::Administrator;

#[get("/token")]
pub fn token(db: Database, enc: State<Arc<Crypto>>, _user: Administrator) -> JsonResult<String> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let it: String = match SettingDao::get(db, enc, &Token::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => {
            let v = Uuid::new_v4().to_string();
            SettingDao::set::<String, String, Crypto>(db, enc, &Token::KEY.to_string(), &v, true)?;
            v
        }
    };
    Ok(Json(it))
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1", max = "255"))]
    pub host: String,
    pub port: u16,
    #[validate(length(min = "1", max = "16"))]
    pub lan: String,
    #[validate(length(min = "1", max = "32"))]
    pub dns1: String,
    #[validate(length(min = "1", max = "32"))]
    pub dns2: String,
}

impl Form {
    const KEY: &'static str = "site.author";
}

impl Default for Form {
    fn default() -> Self {
        Self {
            host: "vpn.change-me.com".to_string(),
            port: 1194,
            lan: "192.168.0.0".to_string(),
            dns1: "8.8.8.8".to_string(),
            dns2: "8.8.4.4".to_string(),
        }
    }
}

#[get("/")]
pub fn get(db: Database, _user: Administrator, enc: State<Arc<Crypto>>) -> JsonResult<Form> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let it: Form = match SettingDao::get(db, enc, &Form::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Form::default(),
    };
    Ok(Json(it))
}

#[post("/", data = "<form>")]
pub fn post(
    _user: Administrator,
    enc: State<Arc<Crypto>>,
    db: Database,
    form: Json<Form>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let form = form.deref();
    SettingDao::set::<String, Form, Crypto>(db, enc, &Form::KEY.to_string(), form, false)?;
    Ok(Json(()))
}

#[get("/download")]
pub fn download(_user: Administrator, enc: State<Arc<Crypto>>, db: Database) -> JsonResult<()> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let it: Form = SettingDao::get(db, enc, &Form::KEY.to_string())?;
    // TODO
    Ok(Json(()))
}

pub struct Token;

impl Token {
    pub const KEY: &'static str = "vpn.token";
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
