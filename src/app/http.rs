use std::sync::Arc;

use actix_web::{http::Method, middleware::Logger, server, App};

use super::super::{env::Config, errors::Result, plugins::nut};

pub fn run(cfg: Config) -> Result<()> {
    let cfg = Arc::new(cfg);
    let addr = cfg.http.address();

    server::new(move || {
        App::with_state(nut::State {
            config: cfg.clone(),
        })
        .middleware(Logger::default())
        .resource(r"/3rd/{file:.*}", |r| {
            r.method(Method::GET).f(nut::controllers::html::third)
        })
        .resource(r"/attachments/{file:.*}", |r| {
            r.method(Method::GET).f(nut::controllers::html::attachments)
        })
        .resource(r"/global/{file:.*}", |r| {
            r.method(Method::GET).f(nut::controllers::html::global)
        })
        .resource(r"/assets/{file:.*}", |r| {
            r.method(Method::GET).f(nut::controllers::html::assets)
        })
        .finish()
    })
    .bind(addr)?
    .run();
    Ok(())
}
