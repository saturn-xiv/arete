use std::sync::Arc;
use std::thread;
use std::time::Duration;

use super::super::super::{
    crypto::Crypto,
    env::{self, Config},
    errors::Result,
    jwt::Jwt,
    plugins::nut,
};

#[actix_rt::main]
pub async fn launch(cfg: Config) -> Result<()> {
    let db = cfg.database.open()?;
    let jwt = Arc::new(Jwt::new(cfg.secrets.0.clone()));
    let enc = Arc::new(Crypto::new(cfg.secrets.clone())?);

    info!("start send email thread");
    // {
    //     let db = db.clone();
    //     let enc = enc.clone();
    //     let db = db.clone();
    //     let qu = qu.clone();
    //     thread::spawn(move || loop {
    //         if let Ok(e) = qu.consume(
    //             format!(
    //                 "{}-{}-{}",
    //                 env::NAME,
    //                 env::VERSION,
    //                 nut::tasks::send_email::NAME
    //             ),
    //             nut::tasks::send_email::NAME.to_string(),
    //             Box::new(nut::tasks::send_email::Consumer {
    //                 db: db.clone(),
    //                 encryptor: enc.clone(),
    //             }),
    //         ) {
    //             error!("send email thread failed {:?}", e);
    //         }
    //         thread::sleep(Duration::from_secs(30));
    //     });
    // }

    // let err = super::rocket(cfg.rocket()?)
    //     .manage(jwt)
    //     .manage(enc)
    //     .manage(qu)
    //     .attach(Database::fairing())
    //     .attach(Cache::fairing())
    //     .launch();
    // let mut rt = custom(cfg)
    // .mount("/3rd", StaticFiles::from("node_modules"))
    // .mount("/assets", StaticFiles::from("assets"))
    // .mount("/upload", StaticFiles::from(FileSystem::root()));

    Ok(())
}
