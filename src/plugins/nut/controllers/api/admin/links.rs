use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::super::{
    models::link::{Dao as LinkDao, Item as Link},
    request::Administrator,
};

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub lang: String,
    #[validate(length(min = "1"))]
    pub label: String,
    #[validate(length(min = "1"))]
    pub href: String,
    #[validate(length(min = "1"))]
    pub loc: String,
    pub x: i16,
    pub y: i16,
}

#[get("/admin/links")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Link>> {
    let db = db.deref();
    let it = LinkDao::all(db)?;
    Ok(Json(it))
}

#[get("/admin/links/<id>")]
pub fn show(_user: Administrator, id: i64, db: Database) -> JsonResult<Link> {
    let db = db.deref();
    let it = LinkDao::by_id(db, &id)?;
    Ok(Json(it))
}

#[post("/admin/links", format = "json", data = "<form>")]
pub fn create(_user: Administrator, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LinkDao::create(
        db,
        &form.lang,
        &form.label,
        &form.href,
        &form.loc,
        &form.x,
        &form.y,
    )?;
    Ok(Json(()))
}

#[post("/admin/links/<id>", format = "json", data = "<form>")]
pub fn update(_user: Administrator, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LinkDao::update(
        db,
        &id,
        &form.lang,
        &form.label,
        &form.href,
        &form.loc,
        &form.x,
        &form.y,
    )?;
    Ok(Json(()))
}

#[delete("/admin/links/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    LinkDao::delete(db, &id)?;
    Ok(Json(()))
}
