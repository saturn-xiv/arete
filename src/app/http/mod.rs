pub mod routes;
pub mod server;

use rocket::{config::Config, custom, Rocket};

use super::super::{catchers::catchers, plugins::nut};

pub fn rocket(cfg: Config) -> Rocket {
    custom(cfg)
        .mount(
            "/",
            routes![nut::html::about, nut::html::contact, nut::html::index],
        )
        .register(catchers())
}
