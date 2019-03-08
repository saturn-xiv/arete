pub mod routes;
pub mod server;

use actix::prelude::*;

use super::super::{graphql::actix::GraphQLExecutor, orm::DbExecutor, redis::CacheExecutor};

pub struct State {
    pub graphql: Addr<GraphQLExecutor>,
    pub db: Addr<DbExecutor>,
    pub cache: Addr<CacheExecutor>,
}
