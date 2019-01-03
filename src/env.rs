use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use base64;
use r2d2;

use super::{
    crypto::{self, Encryptor},
    errors::{Error, Result},
    orm::{Connection as DbConnection, Pool as DbPool},
    queue::rabbitmq::Config as RabbitMQConfig,
    redis::Pool as RedisPool,
};

#[cfg(not(debug_assertions))]
pub fn version() -> String {
    format!("{}({})", env!("GIT_HEAD"), env!("BUILD_TIME"))
}
#[cfg(debug_assertions)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &'static str = include_str!("banner.txt");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Environment {
    TEST,
    DEVELOPMENT,
    PRODUCTION,
}

impl fmt::Display for Environment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::TEST => fmt.write_str("test"),
            Environment::DEVELOPMENT => fmt.write_str("development"),
            Environment::PRODUCTION => fmt.write_str("production"),
        }
    }
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "test" => Ok(Environment::TEST),
            "development" => Ok(Environment::DEVELOPMENT),
            "production" => Ok(Environment::PRODUCTION),
            v => Err(format!("unknown environment {}", v).into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub secrets: String,
    pub postgresql: String,
    pub redis: String,
    pub rabbitmq: RabbitMQConfig,
    pub http: Http,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: Environment::DEVELOPMENT,
            http: Http::default(),
            secrets: base64::encode(&crypto::sodium::Encryptor::random(32)),
            redis: "redis://127.0.0.1:5432/0".to_string(),
            postgresql: format!("postgres://postgres:@127.0.0.1:5432/{}", NAME),
            rabbitmq: RabbitMQConfig::default(),
        }
    }
}

impl Config {
    pub fn secrets(&self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.secrets)?;
        Ok(buf)
    }

    pub fn redis(&self) -> Result<RedisPool> {
        let manager = r2d2_redis::RedisConnectionManager::new(&self.redis[..])?;
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(pool)
    }

    pub fn postgresql(&self) -> Result<DbPool> {
        let manager = diesel::r2d2::ConnectionManager::<DbConnection>::new(&self.postgresql[..]);
        Ok(DbPool::new(manager)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub theme: String,
    pub workers: u16,
    pub keep_alive: Option<u32>,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 64,
            theme: "bootstrap".to_string(),
            keep_alive: Some(120),
        }
    }
}

impl Http {
    const THEMES: &'static str = "themes";

    pub fn address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }

    pub fn global(&self) -> PathBuf {
        Path::new(Self::THEMES).join("global")
    }
    pub fn templates(&self) -> PathBuf {
        Path::new(Self::THEMES)
            .join(self.theme.clone())
            .join("views")
    }
    pub fn assets(&self) -> PathBuf {
        Path::new(Self::THEMES)
            .join(self.theme.clone())
            .join("assets")
    }
    pub fn third(&self) -> PathBuf {
        Path::new("node_modules").to_path_buf()
    }
    pub fn upload(&self) -> PathBuf {
        Path::new("tmp").join("upload")
    }
}
