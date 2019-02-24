use std::ops::Deref;
use std::sync::Arc;

use hyper::header::Header as HyperHeader;
use rocket::{
    http::{
        hyper::header::{Authorization, Bearer},
        Status,
    },
    request::{self, FromRequest},
    Outcome, Request, State,
};

use super::super::super::{jwt::Jwt, orm::Database};
use super::{
    controllers,
    models::{
        policy::{Dao as PolicyDao, Role},
        user::Dao as UserDao,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth) = req
            .headers()
            .get_one(Authorization::<Bearer>::header_name())
        {
            if let Ok(auth) = auth.parse::<Bearer>() {
                let header = "Bearer ";
                return Outcome::Success(Token(auth.token[header.len()..].to_string()));
            }
        }
        Outcome::Failure((Status::NonAuthoritativeInformation, ()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentUser {
    pub id: i64,
    pub policies: Vec<(Role, Option<String>)>,
}

impl CurrentUser {
    pub fn is_admin(&self) -> bool {
        self.is(&Role::Admin)
    }
    pub fn is(&self, role: &Role) -> bool {
        self.can(role, &None)
    }
    pub fn can(&self, role: &Role, resource: &Option<String>) -> bool {
        for (rl, rs) in self.policies.iter() {
            if *rl == *role && *rs == *resource {
                return true;
            }
        }
        false
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let Token(token) = req.guard::<Token>()?;
        let Database(db) = req.guard::<Database>()?;
        let db = db.deref();
        let jwt = req.guard::<State<Arc<Jwt>>>()?;
        let jwt = jwt.deref();

        

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Administrator {
    pub id: i64,
}

impl<'a, 'r> FromRequest<'a, 'r> for Administrator {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = req.guard::<CurrentUser>()?;
        if user.is(&Role::Admin) || user.is(&Role::Root) {
            return Outcome::Success(Administrator { id: user.id });
        }

        Outcome::Failure((Status::Forbidden, ()))
    }
}
