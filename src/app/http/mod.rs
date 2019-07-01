pub mod routes;
pub mod server;

use rocket::{config::Config, custom, Rocket};
use rocket_contrib::serve::StaticFiles;

use super::super::{
    catchers::catchers,
    plugins::{forum, nut, ops::vpn, wiki},
};

pub fn rocket(cfg: Config) -> Rocket {
    let mut rt = custom(cfg)
        .mount("/3rd", StaticFiles::from("node_modules"))
        .mount("/assets", StaticFiles::from("assets"))
        .mount("/upload", StaticFiles::from(FileSystem::root()));

    for (path, api, html) in vec![
        forum::routes(),
        wiki::routes(),
        vpn::routes(),
        nut::routes(),
    ] {
        rt = rt
            .mount(&format!("/api/{}", path), api)
            .mount(&format!("/{}", path), html);
    }

    rt.register(catchers())
}
