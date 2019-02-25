use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::future;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    rt::{self, Future},
    service::service_fn,
    Body, Method, Response, Server, StatusCode,
};

use super::super::super::{
    env::{self, Config},
    errors::Result,
    graphql::{context::Context, mutation::Mutation, query::Query, session::Session, Schema},
    plugins::{nut::tasks::send_email, ROUTER},
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

    let service = move || {
        let root = root.clone();
        let ctx = ctx.clone();
        service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
            info!("{:?} {} {}", req.version(), req.method(), req.uri());
            let root = root.clone();
            let ctx = ctx.clone();
            let gtx = Arc::new((ctx.clone(), Session::new(&ctx, &req)));

            match (req.method(), req.uri().path()) {
                (&Method::GET, "/doc") => Box::new(juniper_hyper::graphiql(GRAPHQL)),
                (&Method::GET, GRAPHQL) => Box::new(juniper_hyper::graphql(root, gtx, req)),
                (&Method::POST, GRAPHQL) => Box::new(juniper_hyper::graphql(root, gtx, req)),
                _ => {
                    let response = match ROUTER.handle(&ctx, &req) {
                        Ok(v) => match v {
                            Some((t, s)) => {
                                let mut res = Response::new(Body::from(s));
                                match HeaderValue::from_str(&t.to_string()) {
                                    Ok(t) => {
                                        res.headers_mut().insert(CONTENT_TYPE, t);
                                    }
                                    Err(e) => {
                                        error!("{}", e);
                                    }
                                }
                                *res.status_mut() = StatusCode::OK;
                                res
                            }
                            None => {
                                let mut res = Response::new(Body::empty());
                                *res.status_mut() = StatusCode::NOT_FOUND;
                                res
                            }
                        },
                        Err(e) => {
                            error!("{}", e);
                            let mut res = Response::new(Body::from(e.to_string()));
                            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            res
                        }
                    };

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
