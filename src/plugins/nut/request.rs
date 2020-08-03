use std::fmt;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

use actix_web::{
    dev::Payload,
    error::{Error as ActixError, ErrorUnauthorized},
    http::StatusCode,
    web, FromRequest, HttpRequest,
};
use futures::future::Future;

use super::super::super::{
    errors::{Error, Result as Result_},
    jwt::Jwt,
    orm::Pool as DbPool,
    request::Token as Auth,
};
use super::models::user::{Dao as UserDao, Item as User};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    SignIn,
    Confirm,
    Unlock,
    ResetPassword,
}

impl fmt::Display for Action {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::SignIn => fmt.write_str("sign-in"),
            Action::Confirm => fmt.write_str("confirm"),
            Action::Unlock => fmt.write_str("unlock"),
            Action::ResetPassword => fmt.write_str("reset-password"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub uid: String,
    pub sub: String,
    pub act: Action,
    pub nbf: i64,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentUser(pub User);

impl fmt::Display for CurrentUser {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl CurrentUser {
    pub fn parse(token: Auth, db: web::Data<DbPool>, jwt: web::Data<Arc<Jwt>>) -> Result_<User> {
        let token = jwt.parse::<Token>(&token.0)?;
        if token.claims.act != Action::SignIn {
            return Err(Error::Http(StatusCode::BAD_REQUEST).into());
        }
        let db = db.get()?;
        let db = db.deref();
        let user = UserDao::by_uid(db, &token.claims.uid)?;
        user.available()?;
        Ok(user)
    }
}

impl FromRequest for CurrentUser {
    type Config = ();
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let auth = Auth::from_request(req, pl);
        let jwt = web::Data::<Arc<Jwt>>::from_request(req, pl);
        let db = web::Data::<DbPool>::from_request(req, pl);

        Box::pin(async move {
            let auth = auth.await?;
            let jwt = jwt.await?;
            let db = db.await?;

            match Self::parse(auth, db, jwt) {
                Ok(it) => Ok(Self(it)),
                Err(e) => Err(ErrorUnauthorized(e)),
            }
        })
    }
}
