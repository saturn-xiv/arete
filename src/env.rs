use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use chrono::Utc;
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
pub enum Environment {
    Production,
    Stage,
    Development,
    Test,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub env: Environment,
    pub secrets: Key,
    pub postgresql: PostgreSqlConfig,
    pub redis: RedisConfig,
    pub rabbitmq: RabbitMQConfig,
    pub http: Http,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub port: u16,
    pub workers: usize,
    pub upload: Upload,
    pub origin: String,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 1 << 3,
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
