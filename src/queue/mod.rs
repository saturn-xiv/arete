pub mod rabbitmq;

use mime::APPLICATION_JSON;
use serde::{de::DeserializeOwned, ser::Serialize};
use uuid::Uuid;

use super::errors::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub content_type: String,
    pub payload: Vec<u8>,
}

impl Task {
    pub fn new<T: Serialize>(payload: &T) -> Result<Self> {
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            content_type: APPLICATION_JSON.to_string(),
            payload: serde_json::to_vec(payload)?,
        })
    }

    pub fn get<T: DeserializeOwned>(&self) -> Result<T> {
        if APPLICATION_JSON.to_string() == self.content_type {
            let it = serde_json::from_slice(&self.payload)?;
            return Ok(it);
        }
        Err(Error::BadTaskMessage.into())
    }
}

pub trait Queue: Send + Sync {
    fn publish(&self, queue: String, task: Task) -> Result<()>;
    fn consume(&self, consumer: String, queue: String, handler: Box<dyn Handler>) -> Result<()>;
}

pub trait Handler: Sync + Send {
    fn handle(&self, task: &Task) -> Result<()>;
}
