pub mod seo;

use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;
use handlebars::Handlebars;
use mime::TEXT_HTML_UTF_8;

use super::super::super::{errors::Result, VIEWS_ROOT};

#[get("/")]
async fn home(hbs: web::Data<Handlebars<'_>>) -> Result<impl Responder> {
    // TODO get theme and tpl name from db
    let body = hbs.render(
        "bootstrap/views/home/blog",
        &json!({
            "now": Utc::now().naive_local()
        }),
    )?;

    Ok(HttpResponse::Ok()
        .content_type(TEXT_HTML_UTF_8.to_string())
        .body(body))
}

#[get("/assets/{theme}/{file:.*}")]
async fn assets(params: web::Path<(String, String)>) -> Result<impl Responder> {
    let file = Path::new(VIEWS_ROOT)
        .join(&params.0)
        .join("assets")
        .join(&params.1);
    let file = NamedFile::open(&file)?;
    Ok(file.use_last_modified(true))
}
