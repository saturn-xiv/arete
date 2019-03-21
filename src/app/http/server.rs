use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use super::super::super::{
    env::{self, Config, NAME},
    errors::{Error, Result},
    graphql,
    jwt::Jwt,
    orm::Database,
    plugins::{forum, nut, wiki},
    queue::Queue,
    redis::Redis,
};

pub fn launch(cfg: Config) -> Result<()> {
    // let ctx = Arc::new(graphql::context::Context::new(&cfg)?);

    // info!("start send email thread");
    // {
    //     let ctx = ctx.clone();
    //     thread::spawn(move || loop {
    //         if let Ok(e) = ctx.queue.consume(
    //             format!(
    //                 "{}-{}-{}",
    //                 env::NAME,
    //                 env::VERSION,
    //                 nut::tasks::send_email::NAME
    //             ),
    //             nut::tasks::send_email::NAME.to_string(),
    //             Box::new(nut::tasks::send_email::Consumer { ctx: ctx.clone() }),
    //         ) {
    //             error!("send email thread failed {:?}", e);
    //         }
    //         thread::sleep(Duration::from_secs(30));
    //     });
    // }

    let err = super::rocket(cfg.rocket()?)
        .manage(graphql::new())
        .manage(Arc::new(Jwt::new(cfg.secrets.0.clone())))
        .attach(Database::fairing())
        .attach(Redis::fairing())
        .launch();
    Err(err.into())
}
