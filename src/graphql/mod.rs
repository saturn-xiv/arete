pub mod context;
pub mod mutation;
pub mod query;
pub mod session;

use std::sync::Arc;

use serde::Serialize;

use super::errors::Result;

pub type Schema =
    juniper::RootNode<'static, query::Query, mutation::Mutation, juniper::DefaultScalarValue>;
pub type Context = (Arc<context::Context>, session::Session);

pub trait Handler {
    type Item: Serialize;
    fn handle(&self, c: &context::Context, s: &session::Session) -> Result<Self::Item>;
}
