pub mod routes;
pub mod server;

use super::super::plugins::{forum, nut, wiki};

fn rocket(cfg: rocket::Config) -> rocket::Rocket {
    let mut routes = Vec::new();
    routes.extend_from_slice(&nut::controllers::ROUTES);
    routes.extend_from_slice(&wiki::controllers::ROUTES);
    routes.extend_from_slice(&forum::controllers::ROUTES);

    let mut app = rocket::custom(cfg);
    for (k, v) in routes {
        app = app.mount(&k, v);
    }

    app.register(nut::catchers::catchers())
}
