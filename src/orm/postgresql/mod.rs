pub mod schema;

use std::default::Default;
use std::fmt;

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
pub type Connection = diesel::pg::PgConnection;
pub type ID = i64;

pub const UP: &str = include_str!("up.sql");

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            name: "dev".to_string(),
            password: None,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            match self.password {
                Some(ref v) => v,
                None => "",
            },
            self.host,
            self.port,
            self.name
        )
    }
}

pub fn schema_migrations_exists(name: &str) -> String {
    format!(
        "SELECT table_name AS name FROM information_schema.tables WHERE table_name = '{}'",
        name
    )
}
