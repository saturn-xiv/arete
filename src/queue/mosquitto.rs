use std::fmt::Debug;
use std::result::Result as StdResult;
use std::time::Duration;

use paho_mqtt::{Client, ConnectOptionsBuilder, CreateOptionsBuilder, MessageBuilder};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::{
    errors::Result,
    sys::{hostname, pid},
};

pub struct Connection {
    queue: paho_mqtt::Client,
}

impl Connection {
    const QOS: i32 = 1;
    pub const WILL: &'static str = "will";
    pub const KEEP_ALIVE: u64 = 20;
    pub fn new(host: &str, port: Option<u16>) -> Result<Self> {
        Ok(Self {
            queue: Client::new(
                CreateOptionsBuilder::new()
                    .server_uri(format!(
                        "tcp://{}:{}",
                        host,
                        match port {
                            Some(v) => v,
                            None => 1883,
                        }
                    ))
                    .client_id(format!("{}-{}", hostname()?, pid()))
                    .finalize(),
            )?,
        })
    }

    pub fn reconnect(&mut self) -> Result<()> {
        self.queue.reconnect()?;
        Ok(())
    }
    pub fn send<K: Serialize + Debug>(&self, payload: &K, to: &[String]) -> Result<()> {
        self.queue.connect(None)?;
        for to in to {
            self.queue.publish(
                MessageBuilder::new()
                    .topic(to)
                    .payload(serde_json::to_vec(payload)?)
                    .qos(Self::QOS)
                    .finalize(),
            )?;
        }
        Ok(())
    }

    pub fn receive<V, E, H>(&mut self, id: &str, topics: &[String], hnd: &H) -> Result<()>
    where
        V: DeserializeOwned + Debug,
        E: Debug,
        H: Handler<Item = V, Error = E>,
    {
        let rx = self.queue.start_consuming();

        debug!("connecting to the MQTT broker...");
        self.queue.connect(
            ConnectOptionsBuilder::new()
                .keep_alive_interval(Duration::from_secs(Self::KEEP_ALIVE))
                .clean_session(false)
                .will_message(
                    paho_mqtt::MessageBuilder::new()
                        .topic(Self::WILL)
                        .payload(id)
                        .finalize(),
                )
                .finalize(),
        )?;
        let qv = self.queue.subscribe_many(topics, &[Self::QOS, Self::QOS])?;
        debug!("QoS granted: {:?}", qv);

        debug!("waiting for messages...");
        for msg in rx.iter() {
            if let Some(msg) = msg {
                let payload = serde_json::from_slice(msg.payload())?;
                info!(
                    "receive message {}[{}] {:?}",
                    msg.topic(),
                    msg.qos(),
                    payload
                );
                if let Err(e) = hnd.handle(&payload) {
                    error!("{:?}", e);
                }
            }
        }
        Ok(())
    }
}

pub trait Handler {
    type Item;
    type Error: Debug;
    fn handle(&self, msg: &Self::Item) -> StdResult<(), Self::Error>;
}
