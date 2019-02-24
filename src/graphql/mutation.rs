use juniper::FieldResult;
use validator::Validate;

use super::super::i18n;
use super::{Context, Handler};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field saveLocale(&executor, form: i18n::graphql::Save) -> FieldResult<()> {
            // TODO
            __graphql!(executor, &form)
        }
    }
);
