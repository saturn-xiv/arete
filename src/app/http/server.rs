use std::sync::Arc;

use actix_web::server::HttpServer;

use super::super::super::{env::Config, env::NAME, errors::Result};
use super::new;

pub fn launch(cfg: Config) -> Result<()> {
    let sys = actix::System::new(NAME);

    let cfg = Arc::new(cfg);
    let dbp = cfg.postgresql()?;

    let src = cfg.clone();
    HttpServer::new(move || new(src.clone(), dbp.clone()).finish())
        .bind(cfg.http.address())?
        .shutdown_timeout(60)
        .workers(cfg.http.workers)
        .keep_alive(cfg.http.keep_alive)
        .start();

    let _ = sys.run();

    Ok(())
}
