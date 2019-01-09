use std::sync::Arc;
use std::thread;
use std::time::Duration;

use super::super::super::{
    crypto::sodium::Encryptor as Sodium,
    env::{self, Config},
    errors::Result,
    jwt::Jwt,
    orm::Database,
    plugins::nut::tasks::send_email,
    queue::Queue,
    redis::Redis,
};

pub fn launch(cfg: Config) -> Result<()> {
    let queue = Arc::new(cfg.rabbitmq.clone().open()?);
    let jwt = Arc::new(Jwt::new(cfg.secrets.clone()));
    let dbp = cfg.database()?;
    let enc = Arc::new(Sodium::new(&cfg.secrets()?)?);

    info!("start send email thread");
    {
        let queue = queue.clone();
        let dbp = dbp.clone();
        let enc = enc.clone();
        thread::spawn(move || loop {
            if let Ok(e) = queue.consume(
                format!("{}-{}-{}", env::NAME, env::version(), send_email::NAME),
                send_email::NAME.to_string(),
                Box::new(send_email::Consumer::new(dbp.clone(), enc.clone())),
            ) {
                error!("send email thread failed {:?}", e);
            }
            thread::sleep(Duration::from_secs(30));
        });
    }
    let app = super::rocket(cfg.rocket()?)
        .manage(jwt.clone())
        .manage(enc.clone())
        .manage(queue.clone())
        .attach(Database::fairing())
        .attach(Redis::fairing());;
    let err = app.launch();
    Err(err.into())
}
