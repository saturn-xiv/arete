use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rocket::config::{Config as RocketConfig, Environment, Limits, LoggingLevel, Value};

use super::{
    crypto::Key,
    errors::Result,
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
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub env: String,
    pub secrets: Key,
    pub database: String,
    pub redis: String,
    pub rabbitmq: RabbitMQConfig,
    pub http: Http,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: Environment::Development.to_string(),
            http: Http::default(),
            secrets: Key::default(),
            redis: "redis://127.0.0.1:6379/0".to_string(),
            database: format!("postgres://postgres:@127.0.0.1:5432/{}", NAME),
            rabbitmq: RabbitMQConfig::default(),
        }
    }
}

impl Config {
    pub fn env(&self) -> Environment {
        match self.env.parse() {
            Ok(v) => v,
            Err(_) => Environment::Development,
        }
    }

    pub fn rocket(&self) -> Result<RocketConfig> {
        let env = self.env();
        let mut databases = BTreeMap::new();
        {
            let mut cfg = BTreeMap::new();
            cfg.insert("url".to_string(), Value::String(self.database.clone()));
            databases.insert("postgresql".to_string(), cfg);
        }
        {
            let mut cfg = BTreeMap::new();
            cfg.insert("url".to_string(), Value::String(self.redis.clone()));
            databases.insert("redis".to_string(), cfg);
        }

        let it = RocketConfig::build(env)
            .address("0.0.0.0")
            .workers(self.http.workers)
            .port(self.http.port)
            .secret_key(self.secrets.0.clone())
            .keep_alive(match self.http.keep_alive {
                Some(v) => v,
                None => 0,
            })
            .limits(
                Limits::new()
                    .limit("forms", self.http.limits * (1 << 10 << 10))
                    .limit("json", self.http.limits * (1 << 10 << 10)),
            )
            .extra("databases", databases)
            .extra(
                "template_dir",
                match self.http.templates().to_str() {
                    Some(v) => v,
                    None => "templates",
                },
            )
            .log_level(match env {
                Environment::Production => LoggingLevel::Normal,
                _ => LoggingLevel::Debug,
            })
            .finalize()?;

        Ok(it)
    }

    pub fn redis(&self) -> Result<RedisPool> {
        let manager = r2d2_redis::RedisConnectionManager::new(&self.redis[..])?;
        let pool = r2d2::Pool::builder().build(manager)?;
        Ok(pool)
    }

    pub fn database(&self) -> Result<DbPool> {
        let manager = diesel::r2d2::ConnectionManager::<DbConnection>::new(&self.database[..]);
        Ok(DbPool::new(manager)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub port: u16,
    pub theme: String,
    pub workers: u16,
    pub limits: u64,
    pub keep_alive: Option<u32>,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 1 << 3,
            limits: 1 << 5,
            theme: "bootstrap".to_string(),
            keep_alive: Some(120),
        }
    }
}

impl Http {
    pub const THEMES: &'static str = "themes";

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
