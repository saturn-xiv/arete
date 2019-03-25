use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::{Email, EmailBuilder};
use serde_json;
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    errors::Result,
    graphql::{context::Context, session::Session, Handler},
    orm::Pool as Db,
    queue::Handler as QueueHandler,
    queue::Queue,
    settings::Dao as SettingsDao,
};
use super::super::models::user::Dao as UserDao;

#[derive(Debug, GraphQLInputObject, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[validate(length(min = "1"))]
    pub host: String,
    #[validate(email, length(min = "1"))]
    pub email: String,
    #[validate(length(min = "1"))]
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
    const KEY: &'static str = "site.smtp";
}

#[derive(Validate)]
pub struct Get {}

impl Handler for Get {
    type Item = Config;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let enc = c.encryptor.deref();
        s.administrator(db)?;

        let it: Config = match SettingsDao::get(db, enc, &Config::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => Config::default(),
        };
        Ok(it)
    }
}

impl Handler for Config {
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let enc = c.encryptor.deref();
        s.administrator(db)?;
        SettingsDao::set::<String, Config, Sodium>(db, enc, &Self::KEY.to_string(), &self, true)?;
        Ok(None)
    }
}

#[derive(Validate)]
pub struct Test {}

impl Handler for Test {
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let user = s.administrator(db)?;
        let user = UserDao::by_id(db, &user.id)?;
        c.queue.publish(
            NAME.to_string(),
            Uuid::new_v4().to_string(),
            Task {
                email: user.email.clone(),
                name: user.real_name.clone(),
                subject: format!("Hi, {}", user.real_name),
                body: "This is a test email.".to_string(),
            },
        )?;
        Ok(None)
    }
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

pub struct Printer {
    pub db: Db,
    pub encryptor: Arc<Sodium>,
}

impl QueueHandler for Printer {
    fn handle(&self, _id: String, payload: Vec<u8>) -> Result<()> {
        let task: Task = serde_json::from_slice(&payload)?;
        info!("send email {}", task);
        Ok(())
    }
}

pub struct SendEmail {
    pub db: Db,
    pub encryptor: Arc<Sodium>,
}

impl QueueHandler for SendEmail {
    fn handle(&self, _id: String, payload: Vec<u8>) -> Result<()> {
        let task: Task = serde_json::from_slice(&payload)?;

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
