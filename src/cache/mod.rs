pub mod redis;

use std::time::Duration;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::errors::Result;

pub use self::redis::*;

pub trait Provider {
    fn get<K, V, F>(&mut self, key: &K, ttl: Duration, fun: F) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Serialize,
        V: DeserializeOwned + Serialize;
    fn clear(&mut self) -> Result<()>;
}
