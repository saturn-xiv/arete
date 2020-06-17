pub mod schema;

use std::default::Default;
use std::fmt;

/// use DB-NAME
/// show tables;
/// desc TABLE-NAME;
/// SELECT table_name FROM information_schema.tables WHERE table_schema = 'databasename' AND table_name = 'testtable';
/// SHOW TABLES LIKE 'tablename';
pub type Connection = diesel::mysql::MysqlConnection;
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
            port: 3306,
            user: "root".to_string(),
            name: "dev".to_string(),
            password: None,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "mysql://{}:{}@{}:{}/{}",
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
        "SELECT table_name AS name FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = '{}'",
    name
    )
}
