pub mod status;

use std::ops::Deref;
use std::sync::Arc;

use rocket::State;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    cache::Cache,
    cache::Provider,
    crypto::Crypto,
    errors::JsonResult,
    orm::Database,
    queue::{Queue, Task},
    settings::Dao as SettingDao,
};
use super::super::tasks::send_email::{
    Config as Smtp, Task as SendEmailTask, NAME as SendEmailQueueName,
};
use super::users::Administrator;

#[derive(Validate, Serialize, Deserialize)]
pub struct Author {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(email, length(min = "1"))]
    pub email: String,
}

impl Author {
    const KEY: &'static str = "site.author";
}

impl Default for Author {
    fn default() -> Self {
        Self {
            name: "who-am-i".to_string(),
            email: "change-me@gmail.com".to_string(),
        }
    }
}

#[get("/site/author")]
pub fn get_author(
    db: Database,
    _user: Administrator,
    enc: State<Arc<Crypto>>,
) -> JsonResult<Author> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let it: Author = match SettingDao::get(db, enc, &Author::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Author::default(),
    };
    Ok(Json(it))
}

#[post("/site/author", data = "<form>")]
pub fn set_author(
    db: Database,
    _user: Administrator,
    form: Json<Author>,
    enc: State<Arc<Crypto>>,
) -> JsonResult<()> {
    form.validate()?;
    let form = form.deref();
    let db = db.deref();
    let enc = enc.deref();

    SettingDao::set::<String, Author, Crypto>(db, enc, &Author::KEY.to_string(), form, false)?;
    Ok(Json(()))
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Seo {
    pub google: Option<Google>,
    pub baidu: Option<Baidu>,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Google {
    #[validate(length(min = "1"))]
    pub verify_id: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Baidu {
    #[validate(length(min = "1"))]
    pub verify_id: String,
}

impl Seo {
    const KEY: &'static str = "site.seo";
}

impl Default for Seo {
    fn default() -> Self {
        Self {
            google: None,
            baidu: None,
        }
    }
}

#[get("/site/seo")]
pub fn get_seo(db: Database, _user: Administrator, enc: State<Arc<Crypto>>) -> JsonResult<Seo> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();

    let it: Seo = match SettingDao::get(db, enc, &Seo::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Seo::default(),
    };
    Ok(Json(it))
}

#[post("/site/seo", data = "<form>")]
pub fn set_seo(
    db: Database,
    _user: Administrator,
    form: Json<Seo>,
    enc: State<Arc<Crypto>>,
) -> JsonResult<()> {
    form.validate()?;
    let form = form.deref();
    let db = db.deref();
    let enc = enc.deref();
    SettingDao::set::<String, Seo, Crypto>(db, enc, &Seo::KEY.to_string(), form, false)?;

    Ok(Json(()))
}

#[get("/site/smtp")]
pub fn get_smtp(db: Database, _user: Administrator, enc: State<Arc<Crypto>>) -> JsonResult<Smtp> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();

    let it: Smtp = match SettingDao::get(db, enc, &Smtp::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Smtp::default(),
    };
    Ok(Json(it))
}

#[post("/site/smtp", data = "<form>")]
pub fn set_smtp(
    db: Database,
    _user: Administrator,
    form: Json<Smtp>,
    enc: State<Arc<Crypto>>,
) -> JsonResult<()> {
    form.validate()?;
    let form = form.deref();
    let db = db.deref();
    let enc = enc.deref();
    SettingDao::set::<String, Smtp, Crypto>(db, enc, &Smtp::KEY.to_string(), &form, true)?;

    Ok(Json(()))
}

#[patch("/site/smtp")]
pub fn test_smtp(user: Administrator, queue: State<Arc<Box<dyn Queue>>>) -> JsonResult<()> {
    let queue = queue.deref();
    let user = user.0;
    queue.publish(
        SendEmailQueueName.to_string(),
        Task::new(&SendEmailTask {
            email: user.email.clone(),
            name: user.real_name.clone(),
            subject: format!("Hi, {}", user.real_name),
            body: "This is a test email.".to_string(),
        })?,
    )?;
    Ok(Json(()))
}

#[delete("/site/cache")]
pub fn clear_cache(_user: Administrator, cache: Cache) -> JsonResult<()> {
    cache.clear()?;
    Ok(Json(()))
}
