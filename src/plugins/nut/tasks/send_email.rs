use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::{Email, EmailBuilder};
use serde_json;
use validator::Validate;

use super::super::super::super::{
    crypto::sodium::Encryptor as Sodium, errors::Result, orm::Pool as DbPool, queue::Handler,
    settings::Dao as SettingsDao,
};

#[derive(Debug, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[validate(length(min = "1"))]
    pub host: String,
    #[validate(email, length(min = "1"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

#[cfg(debug_assertions)]
pub type Consumer = Printer;
#[cfg(not(debug_assertions))]
pub type Consumer = SendEmail;

pub const NAME: &'static str = "send-email";

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

pub struct Printer {}

impl Printer {
    pub fn new(_dbp: DbPool, _enc: Arc<Sodium>) -> Self {
        Self {}
    }
}

impl Handler for Printer {
    fn handle(&self, _id: String, payload: Vec<u8>) -> Result<()> {
        let task: Task = serde_json::from_slice(&payload)?;
        info!("send email {}", task);
        Ok(())
    }
}

pub struct SendEmail {
    dbp: DbPool,
    enc: Arc<Sodium>,
}

impl SendEmail {
    pub fn new(dbp: DbPool, enc: Arc<Sodium>) -> Self {
        Self { dbp: dbp, enc: enc }
    }
}

impl Handler for SendEmail {
    fn handle(&self, _id: String, payload: Vec<u8>) -> Result<()> {
        let task: Task = serde_json::from_slice(&payload)?;

        info!("send email: {}<{}> {}", task.name, task.email, task.subject);
        let db = self.dbp.get()?;
        let db = db.deref();
        let enc = self.enc.deref();
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
