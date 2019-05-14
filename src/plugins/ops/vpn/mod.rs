/// https://help.ubuntu.com/lts/serverguide/openvpn.html.en
/// https://zshmobi.com/2018/06/11/fast-reverse-proxy-and-l2tp-vpn-setting/
/// https://zshmobi.com/2018/06/14/openvpn-deployment/
/// http://chagridsada.blogspot.com/2011/01/openvpn-system-based-on-userpass.html
pub mod api;
pub mod client;
pub mod models;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgresql")]
pub mod postgresql;
pub mod server;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use std::path::{Component, Path, PathBuf};

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new(&Component::RootDir).join("etc").join("openvpn");
}

use rocket::Route;

pub fn routes() -> (&'static str, Vec<Route>, Vec<Route>) {
    (
        "ops/vpn",
        routes![
            api::users::create,
            api::users::update,
            api::users::change_password,
            api::users::sign_in,
            api::users::connect,
            api::users::disconnect,
            api::logs::index,
        ],
        routes![],
    )
}
