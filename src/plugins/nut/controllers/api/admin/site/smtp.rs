use std::ops::Deref;
use std::sync::Arc;

use rocket::State;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::super::{
    crypto::sodium::Encryptor as Sodium, errors::JsonResult, orm::Database,
    settings::Dao as SettingDao,
};
use super::super::super::super::super::request::Administrator;


impl Default for Form {
    fn default() -> Self {
        Self {
            host: "smtp.gmail.com".to_string(),
            email: "change-me@gmail.com".to_string(),
            password: "".to_string(),
        }
    }
}

const KEY: &'static str = "site.smtp";

#[get("/admin/site/smtp")]
pub fn get(_user: Administrator, db: Database, enc: State<Arc<Sodium>>) -> JsonResult<Form> {
    let db = db.deref();
    let enc = enc.deref().deref();

    let mut it: Form = match SettingDao::get(db, enc, &KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Form::default(),
    };
    it.password = "".to_string();
    Ok(Json(it))
}

#[post("/admin/site/smtp", format = "json", data = "<form>")]
pub fn post(
    _user: Administrator,
    db: Database,
    enc: State<Arc<Sodium>>,
    form: Json<Form>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let enc = enc.deref().deref();
    let form = form.deref();
    SettingDao::set::<String, Form, Sodium>(db, enc, &KEY.to_string(), &form, true)?;
    Ok(Json(()))
}
