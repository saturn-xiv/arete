pub mod routes;
pub mod server;

use rocket::{config::Config, custom, Rocket};

use super::super::{
    catchers::catchers,
    plugins::{forum, nut, wiki},
};

pub fn rocket(cfg: Config) -> Rocket {
    let mut rt = custom(cfg);

    for (path, api, html) in vec![forum::routes(), wiki::routes(), nut::routes()] {
        rt = rt
            .mount(&format!("/api/{}", path), api)
            .mount(&format!("/{}", path), html);
    }

    rt.register(catchers())
}
