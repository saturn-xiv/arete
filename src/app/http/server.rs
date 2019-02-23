use std::sync::Arc;
use std::thread;
use std::time::Duration;

use super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    env::{self, Config},
    errors::Result,
    jwt::Jwt,
    plugins::nut::tasks::send_email,
    queue::Queue,
};

pub fn launch(cfg: Config) -> Result<()> {
    let queue = Arc::new(cfg.rabbitmq.clone().open()?);
    let jwt = Arc::new(Jwt::new(cfg.secrets.0.clone()));
    let dbp = cfg.postgresql.open()?;
    let enc = Arc::new(Sodium::new(cfg.secrets.clone())?);

    info!("start send email thread");
    {
        let queue = queue.clone();
        let dbp = dbp.clone();
        let enc = enc.clone();
        thread::spawn(move || loop {
            if let Ok(e) = queue.consume(
                format!("{}-{}-{}", env::NAME, env::VERSION, send_email::NAME),
                send_email::NAME.to_string(),
                Box::new(send_email::Consumer::new(dbp.clone(), enc.clone())),
            ) {
                error!("send email thread failed {:?}", e);
            }
            thread::sleep(Duration::from_secs(30));
        });
    }
    // TODO
    Ok(())
}
