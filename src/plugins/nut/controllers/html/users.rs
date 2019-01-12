use std::ops::Deref;
use std::sync::Arc;

use rocket::{response::Redirect, State};

use super::super::super::super::super::{errors::Result, i18n::I18n, jwt::Jwt, orm::Database};
use super::super::super::models::user::Dao as UserDao;
use super::super::api::users::{Action, Token};

const SIGN_IN_PATH: &'static str = "/my/users/sign-in";

#[get("/users/confirm/<token>")]
pub fn confirm_token(
    token: String,
    db: Database,
    i18n: I18n,
    jwt: State<Arc<Jwt>>,
) -> Result<Redirect> {
    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Confirm {
        return Err(i18n.e("flashes.bad-action", &None::<String>));
    }

    let db = db.deref();
    let it = UserDao::by_uid(db, &token.uid)?;
    if let Some(_) = it.confirmed_at {
        return Err(i18n.e("nut.errors.user.already-confirm", &None::<String>));
    }
    UserDao::confirm(db, &it.id)?;
    i18n.l(&it.id, "nut.logs.user.confirm", &None::<String>)?;
    Ok(Redirect::to(SIGN_IN_PATH))
}

#[get("/users/unlock/<token>")]
pub fn unlock_token(
    token: String,
    db: Database,
    i18n: I18n,
    jwt: State<Arc<Jwt>>,
) -> Result<Redirect> {
    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Unlock {
        return Err(i18n.e("flashes.bad-action", &None::<String>));
    }

    let db = db.deref();
    let it = UserDao::by_uid(db, &token.uid)?;
    if None == it.locked_at {
        return Err(i18n.e("nut.errors.user.is-not-lock", &None::<String>));
    }
    UserDao::unlock(db, &it.id)?;
    i18n.l(&it.id, "nut.logs.user.unlock", &None::<String>)?;
    Ok(Redirect::to(SIGN_IN_PATH))
}
