use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::super::{errors::Result, graphql::context::Context};

#[derive(GraphQLInputObject, Validate)]
#[graphql(description = "User sign in by password")]
pub struct SignIn {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl SignIn {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}
