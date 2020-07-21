use juniper::FieldResult;

use super::super::plugins::nut::graphql as nut;
use super::{context::Context, OK};

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn usersSignIn(context: &Context, form: nut::users::SignIn) -> FieldResult<OK> {
        form.execute(context)?;
        Ok(OK::default())
    }
}
