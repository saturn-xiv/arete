use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use failure::Error as FailureError;
use futures::{future::Future, Stream};
use lapin::{
    channel::{
        BasicConsumeOptions, BasicProperties, BasicPublishOptions, ConfirmSelectOptions,
        QueueDeclareOptions,
    },
    client::ConnectionOptions,
    message::Delivery,
    types::FieldTable,
};
use mime::{Mime, APPLICATION_JSON};
use serde::ser::Serialize;
use serde_json;
use tokio::{net::TcpStream, runtime::Runtime};

use super::super::{env::NAME, errors::Result};
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
            virtual_host: format!("/{}", NAME),
        }
    }
}

impl Config {
    pub fn open(self) -> Result<RabbitMQ> {
        RabbitMQ::new(
            format!("{}:{}", self.host, self.port),
            self.username,
            self.password,
            self.virtual_host,
        )
    }
}

pub struct RabbitMQ {
    addr: SocketAddr,
    options: ConnectionOptions,
}

impl RabbitMQ {
    pub fn new(addr: String, username: String, password: String, vhost: String) -> Result<Self> {
        Ok(Self {
            addr: addr.parse()?,
            options: ConnectionOptions {
                username: username,
                password: password,
                vhost: vhost,
                ..Default::default()
            },
        })
    }
}
impl Queue for RabbitMQ {
    fn publish<T: Serialize>(&self, queue: String, mid: String, payload: T) -> Result<()> {
        info!("publish task {}", mid);
        let options = self.options.clone();
        let payload = serde_json::to_vec(&payload)?;

        let rt = TcpStream::connect(&self.addr)
            .map_err(FailureError::from)
            .and_then(|stream| {
                lapin::client::Client::connect(stream, options).map_err(FailureError::from)
            })
            .and_then(move |(client, _)| {
                client
                    .create_confirm_channel(ConfirmSelectOptions::default())
                    .map_err(FailureError::from)
            })
            .and_then(move |channel| {
                let id = channel.id;
                info!("created channel with id: {}", id);

                channel
                    .queue_declare(
                        &queue.clone(),
                        QueueDeclareOptions::default(),
                        FieldTable::new(),
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
                                    .with_message_id(mid)
                                    .with_content_type(APPLICATION_JSON.to_string())
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
                            .and_then(move |_| {
                                info!("close channel");
                                channel.close_ok()
                            })
                    })
                    .map_err(FailureError::from)
            });

        if let Err(e) = Runtime::new()?.block_on_all(rt) {
            return Err(format!("failed on publish message {}", e).into());
        }
        Ok(())
    }

    fn consume(
        &self,
        consumer_name: String,
        queue_name: String,
        handler: Box<Handler>,
    ) -> Result<()> {
        let options = self.options.clone();

        let rt = TcpStream::connect(&self.addr)
            .map_err(FailureError::from)
            .and_then(|stream| {
                lapin::client::Client::connect(stream, options).map_err(FailureError::from)
            })
            .and_then(move |(client, heartbeat)| {
                tokio::spawn(heartbeat.map_err(|e| error!("heartbeat error: {}", e)));
                client.create_channel().map_err(FailureError::from)
            })
            .and_then(move |channel| {
                let id = channel.id;
                info!("created channel with id: {}", id);
                let c = channel.clone();
                channel
                    .queue_declare(
                        &queue_name.clone(),
                        QueueDeclareOptions::default(),
                        FieldTable::new(),
                    )
                    .and_then(move |queue| {
                        info!("channel {} declared queue {:?}", id, queue);
                        channel.basic_consume(
                            &queue,
                            &consumer_name,
                            BasicConsumeOptions::default(),
                            FieldTable::new(),
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

        match Runtime::new()?.block_on_all(rt) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("failed on consume message: {:?}", e).into()),
        }
    }
}

pub fn handle_message(msg: Delivery, hnd: &Box<Handler>) -> Result<()> {
    debug!("got message: {:?}", msg);

    let ct: Result<()> = match msg.properties.content_type() {
        Some(v) => {
            if v.parse::<Mime>()? == APPLICATION_JSON {
                Ok(())
            } else {
                Err(format!("bad message content type {}", v).into())
            }
        }
        None => Err("empty message id".into()),
    };
    ct?;

    let id: Result<String> = match msg.properties.message_id() {
        Some(v) => Ok(v.to_string()),
        None => Err("empty message id".into()),
    };
    let id = id?;
    info!("consume message {}", id);
    hnd.handle(id, msg.data)
}
