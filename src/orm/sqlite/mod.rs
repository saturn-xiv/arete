pub mod schema;

use std::default::Default;
use std::fmt;
use std::path::Path;
use std::time::Duration;

use diesel::{connection::SimpleConnection, SqliteConnection};

use super::super::errors::Result;

/// .show 	Displays current settings for various parameters
/// .databases 	Provides database names and files
/// .quit 	Quit sqlite3 program
/// .tables 	Show current tables
/// .schema 	Display schema of table
/// .header 	Display or hide the output table header
/// .mode 	Select mode for the output table
/// .dump 	Dump database in SQL text format
/// pragma compile_options;
/// SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
pub type Connection = SqliteConnection;
pub type ID = i32;

pub const UP: &'static str = include_str!("up.sql");

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file: format!("{}", Path::new("tmp").join("db").display()),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.file)
    }
}

pub fn schema_migrations_exists(name: &str) -> String {
    format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
        name
    )
}

// https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
pub trait Pragma {
    fn busy_timeout(&self, d: Duration) -> Result<()>;
    fn wal_mode(&self, busy_timeout: Duration) -> Result<()>;
}

impl Pragma for Connection {
    fn busy_timeout(&self, d: Duration) -> Result<()> {
        self.batch_execute(&format!(
            "PRAGMA foreign_keys = ON; PRAGMA busy_timeout = {};",
            d.as_micros()
        ))?;
        Ok(())
    }
    fn wal_mode(&self, busy_timeout: Duration) -> Result<()> {
        // NORMAL
        self.batch_execute(&format!(
            "PRAGMA synchronous = OFF; PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON; PRAGMA busy_timeout = {};",
            busy_timeout.as_micros()
        ))?;
        Ok(())
    }
}
