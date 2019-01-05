pub mod routes;
pub mod server;

use std::sync::Arc;

use actix_web::{middleware::Logger, App};

use super::super::{env::Config, plugins::nut};

fn new(cfg: Arc<Config>) -> App<nut::State> {
    App::with_state(nut::State {
        config: cfg.clone(),
    })
    .middleware(Logger::default())
    .resource(r"/3rd/{file:.*}", |r| {
        r.get().f(nut::controllers::html::third)
    })
    .resource(r"/attachments/{file:.*}", |r| {
        r.get().f(nut::controllers::html::attachments)
    })
    .resource(r"/global/{file:.*}", |r| {
        r.get().f(nut::controllers::html::global)
    })
    .resource(r"/assets/{file:.*}", |r| {
        r.get().f(nut::controllers::html::assets)
    })
    .scope(r"/api", |s| {
        s.resource(r"/install", |r| r.post().f(nut::controllers::api::install))
    })
}
