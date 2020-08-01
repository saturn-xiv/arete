use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use juniper::GraphQLObject;
use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::{Email, EmailBuilder};
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto,
    errors::Result,
    orm::Pool as Db,
    queue::{Handler as QueueHandler, Task as QueueTask},
    settings::Dao as SettingsDao,
};

#[derive(GraphQLObject, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(email, length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "smtp.gmail.com".to_string(),
            email: "change-me@gmail.com".to_string(),
            password: "".to_string(),
        }
    }
}

impl Config {
    pub const KEY: &'static str = "site.smtp";
}

#[cfg(debug_assertions)]
pub type Consumer = Printer;
#[cfg(not(debug_assertions))]
pub type Consumer = SendEmail;

pub const NAME: &str = "send-email";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

impl fmt::Display for Task {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}<{}>\n{}\n{}",
            self.name, self.email, self.subject, self.body
        )
    }
}

pub struct Job(Task, Config);

impl Into<Result<Email>> for Job {
    fn into(self) -> Result<Email> {
        let Job(t, c) = self;
        let v = EmailBuilder::new()
            .to((t.email, t.name))
            .from(c.email)
            .subject(t.subject)
            .text(t.body)
            .build()?;
        Ok(v)
    }
}

pub struct Printer {
    pub db: Db,
    pub encryptor: Arc<Crypto>,
}

impl QueueHandler for Printer {
    fn handle(&self, task: &QueueTask) -> Result<()> {
        let task: Task = task.get()?;
        info!("send email {}", task);
        Ok(())
    }
}

pub struct SendEmail {
    pub db: Db,
    pub encryptor: Arc<Crypto>,
}

impl QueueHandler for SendEmail {
    fn handle(&self, task: &QueueTask) -> Result<()> {
        let task: Task = task.get()?;
        info!("send email: {}<{}> {}", task.name, task.email, task.subject);

        let db = self.db.get()?;
        let db = db.deref();
        let enc = self.encryptor.deref();
        let cfg: Config = SettingsDao::get(db, enc, &NAME.to_string())?;

        let mut mailer = SmtpClient::new_simple(&cfg.host)?
            .credentials(Credentials::new(cfg.email.clone(), cfg.password.clone()))
            .timeout(Some(Duration::from_secs(60)))
            .transport();
        let email: Result<Email> = Job(task, cfg).into();

        mailer.send(email?.into())?;
        Ok(())
    }
}
