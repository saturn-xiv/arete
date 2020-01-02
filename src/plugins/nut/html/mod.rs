pub mod users;

use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use mime::TEXT_HTML_UTF_8;

use super::super::super::{env::Theme, errors::Result};

#[derive(Template)]
#[template(path = "bootstrap/index.html")]
struct BootstrapIndex;

#[derive(Template)]
#[template(path = "materialize/index.html")]
struct MaterializeIndex;

#[derive(Template)]
#[template(path = "bulma/index.html")]
struct BulmaIndex;

#[derive(Template)]
#[template(path = "semantic-ui/index.html")]
struct SemanticUiIndex;

#[get("/")]
async fn index(theme: web::Data<Theme>) -> Result<impl Responder> {
    Ok(__html!(
        **theme,
        BootstrapIndex,
        BulmaIndex,
        SemanticUiIndex,
        MaterializeIndex
    ))
}

#[get("/rss/{lang}")]
async fn rss(params: web::Path<String>) -> impl Responder {
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
    format!("robots txt")
}
