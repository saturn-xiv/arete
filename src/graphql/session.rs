use std::ops::Deref;
use std::sync::Arc;

use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

use super::super::{
    errors::{Error, Result},
    jwt::Jwt,
    orm::{Connection, Database},
    plugins::nut::models::{
        policy::{Dao as PolicyDao, Role},
        user::{Dao as UserDao, Item as User},
    },
    request::Token as Auth,
};

pub struct Session {
    pub lang: String,
    pub user: Option<User>,
    pub client_ip: String,
}

impl Session {
    pub fn administrator(&self, db: &Connection) -> Result<&User> {
        let user = self.current_user()?;
        if PolicyDao::can(db, user.id, &Role::Admin, &None) {
            return Ok(user);
        }
        Err(Error::Http(Status::Forbidden).into())
    }

    pub fn auth(&self, db: &Connection, role: &Role, resource: &Option<String>) -> Result<&User> {
        let user = self.current_user()?;
        if PolicyDao::can(db, user.id, role, resource) {
            return Ok(user);
        }
        self.administrator(db)
    }

    pub fn current_user(&self) -> Result<&User> {
        match self.user {
            Some(ref v) => Ok(v),
            None => Err(Error::Http(Status::Unauthorized).into()),
        }
    }
}
