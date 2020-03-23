use rusoto_core::{request::HttpClient, Region};
use rusoto_sqs::{
    CreateQueueRequest, GetQueueUrlRequest, ReceiveMessageRequest, SendMessageRequest,
    Sqs as AwsSqs, SqsClient,
};

use super::super::super::{
    errors::Result,
    queue::{Handler, Task},
};

/// https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-limits.html
pub struct Sqs {
    client: SqsClient,
}

impl Sqs {
    pub fn new(cred: super::Credentials, region: Region) -> Result<Self> {
        Ok(Self {
            client: SqsClient::new_with(HttpClient::new()?, cred.provider(), region),
        })
    }

    pub async fn create_queue(&self, name: String) -> Result<()> {
        self.client
            .create_queue(CreateQueueRequest {
                queue_name: name,
                ..Default::default()
            })
            .await?;
        Ok(())
    }

    async fn get_queue_url(&self, name: String) -> Result<String> {
        if let Some(it) = self
            .client
            .get_queue_url(GetQueueUrlRequest {
                queue_name: name,
                ..Default::default()
            })
            .await?
            .queue_url
        {
            return Ok(it);
        }

        Err(format_err!("can't find queue"))
    }

    pub async fn publish(&self, queue: String, task: Task) -> Result<()> {
        self.client
            .send_message(SendMessageRequest {
                message_body: serde_json::to_string(&task)?,
                queue_url: self.get_queue_url(queue).await?,
                ..Default::default()
            })
            .await?;
        Ok(())
    }
    pub async fn consume(
        &self,
        _consumer: String,
        queue: String,
        handler: Box<dyn Handler>,
    ) -> Result<()> {
        if let Some(items) = self
            .client
            .receive_message(ReceiveMessageRequest {
                queue_url: self.get_queue_url(queue).await?,
                ..Default::default()
            })
            .await?
            .messages
        {
            for it in items {
                if let Some(ref body) = it.body {
                    handler.handle(&serde_json::from_str(body)?)?;
                }
            }
        }
        Ok(())
    }
}
