use std::default::Default;
use std::fmt;
use std::time::Duration;

use r2d2_redis::redis::cmd;
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::errors::Result;

pub type Connection = r2d2_redis::redis::Connection;
pub type Pool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;
pub type PooledConnection = r2d2_redis::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;

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

impl super::Provider for Connection {
    fn get<K, V, F>(&mut self, key: &K, ttl: Duration, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize,
    {
        let key = format!("cache://{}", serde_json::to_string(key)?);
        if let Ok(buf) = cmd("get").arg(&key).query::<Vec<u8>>(self) {
            if let Ok(val) = serde_json::from_slice(buf.as_slice()) {
                return Ok(val);
            }
        }
        warn!("can't get from cache {:?}", key);
        let val = fun()?;
        let _: String = cmd("set")
            .arg(&key)
            .arg(serde_json::to_vec(&val)?.as_slice())
            .arg("ex")
            .arg(ttl.as_secs())
            .query(self)?;
        Ok(val)
    }
    fn clear(&mut self) -> Result<()> {
        warn!("clear cache");
        let rst = cmd("flushdb").query::<String>(self)?;
        info!("{}", rst);
        Ok(())
    }
}
