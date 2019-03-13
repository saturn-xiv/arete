pub mod routes;
pub mod server;

use std::sync::Arc;

use actix::prelude::*;

use super::super::graphql::{actix::GraphQLExecutor, context::Context};

pub struct State {
    pub graphql: Addr<GraphQLExecutor>,
    pub context: Arc<Context>,
}
