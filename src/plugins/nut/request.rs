use std::ops::Deref;
use std::sync::Arc;

use hyper::header::Header as HyperHeader;
use rocket::{
    http::{
        hyper::header::{AcceptLanguage, Authorization, Bearer},
        Cookies, Status,
    },
    request::{self, FromRequest},
    Outcome, Request, State,
};

use super::super::super::{i18n::I18n, jwt::Jwt, orm::Database};
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
pub struct Locale(pub String);

impl Locale {
    fn parse(req: &Request) -> Option<String> {
        let key = "locale";
        // 1. Check URL arguments.
        // 2. Get language information from cookies.
        if let Outcome::Success(cookies) = req.guard::<Cookies>() {
            if let Some(it) = cookies.get(key) {
                return Some(it.value().to_string());
            }
        }
        // 3. Get language information from 'Accept-Language'.
        // https://www.w3.org/International/questions/qa-accept-lang-locales
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.4

        if let Ok(AcceptLanguage(al)) = AcceptLanguage::parse_header(
            &req.headers()
                .get(AcceptLanguage::header_name())
                .map(|x| x.as_bytes().to_vec())
                .collect::<Vec<Vec<u8>>>(),
        ) {
            for it in al {
                if let Some(lng) = it.item.language {
                    return Some(lng);
                }
            }
        }
        None
    }
    fn detect(req: &Request) -> Option<String> {
        if let Some(lang) = Self::parse(req) {
            if let Outcome::Success(i18n) = req.guard::<I18n>() {
                if i18n.exist(&lang) {
                    return Some(lang);
                }
            }
        }
        None
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Locale {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let lang = match Self::detect(req) {
            Some(v) => v,
            None => "en-US".to_string(),
        };
        Outcome::Success(Locale(lang))
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

        if let Ok(token) = jwt.parse::<controllers::api::users::Token>(&token) {
            let token = token.claims;
            if token.act == controllers::api::users::Action::SignIn {
                if let Ok(user) = UserDao::by_uid(db, &token.uid) {
                    if let Ok(_) = user.available() {
                        if let Ok(policies) = PolicyDao::all(db, &user.id) {
                            return Outcome::Success(CurrentUser {
                                id: user.id,
                                policies: policies,
                            });
                        }
                    }
                }
            }
        }

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
