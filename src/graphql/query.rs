use juniper::FieldResult;

use super::super::plugins::nut;
use super::{context::Context, Pager};

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

    #[graphql(description = "All locale items")]
    fn listLocale(context: &Context) -> FieldResult<Vec<nut::graphql::locales::Show>> {
        let items = nut::graphql::locales::Show::load(context)?;
        Ok(items)
    }
}
