use std::default::Default;
use std::fmt;

use rocket::config::Environment;

use super::errors::Result;

pub type Connection = r2d2_redis::redis::Connection;
pub type Pool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;
pub type PooledConnection = r2d2_redis::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;

#[database("redis")]
pub struct Redis(pub Connection);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db: u8,
}

impl Config {
    pub fn open(&self) -> Result<Pool> {
        let manager = r2d2_redis::RedisConnectionManager::new(&self.to_string()[..])?;
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(pool)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
            db: 0,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "redis://{}:{}/{}", self.host, self.port, self.db)
    }
}
