use std::path::{Path, PathBuf};

use super::{
    crypto::Key, orm::Config as PostgreSqlConfig, queue::rabbitmq::Config as RabbitMQConfig,
    redis::Config as RedisConfig,
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
