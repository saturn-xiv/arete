pub mod migration;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgresql")]
pub mod postgresql;

#[cfg(feature = "sqlite")]
pub mod sqlite;

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
