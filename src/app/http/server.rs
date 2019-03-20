use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use actix::{prelude::*, System};
use actix_web::{
    fs::StaticFiles,
    http::{
        header::{ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware::{cors::Cors, Logger},
    server, App,
};

use super::super::super::{
    env::{self, Config, NAME},
    errors::Result,
    graphql,
    plugins::{forum, nut, wiki},
    queue::Queue,
    request::{ClientIp, Home},
};
use super::State as AppState;

pub fn launch(cfg: Config) -> Result<()> {
    let ctx = Arc::new(graphql::context::Context::new(&cfg)?);

    info!("start send email thread");
    {
        let ctx = ctx.clone();
        thread::spawn(move || loop {
            if let Ok(e) = ctx.queue.consume(
                format!(
                    "{}-{}-{}",
                    env::NAME,
                    env::VERSION,
                    nut::tasks::send_email::NAME
                ),
                nut::tasks::send_email::NAME.to_string(),
                Box::new(nut::tasks::send_email::Consumer { ctx: ctx.clone() }),
            ) {
                error!("send email thread failed {:?}", e);
            }
            thread::sleep(Duration::from_secs(30));
        });
    }

    let sys = System::new(NAME);

    let gql = Arc::new(graphql::new());
    let gql = SyncArbiter::start(3, move || graphql::actix::GraphQLExecutor {
        schema: gql.clone(),
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.http.port));
    let origin = cfg.http.origin.clone();

    server::new(move || {
        App::with_state(AppState {
            graphql: gql.clone(),
            context: ctx.clone(),
        })
        .middleware(Logger::default())
        .handler("/3rd", StaticFiles::new(Path::new("node_modules")).unwrap())
        .handler("/assets", StaticFiles::new(Path::new("assets")).unwrap())
        .handler(
            "/upload",
            StaticFiles::new(Path::new("tmp").join("upload")).unwrap(),
        )
        .scope("/wiki", |s| {
            s.resource("/", |r| r.method(Method::GET).with(wiki::html::index))
                .resource("/{name:.*}", |r| {
                    r.method(Method::GET).with(wiki::html::show)
                })
        })
        .scope("/forum", |s| {
            s.resource("/topics", |r| {
                r.method(Method::GET).with(forum::html::topics::index)
            })
            .resource("/topics/{id}", |r| {
                r.method(Method::GET).with(forum::html::topics::show)
            })
            .resource("/posts", |r| {
                r.method(Method::GET).with(forum::html::posts::index)
            })
            .resource("/posts/{id}", |r| {
                r.method(Method::GET).with(forum::html::posts::show)
            })
            .resource("/", |r| r.method(Method::GET).with(forum::html::index))
        })
        .resource("/about", |r| r.method(Method::GET).with(nut::html::about))
        .resource("/contact", |r| {
            r.method(Method::GET).with(nut::html::contact)
        })
        .resource("/", |r| {
            r.name(Home::KEY);
            r.method(Method::GET).with(nut::html::index)
        })
        .configure(|app| {
            Cors::for_app(app)
                .allowed_origin(&origin)
                .allowed_methods(vec![Method::POST])
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT_LANGUAGE, ACCEPT])
                .allowed_headers(vec![
                    ClientIp::X_REAL_IP,
                    ClientIp::X_FORWARDED_FOR,
                    ClientIp::X_APPENGINE_REMOTE_ADDR,
                ])
                .supports_credentials()
                .max_age(60 * 60)
                .resource("/graphql", |r| {
                    r.method(Method::GET).h(graphql::actix::doc);
                    r.method(Method::POST).with(graphql::actix::post)
                })
                .register()
        })
    })
    .bind(&addr)?
    .start();

    let _ = sys.run();

    Ok(())
}
