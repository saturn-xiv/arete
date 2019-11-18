pub mod attachments;
pub mod cards;
pub mod categories;
pub mod friend_links;
pub mod leave_words;
pub mod links;
pub mod locales;
pub mod oauth;
pub mod site;
pub mod tags;
pub mod users;
pub mod votes;

use std::net::SocketAddr;
use std::ops::Deref;

use diesel::prelude::*;
use failure::Error as FailureError;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::{
    crypto::Crypto, errors::JsonResult, i18n::I18n, orm::Database, request::Locale,
};
use super::models::{
    log::Dao as LogDao,
    policy::{Dao as PolicyDao, Item as Policy, Role},
    user::Dao as UserDao,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Install {
    #[validate(length(min = 1, max = 32))]
    pub real_name: String,
    #[validate(email, length(min = 2, max = 64))]
    pub email: String,
    #[validate(length(min = 6, max = 32))]
    pub password: String,
}

#[post("/install", data = "<form>")]
pub fn install(
    db: Database,
    lang: Locale,
    remote: SocketAddr,
    form: Json<Install>,
) -> JsonResult<()> {
    form.validate()?;
    let ip = remote.ip().to_string();
    let db = db.deref();

    db.transaction::<_, FailureError, _>(|| {
        if UserDao::count(db)? > 0 {
            return __i18n_e!(db, &lang.0, "nut.errors.database-is-not-empty");
        }
        UserDao::sign_up::<Crypto>(
            db,
            &form.real_name,
            &"admin".to_string(),
            &form.email,
            &form.password,
        )?;
        let it = UserDao::by_email(db, &form.email)?;
        UserDao::confirm(db, it.id)?;
        let (nbf, exp) = Policy::weeks(1 << 12);
        PolicyDao::apply(db, it.id, &Role::Root, &None::<String>, &nbf, &exp)?;
        PolicyDao::apply(db, it.id, &Role::Admin, &None::<String>, &nbf, &exp)?;
        __i18n_l!(db, it.id, &ip, &lang.0, "nut.logs.init-database")?;

        Ok(())
    })?;

    Ok(Json(()))
}
