use juniper::FieldResult;
use validator::Validate;

use super::super::{i18n, plugins::nut};
use super::{Context, Handler};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field saveLocale(&executor, form: i18n::graphql::Save) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
        field signIn(&executor, form: nut::graphql::mutation::users::SignIn) -> FieldResult<String> {
            __graphql!(executor, &form)
        }
        field signUp(&executor, form: nut::graphql::mutation::users::SignUp) -> FieldResult<()> {
            __graphql!(executor, &form)
        }
    }
);
