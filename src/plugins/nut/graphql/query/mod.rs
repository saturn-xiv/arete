pub mod admin;

use validator::Validate;

use super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler},
};
use super::super::models::user::Show as UserInfo;

#[derive(Validate)]
pub struct CurrentUser;

impl Handler for CurrentUser {
    type Item = Option<UserInfo>;
    fn handle(&self, _: &Context, s: &Session) -> Result<Self::Item> {
        if let Some(ref v) = s.user {
            return Ok(Some((*v).clone().into()));
        }
        Ok(None)
    }
}
