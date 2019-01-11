use std::ops::Deref;
use std::time::Duration;

use r2d2_redis::redis::cmd;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use super::super::{errors::Result, redis::Connection};

impl super::Cache for Connection {
    fn get<K, V, F>(&self, key: &K, ttl: Duration, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize,
    {
        let key = format!("cache://{}", serde_json::to_string(key)?);
        let db = self.deref();
        if let Ok(buf) = cmd("get").arg(&key).query::<Vec<u8>>(db) {
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
            .query(db)?;
        Ok(val)
    }
}
