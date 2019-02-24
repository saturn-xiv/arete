use juniper::FieldResult;
use validator::Validate;

use super::super::{i18n, plugins::nut};
use super::{Context, Handler};

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
         env!("CARGO_PKG_VERSION")
    }
    field listLocaleByLang(&executor, lang: String) -> FieldResult<Vec<i18n::graphql::Item>> {
        __graphql!(executor, &i18n::graphql::ByLang{lang: lang.clone()})
    }
    field availableLanguage(&executor) -> FieldResult<Vec<String>> {
        __graphql!(executor, &i18n::graphql::Languages)
    }
    field currentUser(&executor) -> FieldResult<Option<nut::models::user::Show>> {
        __graphql!(executor, &nut::graphql::query::CurrentUser)
    }
});
