pub mod schema;

use std::default::Default;
use std::fmt;
use std::path::Path;

use diesel::{prelude::*, sql_query, sqlite::SqliteConnection};

use super::super::errors::Result;

/// .show 	Displays current settings for various parameters
/// .databases 	Provides database names and files
/// .quit 	Quit sqlite3 program
/// .tables 	Show current tables
/// .schema 	Display schema of table
/// .header 	Display or hide the output table header
/// .mode 	Select mode for the output table
/// .dump 	Dump database in SQL text format
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

pub fn set_timeout(db: &Connection, timeout: u8) -> Result<()> {
    sql_query(&format!("PRAGMA busy_timeout = {}", timeout as u16 * 1000)).execute(db)?;
    Ok(())
}
