use std::sync::Arc;

use super::super::{
    crypto::sodium::Encryptor as Sodium,
    env::Config,
    errors::Result,
    jwt::Jwt,
    orm::{Database, Pool as DbPool, PooledConnection as DbConnection},
    queue::rabbitmq::RabbitMQ,
    redis::{Pool as CachePool, PooledConnection as CacheConnection, Redis},
};

pub struct Context {
    pub db: Database,
    pub cache: Redis,
    // pub queue: RabbitMQ,
    pub jwt: Arc<Jwt>,
    // pub encryptor: Sodium,
}

impl Context {
    // pub fn new(cfg: &Config) -> Result<Self> {
    //     let it = Self {
    //         db: cfg.postgresql.open()?,
    //         queue: cfg.rabbitmq.clone().open()?,
    //         jwt: Jwt::new(cfg.secrets.0.clone()),
    //         cache: cfg.redis.open()?,
    //         encryptor: Sodium::new(cfg.secrets.clone())?,
    //     };
    //     Ok(it)
    // }
    // pub fn db(&self) -> Result<DbConnection> {
    //     let it = self.db.get()?;
    //     Ok(it)
    // }
    // pub fn cache(&self) -> Result<CacheConnection> {
    //     let it = self.cache.get()?;
    //     Ok(it)
    // }
}

impl juniper::Context for Context {}
