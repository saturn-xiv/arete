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
    pub workers: usize,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 1 << 3,
        }
    }
}
