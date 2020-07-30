pub mod seo;

use std::ops::Deref;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, Responder};
use handlebars::Handlebars;

use super::super::super::{
    cache::Pool as Cache, crypto::Crypto, errors::Result, orm::Pool as Db, request::Locale, theme,
    VIEWS_ROOT,
};

#[derive(Serialize, Debug)]
pub struct Home {}

#[get("/")]
async fn home(
    db: web::Data<Db>,
    ch: web::Data<Cache>,
    hbs: web::Data<Handlebars<'_>>,
    cyp: web::Data<Crypto>,
    locale: Locale,
) -> Result<impl Responder> {
    let db = db.get()?;
    let db = db.deref();
    let mut ch = ch.get()?;
    let cyp = cyp.deref();
    let cyp = cyp.deref();

    // TODO check homepage

    theme::render(
        "home",
        &locale.0,
        db,
        cyp,
        &mut ch,
        &hbs,
        "home/blog",
        &Home {},
    )
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
