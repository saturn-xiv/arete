use juniper::FieldResult;

use super::super::plugins::nut;
use super::{context::Context, Pager};

pub struct Query;

#[juniper::object(
    Context = Context,
    description = "Readonly operations!",
)]
impl Query {
    #[graphql(description = "System information")]
    fn about(context: &Context) -> FieldResult<nut::graphql::About> {
        let it = nut::graphql::About::new(context)?;
        Ok(it)
    }
    fn usersLogs(context: &Context, pager: Pager) -> FieldResult<nut::graphql::users::Logs> {
        Ok(nut::graphql::users::Logs::new(context, &pager)?)
    }
    fn currentUser(context: &Context) -> FieldResult<nut::graphql::users::CurrentUser> {
        let it = nut::graphql::users::CurrentUser::new(context)?;
        Ok(it)
    }
}
