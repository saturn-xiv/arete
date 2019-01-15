use std::ops::Deref;
use std::sync::Arc;

use rocket::State;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::super::{
    crypto::sodium::Encryptor as Sodium, errors::Result, orm::Database, settings::Dao as SettingDao,
};
use super::super::super::super::super::request::Administrator;

#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(email, length(min = "1"))]
    pub email: String,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            name: "who-am-i".to_string(),
            email: "change-me@gmail.com".to_string(),
        }
    }
}
const KEY: &'static str = "site.author";

#[get("/admin/site/author")]
pub fn get(_user: Administrator, db: Database, enc: State<Arc<Sodium>>) -> Result<Json<Form>> {
    let db = db.deref();
    let enc = enc.deref().deref();

    let it: Form = match SettingDao::get(db, enc, &KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Form::default(),
    };
    Ok(Json(it))
}

#[post("/admin/site/author", format = "json", data = "<form>")]
pub fn post(
    _user: Administrator,
    db: Database,
    enc: State<Arc<Sodium>>,
    form: Json<Form>,
) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    let enc = enc.deref().deref();
    let form = form.deref();
    SettingDao::set::<String, Form, Sodium>(db, enc, &KEY.to_string(), &form, false)?;
    Ok(Json(()))
}
