use std::sync::Arc;

use super::super::{
    crypto::sodium::Encryptor as Sodium, jwt::Jwt, orm::Database, queue::rabbitmq::RabbitMQ,
    redis::Redis,
};

pub struct Context {
    pub db: Database,
    pub cache: Redis,
    pub queue: Arc<RabbitMQ>,
    pub jwt: Arc<Jwt>,
    pub encryptor: Arc<Sodium>,
}

impl juniper::Context for Context {}
