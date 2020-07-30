use std::sync::Arc;

use super::super::{
    cache::PooledConnection as Cache, crypto::Crypto, jwt::Jwt, orm::PooledConnection as Db,
    plugins::nut::models::user::Item as User, queue::rabbitmq::RabbitMQ,
};

pub struct Context {
    pub locale: String,
    pub db: Db,
    pub jwt: Arc<Jwt>,
    pub cache: Cache,
    pub crypto: Arc<Crypto>,
    pub queue: Arc<RabbitMQ>,
    pub token: Option<String>,
    pub client_ip: String,
    pub user: Option<User>,
}

impl juniper::Context for Context {}
