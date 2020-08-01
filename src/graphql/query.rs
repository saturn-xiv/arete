use juniper::FieldResult;

use super::super::plugins::nut;
use super::{context::Context, Pager, ID};

pub struct Query;

#[juniper::object(
    Context = Context,
    description = "Readonly operations!",
)]
impl Query {
    #[graphql(description = "System information.")]
    fn about(context: &Context) -> FieldResult<nut::graphql::About> {
        let it = nut::graphql::About::new(context)?;
        Ok(it)
    }

    #[graphql(description = "Current user's logs.")]
    fn usersLogs(context: &Context, pager: Pager) -> FieldResult<nut::graphql::users::Logs> {
        Ok(nut::graphql::users::Logs::new(context, &pager)?)
    }
    #[graphql(description = "Current user's information.")]
    fn currentUser(context: &Context) -> FieldResult<nut::graphql::users::CurrentUser> {
        let it = nut::graphql::users::CurrentUser::new(context)?;
        Ok(it)
    }
    #[graphql(description = "List all user")]
    fn indexUser(context: &Context) -> FieldResult<Vec<nut::graphql::users::User>> {
        let items = nut::graphql::users::User::index(context)?;
        Ok(items)
    }
    #[graphql(description = "List all user's policies")]
    fn indexUserPolicies(
        context: &Context,
        id: ID,
    ) -> FieldResult<Vec<nut::graphql::users::Policy>> {
        let items = nut::graphql::users::Policy::index(context, id)?;
        Ok(items)
    }

    #[graphql(description = "All locale items")]
    fn indexLocale(context: &Context) -> FieldResult<Vec<nut::graphql::locales::Locale>> {
        let items = nut::graphql::locales::Locale::index(context)?;
        Ok(items)
    }
}
