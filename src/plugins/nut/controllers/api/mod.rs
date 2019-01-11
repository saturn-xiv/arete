pub mod admin;
pub mod leave_words;
pub mod locales;
pub mod ueditor;
pub mod users;

use std::net::SocketAddr;
use std::ops::Deref;

use diesel::Connection as DieselConnection;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

use super::super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    env,
    errors::{Error, Result},
    i18n::locale::Dao as LocaleDao,
    orm::Database,
};
use super::super::models::{
    log::Dao as LogDao,
    policy::{Dao as PolicyDao, Item as Policy, Role},
    user::Dao as UserDao,
};

#[get("/about")]
pub fn about(db: Database) -> Result<JsonValue> {
    let db = db.deref();
    let languages = LocaleDao::languages(db)?;
    Ok(json!({
        "languages": languages,
        "version": env::version(),
    }))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Install {
    #[validate(length(min = "1", max = "32"))]
    pub real_name: String,
    #[validate(email, length(min = "2", max = "64"))]
    pub email: String,
    #[validate(length(min = "6", max = "32"))]
    pub password: String,
}

#[post("/install", format = "json", data = "<form>")]
pub fn install(form: Json<Install>, db: Database, remote: SocketAddr) -> Result<JsonValue> {
    form.validate()?;
    let db = db.deref();
    let ip = remote.ip();

    db.transaction::<_, Error, _>(|| {
        if UserDao::count(db)? > 0 {
            return Err("Database isn't empty".into());
        }
        UserDao::sign_up::<Sodium>(
            db,
            &form.real_name,
            &"admin".to_string(),
            &form.email,
            &form.password,
        )?;
        let it = UserDao::by_email(db, &form.email)?;
        UserDao::confirm(db, &it.id)?;
        let (nbf, exp) = Policy::weeks(1 << 12);
        PolicyDao::apply(db, &it.id, &Role::Root, &None::<String>, &nbf, &exp)?;
        PolicyDao::apply(db, &it.id, &Role::Admin, &None::<String>, &nbf, &exp)?;

        LogDao::add(db, &it.id, &ip, "Init database, apply as administrator")?;
        Ok(())
    })?;

    Ok(json!({}))
}
