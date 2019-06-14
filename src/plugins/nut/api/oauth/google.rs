use std::ops::Deref;
use std::sync::Arc;

use chrono::Duration;
use diesel::Connection as DieselConnection;
use failure::Error as FailueError;
use rocket::State;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{
    crypto::Crypto,
    errors::JsonResult,
    i18n::I18n,
    jwt::Jwt,
    oauth::google::{openid::IdToken, ClientSecret, Scope},
    orm::Database,
    request::{ClientIp, Locale},
    settings::Dao as SettingDao,
};
use super::super::super::models::{log::Dao as LogDao, user::Dao as UserDao};
use super::super::users::{Action, Token};

#[get("/oauth/google/sign-in?<callback>")]
pub fn get_sign_in(enc: State<Arc<Crypto>>, callback: String, db: Database) -> JsonResult<String> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();

    let it: ClientSecret = SettingDao::get(db, enc, &ClientSecret::KEY.to_string())?;
    let (url, _, _) = it
        .web
        .oauth2(vec![Scope::Openid, Scope::Email, Scope::Profile], &callback);

    Ok(Json(url))
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    #[validate(length(min = "1"))]
    pub code: String,
    #[validate(length(min = "1"))]
    pub callback: String,
}

#[post("/oauth/google/sign-in", data = "<form>")]
pub fn post_sign_in(
    db: Database,
    enc: State<Arc<Crypto>>,
    form: Json<SignIn>,
    lang: Locale,
    remote: ClientIp,
    jwt: State<Arc<Jwt>>,
) -> JsonResult<String> {
    form.validate()?;
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let jwt = jwt.deref();

    let it: ClientSecret = SettingDao::get(db, enc, &ClientSecret::KEY.to_string())?;
    let access = it
        .web
        .exchange_authorization_code(&form.callback, &form.code)?;
    let info: IdToken = it.web.get(&access.token_info(), &access.access_token)?;

    let uid = db.transaction::<_, FailueError, _>(move || {
        let user = UserDao::google(db, &access.access_token, &info, &remote.0)?;
        __i18n_l!(
            db,
            user.id,
            &remote.0,
            &lang.0,
            "nut.logs.user.sign-in.success"
        )?;
        Ok(user.uid)
    })?;

    let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
    let token = jwt.sum(
        None,
        &Token {
            uid: uid,
            act: Action::SignIn,
            nbf: nbf,
            exp: exp,
        },
    )?;
    Ok(Json(token))
}
