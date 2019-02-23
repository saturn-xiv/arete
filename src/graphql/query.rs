use juniper::FieldResult;
use validator::Validate;

use super::super::i18n;
use super::{Context, Handler};

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
         env!("CARGO_PKG_VERSION")
    }
    field listLocalesByLang(&executor, form: i18n::graphql::ByLang) -> FieldResult<Vec<i18n::graphql::Item>> {
        __graphql!(executor, form)
    }
});
