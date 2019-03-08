use std::sync::Arc;
use std::thread;
use std::time::Duration;

use actix::{prelude::*, System};
use actix_web::{
    http, middleware, server, App, AsyncResponder, Error, FutureResponse, HttpRequest,
    HttpResponse, Json, State,
};
use futures::future::Future;

use super::super::super::{
    env::{self, Config, NAME},
    errors::Result,
    graphql::{
        actix::{doc as graphiql_doc, post as graphiql_post, GraphQLExecutor},
        // context::Context,
        new as create_schema,
    },
    http::Response,
    orm::DbExecutor,
    plugins::nut::tasks::send_email,
    queue::Queue,
    redis::CacheExecutor,
};
use super::State as AppState;

pub fn launch(cfg: Config) -> Result<()> {
    // let ctx = Arc::new(Context::new(&cfg)?);

    // info!("start send email thread");
    // {
    //     let ctx = ctx.clone();
    //     thread::spawn(move || loop {
    //         if let Ok(e) = ctx.queue.consume(
    //             format!("{}-{}-{}", env::NAME, env::VERSION, send_email::NAME),
    //             send_email::NAME.to_string(),
    //             Box::new(send_email::Consumer { ctx: ctx.clone() }),
    //         ) {
    //             error!("send email thread failed {:?}", e);
    //         }
    //         thread::sleep(Duration::from_secs(30));
    //     });
    // }

    let sys = System::new(NAME);

    let cache = cfg.redis.open()?;
    let db = cfg.postgresql.open()?;
    let graphql = Arc::new(create_schema());
    let (gdb, gch) = (db.clone(), cache.clone());
    let graphql = SyncArbiter::start(3, move || GraphQLExecutor {
        schema: graphql.clone(),
        db: gdb.clone(),
        cache: gch.clone(),
    });

    let db = SyncArbiter::start(3, move || DbExecutor(db.clone()));
    let cache = SyncArbiter::start(3, move || CacheExecutor(cache.clone()));

    server::new(move || {
        App::with_state(AppState {
            graphql: graphql.clone(),
            db: db.clone(),
            cache: cache.clone(),
        })
        .middleware(middleware::Logger::default())
        .resource("/graphql", |r| {
            r.method(http::Method::POST).with(graphiql_post)
        })
        .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql_doc))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    // let root = Arc::new(new_schema);

    // let service = move || {
    //     let root = root.clone();
    //     let ctx = ctx.clone();
    //     service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
    //         info!("{:?} {} {}", req.version(), req.method(), req.uri());
    //         let root = root.clone();
    //         let ctx = ctx.clone();
    //         let gtx = Arc::new((ctx.clone(), Session::new(&ctx, &req)));

    //         match (req.method(), req.uri().path()) {
    //             (&Method::GET, "/doc") => Box::new(juniper_hyper::graphiql(GRAPHQL)),
    //             (&Method::GET, GRAPHQL) => Box::new(juniper_hyper::graphql(root, gtx, req)),
    //             (&Method::POST, GRAPHQL) => Box::new(juniper_hyper::graphql(root, gtx, req)),
    //             _ => {
    //                 let res = match ROUTER.handle(&ctx, &req) {
    //                     Ok(v) => match v {
    //                         Some(r) => r,
    //                         None => Response::NotFound,
    //                     },
    //                     Err(e) => Response::InternalServerError(e),
    //                 };

    //                 Box::new(future::ok(res.into()))
    //             }
    //         }
    //     })
    // };

    // let addr = ([127, 0, 0, 1], cfg.http.port).into();
    // let server = Server::bind(&addr)
    //     .serve(service)
    //     .map_err(|e| error!("server error: {}", e));
    // info!("listening on http://{}", addr);
    // rt::run(server);

    Ok(())
}

// const GRAPHQL: &'static str = "/graphql";
