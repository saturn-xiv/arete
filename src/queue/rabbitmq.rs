use std::time::{SystemTime, UNIX_EPOCH};

use amq_protocol_uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};
use failure::Error as FailureError;
use futures::{future::Future, Stream};
use lapin::{
    message::Delivery,
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Client, ConnectionProperties,
};
use mime::{Mime, APPLICATION_JSON};
use serde::ser::Serialize;
use tokio::runtime::Runtime;

use super::super::errors::{Error, Result};
use super::{Handler, Queue};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub virtual_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5672,
            username: "guest".to_string(),
            password: "guest".to_string(),
            virtual_host: "/dev".to_string(),
        }
    }
}

impl Config {
    pub fn open(self) -> RabbitMQ {
        RabbitMQ {
            uri: AMQPUri {
                vhost: self.virtual_host,
                authority: AMQPAuthority {
                    port: self.port,
                    host: self.host,
                    userinfo: AMQPUserInfo {
                        username: self.username,
                        password: self.password,
                    },
                },
                ..Default::default()
            },
            conn: ConnectionProperties::default(),
        }
    }
}

pub struct RabbitMQ {
    uri: AMQPUri,
    conn: ConnectionProperties,
}

impl Queue for RabbitMQ {
    fn publish<T: Serialize>(&self, queue: String, mid: String, payload: T) -> Result<()> {
        info!("publish task {}", mid);

        let payload = serde_json::to_vec(&payload)?;

        let rt = Client::connect_uri(self.uri.clone(), self.conn.clone())
            .map_err(FailureError::from)
            .and_then(move |client| client.create_channel().map_err(FailureError::from))
            .and_then(move |channel| {
                let id = channel.id();
                info!("created channel with id: {}", id);

                channel
                    .queue_declare(
                        &queue.clone(),
                        QueueDeclareOptions::default(),
                        FieldTable::default(),
                    )
                    .and_then(move |_| {
                        info!("channel {} declared queue {}", id, queue);
                        channel
                            .basic_publish(
                                "",
                                &queue,
                                payload,
                                BasicPublishOptions::default(),
                                BasicProperties::default()
                                    .with_message_id((&mid[..]).into())
                                    .with_content_type((&APPLICATION_JSON.to_string()[..]).into())
                                    .with_timestamp(
                                        SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .expect("get timestamp")
                                            .as_secs(),
                                    ),
                            )
                            .map(|confirmation| {
                                info!("publish got confirmation: {:?}", confirmation)
                            })
                    })
                    .map_err(FailureError::from)
            });

        Runtime::new()?.block_on_all(rt)?;
        Ok(())
    }

    fn consume(
        &self,
        consumer_name: String,
        queue_name: String,
        handler: Box<dyn Handler>,
    ) -> Result<()> {
        let rt = Client::connect_uri(self.uri.clone(), self.conn.clone())
            .map_err(FailureError::from)
            .and_then(move |client| client.create_channel().map_err(FailureError::from))
            .and_then(move |channel| {
                let id = channel.id();
                info!("created channel with id: {}", id);
                let c = channel.clone();
                channel
                    .queue_declare(
                        &queue_name.clone(),
                        QueueDeclareOptions::default(),
                        FieldTable::default(),
                    )
                    .and_then(move |queue| {
                        info!("channel {} declared queue {:?}", id, queue);
                        channel.basic_consume(
                            &queue,
                            &consumer_name,
                            BasicConsumeOptions::default(),
                            FieldTable::default(),
                        )
                    })
                    .and_then(|stream| {
                        info!("got consumer stream");
                        stream.for_each(move |message| {
                            let tag = message.delivery_tag;
                            if let Err(e) = handle_message(message, &handler) {
                                error!("failed to handle message: {:?}", e);
                            }
                            c.basic_ack(tag, false)
                        })
                    })
                    .map_err(FailureError::from)
            });

        Runtime::new()?.block_on_all(rt)?;
        Ok(())
    }
}

pub fn handle_message(msg: Delivery, hnd: &Box<dyn Handler>) -> Result<()> {
    let props = msg.properties;
    debug!("got message: {:?}", props);

    let ct: Result<()> = match props.content_type() {
        Some(v) => {
            let v = v.to_string();
            if v.parse::<Mime>()? == APPLICATION_JSON {
                Ok(())
            } else {
                Err(Error::RabbitMQBadContentType(v).into())
            }
        }
        None => Err(Error::RabbitMQEmptyContentType.into()),
    };
    ct?;

    let id: Result<String> = match props.message_id() {
        Some(v) => Ok(v.to_string()),
        None => Err(Error::RabbitMQEmptyMessageId.into()),
    };
    let id = id?;
    info!("consume message {}", id);
    hnd.handle(id, msg.data)
}
