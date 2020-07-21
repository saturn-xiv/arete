use juniper::FieldResult;

use super::super::plugins::nut::graphql as nut;
use super::context::Context;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn about(context: &Context) -> FieldResult<nut::About> {
        let it = nut::About::new(context)?;
        Ok(it)
    }
}
