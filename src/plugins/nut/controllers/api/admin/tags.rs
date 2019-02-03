use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::super::{
    models::tag::{Dao as TagDao, Item as Tag},
    request::Administrator,
};

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub icon: String,
    #[validate(length(min = "1"))]
    pub color: String,
}

#[get("/admin/tags")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Tag>> {
    let db = db.deref();
    let it = TagDao::all(db)?;
    Ok(Json(it))
}

#[get("/admin/tags/<id>")]
pub fn show(_user: Administrator, id: i64, db: Database) -> JsonResult<Tag> {
    let db = db.deref();
    let it = TagDao::by_id(db, &id)?;
    Ok(Json(it))
}

#[post("/admin/tags", format = "json", data = "<form>")]
pub fn create(_user: Administrator, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    TagDao::create(db, &form.name, &form.icon, &form.color)?;
    Ok(Json(()))
}

#[post("/admin/tags/<id>", format = "json", data = "<form>")]
pub fn update(_user: Administrator, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    TagDao::update(db, &id, &form.name, &form.icon, &form.color)?;
    Ok(Json(()))
}

#[delete("/admin/tags/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, Error, _>(|| {
        TagDao::delete(db, &id)?;
        Ok(())
    })?;
    Ok(Json(()))
}
