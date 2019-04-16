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

use rocket::Rocket;

pub fn mount(rt: Rocket) -> Rocket {
    rt.mount(
        "/api/forum",
        routes![
            api::posts::create,
            api::posts::update,
            api::posts::destroy,
            api::posts::show,
            api::posts::index,
            api::topics::create,
            api::topics::update,
            api::topics::destroy,
            api::topics::show,
            api::topics::index,
        ],
    )
    .mount(
        "/forum",
        routes![
            html::index,
            html::posts::index,
            html::posts::show,
            html::topics::index,
            html::topics::show,
        ],
    )
}
