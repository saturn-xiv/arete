use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::future;
use futures_cpupool::CpuPool;
use hyper::{
    rt::{self, Future},
    service::service_fn,
    Body, Method, Response, Server, StatusCode,
};

use super::super::super::{
    env::{self, Config},
    errors::Result,
    graphql::{context::Context, mutation::Mutation, query::Query, Schema},
    plugins::nut::tasks::send_email,
    queue::Queue,
};

pub fn launch(cfg: Config) -> Result<()> {
    let ctx = Arc::new(Context::new(&cfg)?);

    info!("start send email thread");
    {
        let ctx = ctx.clone();
        thread::spawn(move || loop {
            if let Ok(e) = ctx.queue.consume(
                format!("{}-{}-{}", env::NAME, env::VERSION, send_email::NAME),
                send_email::NAME.to_string(),
                Box::new(send_email::Consumer { ctx: ctx.clone() }),
            ) {
                error!("send email thread failed {:?}", e);
            }
            thread::sleep(Duration::from_secs(30));
        });
    }

    let root = Arc::new(Schema::new(Query, Mutation));
    let pool = CpuPool::new(cfg.http.workers);
    let service = move || {
        let root = root.clone();
        let ctx = ctx.clone();
        let pool = pool.clone();
        service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
            let root = root.clone();
            let ctx = ctx.clone();
            let pool = pool.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/doc") => Box::new(juniper_hyper::graphiql(GRAPHQL)),
                (&Method::GET, GRAPHQL) => Box::new(juniper_hyper::graphql(pool, root, ctx, req)),
                (&Method::POST, GRAPHQL) => Box::new(juniper_hyper::graphql(pool, root, ctx, req)),
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };

    let addr = ([127, 0, 0, 1], cfg.http.port).into();
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| error!("server error: {}", e));
    info!("listening on http://{}", addr);
    rt::run(server);

    Ok(())
}

const GRAPHQL: &'static str = "/graphql";
