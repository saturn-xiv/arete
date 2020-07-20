pub mod helpers;

use std::ffi::OsStr;
use std::fs::read_dir;
use std::net::SocketAddr;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{
    http::{header, Method},
    middleware::Logger,
    web, App, HttpServer,
};
use chrono::Duration as ChronoDuration;
use handlebars::Handlebars;

use super::super::super::{
    crypto::Crypto,
    env::{Config, Environment, NAME},
    errors::Result,
    graphql,
    jwt::Jwt,
    plugins::nut,
    VIEWS_ROOT,
};

#[actix_rt::main]
pub async fn launch(cfg: Config) -> Result<()> {
    let db = cfg.database.open()?;

    info!("load theme files");
    let mut handlebars = Handlebars::new();
    {
        handlebars.set_strict_mode(true);
        {
            handlebars.register_helper("lower", Box::new(helpers::lower));
            handlebars.register_helper("upper", Box::new(helpers::upper));
            handlebars.register_helper("hex", Box::new(helpers::hex));
            handlebars.register_helper("money", Box::new(helpers::money));
        }
        for entry in read_dir("helpers")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new("rhai")) {
                if let Some(name) = path.file_stem() {
                    if let Some(name) = name.to_str() {
                        debug!("load template helper {}", name);
                        handlebars.register_script_helper_file(name, &path)?;
                    }
                }
            }
        }
        handlebars.register_templates_directory(".hbs", VIEWS_ROOT)?;
    }
    let handlebars = web::Data::new(handlebars);

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
    let jwt = Jwt::new(cfg.secrets.0.clone());
    let enc = Crypto::new(cfg.secrets.clone())?;
    let mq = cfg.rabbitmq.open();
    let che = cfg.cache.open()?;
    let schema = web::Data::new(graphql::Schema::new(
        graphql::query::Root {},
        graphql::mutation::Root {},
    ));
    let env = cfg.env.clone();

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(che.clone())
            .data(enc.clone())
            .data(jwt.clone())
            .data(mq.clone())
            .app_data(schema.clone())
            .app_data(handlebars.clone())
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(1 << 16))
            .wrap(match env {
                Environment::Production => Cors::new()
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
                    .send_wildcard()
                    .finish(),
                _ => Cors::default(),
            })
            .wrap(
                CookieSession::signed(&cookie)
                    .name(NAME)
                    .http_only(true)
                    .max_age_time(ChronoDuration::hours(1))
                    .path("/")
                    .secure(cfg!(not(debug_assertions))),
            )
            .service(nut::html::home)
            .service(nut::html::assets)
            .service(nut::html::seo::rss)
            .service(nut::html::seo::robots_txt)
            .service(nut::html::seo::sitemap_xml_gz)
            .service(web::resource(graphql::SOURCE).route(web::post().to(graphql::post)))
            .service(web::resource("/graphiql").route(web::get().to(graphql::get)))
            .service(actix_files::Files::new("/3rd", "node_modules").use_last_modified(true))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
