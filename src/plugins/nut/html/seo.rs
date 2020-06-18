use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;

use super::super::super::super::errors::Result;

#[get("/rss/{lang}")]
async fn rss(params: web::Path<String>) -> impl Responder {
    // TODO
    format!("lang {}", params)
}

#[derive(Template)]
#[template(path = "robots.txt", escape = "none")]
struct RobotsTxt;

// http://www.robotstxt.org/
#[get("/robots.txt")]
async fn robots_txt() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN_UTF_8.to_string())
        .body(RobotsTxt.render()?))
}

#[get("/sitemap.xml.gz")]
async fn sitemap_xml_gz() -> impl Responder {
    // TODO
    "sitemap.xml.gz"
}
