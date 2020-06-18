use std::default::Default;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use super::{
    cache::Config as CacheConfig, crypto::Key, errors::Result, orm::Config as DatabaseConfig,
    queue::rabbitmq::Config as RabbitMQConfig,
};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &str = include_str!("banner.txt");

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Environment {
    Production,
    Development,
    Test,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Production => fmt.write_str("production"),
            Environment::Development => fmt.write_str("development"),
            Environment::Test => fmt.write_str("test"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub env: Environment,
    pub secrets: Key,
    pub http: Http,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub rabbitmq: RabbitMQConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub origin: String,
    pub port: u16,
    pub upload: Upload,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
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
