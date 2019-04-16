use std::sync::Arc;

use super::super::{
    cache::Cache, crypto::Crypto, jwt::Jwt, orm::Database, queue::rabbitmq::RabbitMQ,
};

pub struct Context {
    pub db: Database,
    pub cache: Cache,
    pub queue: Arc<RabbitMQ>,
    pub jwt: Arc<Jwt>,
    pub encryptor: Arc<Crypto>,
}

impl juniper::Context for Context {}
