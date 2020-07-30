pub mod migration;
pub mod mysql;
pub mod postgresql;
pub mod sqlite;

use diesel::{sql_query, sql_types::Text, RunQueryDsl};

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

use super::errors::Result;

pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>;
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

impl Config {
    pub fn open(&self) -> Result<Pool> {
        let manager = diesel::r2d2::ConnectionManager::<Connection>::new(&self.to_string()[..]);
        Ok(Pool::new(manager)?)
    }
}

pub trait Dao {
    fn version(&self) -> Result<String>;
}

#[derive(QueryableByName)]
pub struct Version {
    #[sql_type = "Text"]
    pub value: String,
}

impl Dao for Connection {
    fn version(&self) -> Result<String> {
        let it: Version = sql_query(VERSION).get_result(self)?;
        Ok(it.value)
    }
}
