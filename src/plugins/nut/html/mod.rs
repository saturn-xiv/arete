pub mod users;

use actix_web::{get, web, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("home")
}

#[get("/rss/{lang}")]
async fn rss(params: web::Path<String>) -> impl Responder {
    format!("lang {}", params)
}

#[get("/robots.txt")]
async fn robots_txt() -> impl Responder {
    format!("robots txt")
}

#[get("/sitemap.xml.gz")]
async fn sitemap_xml_gz() -> impl Responder {
    format!("robots txt")
}
