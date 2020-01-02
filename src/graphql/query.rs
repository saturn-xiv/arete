use super::super::env::VERSION;
use super::context::Context;

pub struct Root;

#[juniper::object(
    Context = Context,
)]
impl Root {
    fn apiVersion(context: &Context) -> &str {
        VERSION
    }
}
