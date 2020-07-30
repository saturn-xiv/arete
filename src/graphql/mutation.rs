use juniper::FieldResult;
use validator::Validate;

use super::super::plugins::nut::graphql as nut;
use super::{context::Context, OK};

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn usersSignIn(context: &Context, form: nut::users::SignIn) -> FieldResult<OK> {
        form.validate()?;
        form.execute(context)?;
        Ok(OK::default())
    }
    fn usersSignUp(context: &Context, form: nut::users::SignUp) -> FieldResult<OK> {
        form.validate()?;
        form.execute(context)?;
        Ok(OK::default())
    }
    fn usersProfile(context: &Context, form: nut::users::SignUp) -> FieldResult<OK> {
        form.validate()?;
        form.execute(context)?;
        Ok(OK::default())
    }
}
