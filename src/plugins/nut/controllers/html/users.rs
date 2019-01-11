use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;

use rocket::{response::Redirect, State};

use super::super::super::super::super::{errors::Result, jwt::Jwt, orm::Database};
use super::super::super::models::{log::Dao as LogDao, user::Dao as UserDao};
use super::super::api::users::{Action, Token};

const SIGN_IN_PATH: &'static str = "/my/users/sign-in";

#[get("/users/confirm/<token>")]
pub fn confirm_token(
    token: String,
    remote: SocketAddr,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> Result<Redirect> {
    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Confirm {
        return Err("bad action".into());
    }

    let db = db.deref();
    let ip = remote.ip();
    let it = UserDao::by_uid(db, &token.uid)?;
    if let Some(_) = it.confirmed_at {
        return Err("User already confirmed".into());
    }
    UserDao::confirm(db, &it.id)?;
    LogDao::add(db, &it.id, &ip, "Confirmed")?;
    Ok(Redirect::to(SIGN_IN_PATH))
}

#[get("/users/unlock/<token>")]
pub fn unlock_token(
    token: String,
    remote: SocketAddr,
    db: Database,
    jwt: State<Arc<Jwt>>,
) -> Result<Redirect> {
    let token = jwt.parse::<Token>(&token)?.claims;
    if token.act != Action::Unlock {
        return Err("bad action".into());
    }

    let db = db.deref();
    let ip = remote.ip();
    let it = UserDao::by_uid(db, &token.uid)?;
    if None == it.locked_at {
        return Err("User already isn't locked".into());
    }
    UserDao::unlock(db, &it.id)?;
    LogDao::add(db, &it.id, &ip, "Unlock")?;
    Ok(Redirect::to(SIGN_IN_PATH))
}
