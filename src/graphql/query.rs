use juniper::FieldResult;
use validator::Validate;

use super::super::i18n;
use super::context::{Context, Handler};

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
    field listLocalesByLang(&executor, form: i18n::graphql::ByLang) -> FieldResult<Vec<i18n::graphql::Item>> {
         form.validate()?;
        let items = form.handle(executor.context())?;
        Ok(items)
    }
});
