use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::super::super::{errors::Result, graphql::context::Context};

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "LockUser", description = "Lock account by uid")]
pub struct Lock {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl Lock {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}
