pub mod client;
pub mod models;
pub mod server;

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

use std::path::{Component, Path, PathBuf};

// https://www.wireguard.com/

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new(&Component::RootDir).join("etc").join("openvpn");
}
