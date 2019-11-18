use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
};
use super::super::models::link::{Dao as LinkDao, Item as Link};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = 1))]
    pub lang: String,
    #[validate(length(min = 1))]
    pub label: String,
    #[validate(length(min = 1))]
    pub href: String,
    #[validate(length(min = 1))]
    pub loc: String,
    pub x: i16,
    pub y: i16,
}

#[post("/links", data = "<form>")]
pub fn create(_user: Administrator, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LinkDao::create(
        db,
        &form.lang,
        &form.label,
        &form.href,
        &form.loc,
        form.x,
        form.y,
    )?;
    Ok(Json(()))
}

#[post("/links/<id>", data = "<form>")]
pub fn update(_user: Administrator, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LinkDao::update(
        db,
        id,
        &form.lang,
        &form.label,
        &form.href,
        &form.loc,
        form.x,
        form.y,
    )?;
    Ok(Json(()))
}

#[get("/links/<id>")]
pub fn show(id: ID, db: Database) -> JsonResult<Link> {
    let db = db.deref();
    let it = LinkDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/links")]
pub fn index(db: Database) -> JsonResult<Vec<Link>> {
    let db = db.deref();
    let items = LinkDao::all(db)?;
    Ok(Json(items))
}

#[delete("/links/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    LinkDao::delete(db, id)?;
    Ok(Json(()))
}
