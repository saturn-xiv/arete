use std::time::{SystemTime, UNIX_EPOCH};

use amq_protocol_uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};

use futures_util::stream::StreamExt;
use lapin::{
    message::Delivery, options::*, types::FieldTable, BasicProperties, Channel, Connection,
    ConnectionProperties,
};

use super::super::errors::Result;
use super::Handler;

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
    pub fn open(&self) -> RabbitMQ {
        RabbitMQ {
            uri: AMQPUri {
                vhost: self.virtual_host.clone(),
                authority: AMQPAuthority {
                    port: self.port,
                    host: self.host.clone(),
                    userinfo: AMQPUserInfo {
                        username: self.username.clone(),
                        password: self.password.clone(),
                    },
                },
                ..Default::default()
            },
            conn: ConnectionProperties::default(),
        }
    }
}

#[derive(Clone)]
pub struct RabbitMQ {
    uri: AMQPUri,
    conn: ConnectionProperties,
}

impl RabbitMQ {
    pub async fn open(&self, queue: &str) -> Result<Channel> {
        let con = Connection::connect_uri(self.uri.clone(), self.conn.clone()).await?;
        let ch = con.create_channel().await?;
        ch.queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await?;
        Ok(ch)
    }

    pub async fn publish(&self, queue: &str, task: super::Task) -> Result<()> {
        let ch = self.open(queue).await?;
        info!("publish task {}://{}", queue, task.id);
        ch.basic_publish(
            "",
            queue,
            BasicPublishOptions::default(),
            task.payload,
            BasicProperties::default()
                .with_message_id((&task.id[..]).into())
                .with_content_type((&task.content_type[..]).into())
                .with_timestamp(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("get timestamp")
                        .as_secs(),
                ),
        )
        .await?
        .await?;

        Ok(())
    }

    pub async fn consume<H: Handler>(
        &self,
        consumer: &str,
        queue: &str,
        handler: &H,
    ) -> Result<()> {
        let ch = self.open(queue).await?;
        let mut cm = ch
            .basic_consume(
                queue,
                consumer,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!(
            "consuming from channel {}@{}/{}...",
            consumer,
            queue,
            ch.id()
        );
        while let Some(msg) = cm.next().await {
            let (ch, msg) = msg?;
            debug!("received message: {:?}", msg);
            handle_message(msg.clone(), handler)?;
            ch.basic_ack(msg.delivery_tag, BasicAckOptions::default())
                .wait()?;
        }
        Ok(())
    }
}

pub fn handle_message<H: Handler>(msg: Delivery, hnd: &H) -> Result<()> {
    let props = msg.properties;
    info!("got message: {:?}", props);

    if let Some(content_type) = props.content_type() {
        if let Some(id) = props.message_id() {
            return hnd.handle(&super::Task {
                id: id.to_string(),
                content_type: content_type.to_string(),
                payload: msg.data,
            });
        }
    }

    Err(format_err!("bad task message"))
}
