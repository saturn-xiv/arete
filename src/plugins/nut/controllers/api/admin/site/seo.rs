use std::ops::Deref;
use std::sync::Arc;

use rocket::State;
use rocket_contrib::json::Json;

use super::super::super::super::super::super::super::{
    crypto::sodium::Encryptor as Sodium, errors::Result, orm::Database, settings::Dao as SettingDao,
};
use super::super::super::super::super::request::Administrator;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub google: Option<Google>,
    pub baidu: Option<Baidu>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Google {
    pub site_verify_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Baidu {
    pub site_verify_id: String,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            google: None,
            baidu: None,
        }
    }
}
const KEY: &'static str = "site.seo";

#[get("/admin/site/seo")]
pub fn get(_user: Administrator, db: Database, enc: State<Arc<Sodium>>) -> Result<Json<Form>> {
    let db = db.deref();
    let enc = enc.deref().deref();

    let it: Form = match SettingDao::get(db, enc, &KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Form::default(),
    };
    Ok(Json(it))
}

#[post("/admin/site/seo", format = "json", data = "<form>")]
pub fn post(
    _user: Administrator,
    db: Database,
    enc: State<Arc<Sodium>>,
    form: Json<Form>,
) -> Result<Json<()>> {
    let db = db.deref();
    let enc = enc.deref().deref();
    let form = form.deref();
    SettingDao::set::<String, Form, Sodium>(db, enc, &KEY.to_string(), &form, false)?;
    Ok(Json(()))
}
