use serde::Serialize;

use super::super::{
    crypto::sodium::Encryptor as Sodium,
    env::Config,
    errors::Result,
    jwt::Jwt,
    orm::{Pool as Database, PooledConnection as DbConnection},
    queue::rabbitmq::RabbitMQ,
    redis::Pool as Cache,
};

pub struct Context {
    pub db: Database,
    pub cache: Cache,
    pub queue: RabbitMQ,
    pub jwt: Jwt,
    pub encryptor: Sodium,
}

impl Context {
    pub fn new(cfg: &Config) -> Result<Self> {
        let it = Self {
            db: cfg.postgresql.open()?,
            queue: cfg.rabbitmq.clone().open()?,
            jwt: Jwt::new(cfg.secrets.0.clone()),
            cache: cfg.redis.open()?,
            encryptor: Sodium::new(cfg.secrets.clone())?,
        };
        Ok(it)
    }
    pub fn db(&self) -> Result<DbConnection> {
        let it = self.db.get()?;
        Ok(it)
    }
}

impl juniper::Context for Context {}

pub trait Handler {
    type Item: Serialize;
    fn handle(&self, ctx: &Context) -> Result<Self::Item>;
}
