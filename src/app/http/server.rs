use std::net::SocketAddr;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{
    http::{header, Method},
    middleware::Logger,
    web, App, HttpServer,
};
use chrono::Duration as ChronoDuration;

use super::super::super::{
    crypto::Crypto,
    env::{Config, NAME},
    errors::Result,
    jwt::Jwt,
    plugins::nut,
    storage::fs::FileSystem,
};

#[actix_rt::main]
pub async fn launch(cfg: Config) -> Result<()> {
    let db = cfg.database.open()?;

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

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.http.port));
    let cookie = {
        let key: Result<Vec<u8>> = cfg.secrets.clone().into();
        key?
    };
    let origin = cfg.http.origin.clone();
    let theme = cfg.http.theme.clone();
    let jwt = Jwt::new(cfg.secrets.0.clone());
    let enc = Crypto::new(cfg.secrets.clone())?;
    let mq = cfg.rabbitmq.open();
    let che = cfg.cache.open()?;

    HttpServer::new(move || {
        App::new()
            .data(theme.clone())
            .data(db.clone())
            .data(che.clone())
            .data(enc.clone())
            .data(jwt.clone())
            .data(mq.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin(&origin)
                    .allowed_methods(vec![
                        Method::GET,
                        Method::POST,
                        Method::PUT,
                        Method::PATCH,
                        Method::DELETE,
                    ])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                    ])
                    .supports_credentials()
                    .max_age(60 * 60)
                    .finish(),
            )
            .wrap(
                CookieSession::signed(&cookie)
                    .name(NAME)
                    .http_only(true)
                    .max_age_time(ChronoDuration::hours(1))
                    .path("/")
                    .secure(false),
            )
            .service(
                web::scope("/api")
                    .service(nut::api::install)
                    .service(nut::api::about)
                    .service(nut::api::users::sign_in)
                    .service(nut::api::users::sign_up)
                    .service(nut::api::users::confirm)
                    .service(nut::api::users::unlock)
                    .service(nut::api::users::forgot_password)
                    .service(nut::api::users::confirm_by_token)
                    .service(nut::api::users::unlock_by_token)
                    .service(nut::api::users::reset_password)
                    .service(nut::api::users::index)
                    .service(nut::api::users::self_)
                    .service(nut::api::users::logs)
                    .service(nut::api::users::profile)
                    .service(nut::api::users::change_password)
                    .service(nut::api::users::sign_out),
            )
            .service(nut::html::index)
            .service(nut::html::rss)
            .service(nut::html::robots_txt)
            .service(nut::html::sitemap_xml_gz)
            .service(nut::html::users::index)
            .service(nut::html::users::show)
            .service(actix_files::Files::new("/3rd", "node_modules").use_last_modified(true))
            .service(actix_files::Files::new("/assets", "assets").use_last_modified(true))
            .service(actix_files::Files::new("/upload", FileSystem::root()).use_last_modified(true))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
