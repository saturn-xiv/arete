use std::ops::Deref;

use actix_web::{web, Responder};
use handlebars::Handlebars;

use super::super::super::super::{
    cache::Pool as Cache, crypto::Crypto, errors::Result, orm::Pool as Db, request::Locale, theme,
};

#[get("/posts")]
pub async fn index(
    db: web::Data<Db>,
    lng: Locale,
    cyp: web::Data<Crypto>,
    ch: web::Data<Cache>,
    hbs: web::Data<Handlebars<'_>>,
) -> Result<impl Responder> {
    let db = db.get()?;
    let db = db.deref();
    let mut ch = ch.get()?;
    let cyp = cyp.deref();
    let cyp = cyp.deref();

    let data = json!({
        "name": "Handlebars"
    });
    theme::render(
        "forum.posts.index",
        &lng.0,
        db,
        cyp,
        &mut ch,
        &hbs,
        "wiki/show",
        &data,
    )
}
