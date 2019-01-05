pub mod routes;
pub mod server;

use std::sync::Arc;

use actix::prelude::*;
use actix_web::{middleware::Logger, App};

use super::super::{
    env::Config,
    orm::{DbExecutor, Pool as DbPool},
    plugins::nut,
};

fn new(cfg: Arc<Config>, db: DbPool) -> App<nut::State> {
    let db = SyncArbiter::start(3, move || DbExecutor(db.clone()));
    App::with_state(nut::State {
        config: cfg.clone(),
        db: db.clone(),
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
            .nested(r"/leave-words", |s| {
                s.resource(r"/new", |r| {
                    r.post().f(nut::controllers::api::leave_words::create)
                })
            })
    })
}
