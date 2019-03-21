pub mod routes;
pub mod server;

use rocket::{config::Config, custom, Rocket};

use super::super::{
    catchers::catchers,
    graphql,
    plugins::{forum, nut, wiki},
};

pub fn rocket(cfg: Config) -> Rocket {
    custom(cfg)
        .mount(graphql::GRAPHQL, routes![graphql::get, graphql::post])
        .mount(
            "/forum",
            routes![
                forum::html::index,
                forum::html::posts::index,
                forum::html::posts::show,
                forum::html::topics::index,
                forum::html::topics::show
            ],
        )
        .mount("/wiki", routes![wiki::html::show, wiki::html::index])
        .mount(
            "/",
            routes![nut::html::about, nut::html::contact, nut::html::index],
        )
        .register(catchers())
}
