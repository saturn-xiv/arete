use juniper::FieldResult;
use validator::Validate;

use super::super::i18n;
use super::context::{Context, Handler};

pub struct Mutation;

graphql_object!(
    Mutation: Context | &self | {
        field saveLocale(&executor, form: i18n::graphql::Save) -> FieldResult<()> {
            form.validate()?;
            form.handle(executor.context())?;
            Ok(())
        }
    }
);
