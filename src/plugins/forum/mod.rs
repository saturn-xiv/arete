pub mod api;
pub mod html;
pub mod models;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "sqlite")]
pub mod sqlite;
pub mod themes;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

use rocket::Route;

pub fn routes() -> (&'static str, Vec<Route>, Vec<Route>) {
    (
        "forum",
        routes![
            api::posts::create,
            api::posts::update,
            api::posts::destroy,
            api::posts::show,
            api::posts::index_by_administrator,
            api::posts::index_by_owner,
            api::topics::create,
            api::topics::update,
            api::topics::destroy,
            api::topics::show,
            api::topics::index_by_administrator,
            api::topics::index_by_owner,
        ],
        routes![
            html::index,
            html::posts::index,
            html::posts::show,
            html::topics::index,
            html::topics::show,
        ],
    )
}
