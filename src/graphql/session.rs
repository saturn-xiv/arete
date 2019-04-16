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
    plugins::nut::{
        graphql::users::{Action, Token},
        models::{
            policy::{Dao as PolicyDao, Role},
            user::{Dao as UserDao, Item as User},
        },
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
        if PolicyDao::can(db, &user.id, &Role::Admin, &None) {
            return Ok(user);
        }
        Err(Error::Http(Status::Forbidden).into())
    }

    pub fn auth(&self, db: &Connection, role: &Role, resource: &Option<String>) -> Result<&User> {
        let user = self.current_user()?;
        if PolicyDao::can(db, &user.id, role, resource) {
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

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let Auth(token) = req.guard::<Auth>()?;
        let jwt = req.guard::<State<Arc<Jwt>>>()?;
        if let Ok(token) = jwt.parse::<Token>(&token) {
            let token = token.claims;
            if token.act == Action::SignIn {
                let Database(db) = req.guard::<Database>()?;
                let db = db.deref();
                if let Ok(user) = UserDao::by_uid(db, &token.uid) {
                    if let Ok(_) = user.available() {
                        return Outcome::Success(user);
                    }
                }
            }
        }

        Outcome::Failure((Status::NonAuthoritativeInformation, ()))
    }
}
