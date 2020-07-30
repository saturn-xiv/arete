use juniper::GraphQLInputObject;
use validator::Validate;

use super::super::super::super::{errors::Result, graphql::context::Context};

#[derive(GraphQLInputObject, Validate)]
#[graphql(
    name = "SignInUser",
    description = "User sign in by nickname/email & password"
)]
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

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "SignUpUser", description = "User sign up by email")]
pub struct SignUp {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl SignUp {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UpdateUserProfile", description = "Update user profile")]
pub struct Profile {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl Profile {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UserConfirm", description = "Confirm user registerion")]
pub struct Confirm {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl Confirm {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UserUnlock", description = "Unlock your account")]
pub struct Unlock {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl Unlock {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}

#[derive(GraphQLInputObject, Validate)]
#[graphql(name = "UserForgotPassword", description = "Forgot password?")]
pub struct ForgotPassword {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl ForgotPassword {
    pub fn execute(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }
}
