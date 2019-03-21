use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use chrono::Utc;
use rocket::config::{Config as RocketConfig, Environment, Limits, LoggingLevel, Value};
use uuid::Uuid;

use super::{
    crypto::Key, errors::Result, orm::Config as PostgreSqlConfig,
    queue::rabbitmq::Config as RabbitMQConfig, redis::Config as RedisConfig,
};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

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
    pub postgresql: PostgreSqlConfig,
    pub redis: RedisConfig,
    pub rabbitmq: RabbitMQConfig,
    pub http: Http,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: Environment::Development.to_string(),
            secrets: Key::default(),
            postgresql: PostgreSqlConfig::default(),
            redis: RedisConfig::default(),
            rabbitmq: RabbitMQConfig::default(),
            http: Http::default(),
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
    pub fn is_prod(&self) -> bool {
        self.env() == Environment::Production
    }
    pub fn rocket(&self) -> Result<RocketConfig> {
        let env = self.env();
        let mut databases = HashMap::new();
        {
            let mut it = HashMap::new();
            it.insert("url", Value::from(self.postgresql.to_string()));
            databases.insert("postgresql", Value::from(it));
        }
        {
            let mut it = HashMap::new();
            it.insert("url", Value::from(self.redis.to_string()));
            databases.insert("redis", Value::from(it));
        }

        let it = RocketConfig::build(env)
            .log_level(match env {
                Environment::Production => LoggingLevel::Normal,
                _ => LoggingLevel::Debug,
            })
            .address("127.0.0.1")
            .secret_key(&self.secrets.0[..])
            .limits(
                Limits::new()
                    .limit("forms", 5 * (1 << 20))
                    .limit("json", 5 * (1 << 20)),
            )
            .keep_alive(self.http.keep_alive)
            .port(self.http.port)
            .workers(self.http.workers)
            .extra("databases", databases)
            .finalize()?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub origin: String,
    pub port: u16,
    pub workers: u16,
    pub keep_alive: u32,
    pub upload: Upload,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 1 << 3,
            keep_alive: 120,
            upload: Upload::default(),
            origin: "https://www.change-me.com".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Upload {
    Filesystem {
        local_root: String,
        endpoint: String,
    },
    S3 {
        provider: String,
        aws_access_key_id: String,
        aws_secret_access_key: String,
    },
}

impl Default for Upload {
    fn default() -> Self {
        Upload::Filesystem {
            local_root: "tmp/upload".to_string(),
            endpoint: "/upload".to_string(),
        }
    }
}

impl Upload {
    pub fn save(&self, name: &str, body: &[u8]) -> Result<String> {
        let now = Utc::now().format("%F").to_string();
        let mut file = Path::new(&now).join(Uuid::new_v4().to_string());
        if let Some(ext) = Path::new(&name).extension() {
            file.set_extension(ext);
        }
        match self {
            Upload::Filesystem {
                local_root,
                endpoint,
            } => {
                let dst = Path::new(local_root).join(&file);
                if let Some(d) = dst.parent() {
                    create_dir_all(d)?;
                }
                let mut dst = File::create(dst)?;
                dst.write_all(body)?;
                Ok(format!("{}/{}", endpoint, file.display()))
            }
            _ => Err(format_err!("not support storage")),
        }
    }
}
