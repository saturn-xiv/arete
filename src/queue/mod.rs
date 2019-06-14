pub mod rabbitmq;

use serde::ser::Serialize;

use super::errors::Result;

pub trait Queue {
    fn publish<T: Serialize>(&self, name: String, id: String, payload: T) -> Result<()>;
    fn consume(&self, consumer: String, queue: String, handler: Box<dyn Handler>) -> Result<()>;
}

pub trait Handler: Sync + Send {
    fn handle(&self, id: String, payload: Vec<u8>) -> Result<()>;
}
