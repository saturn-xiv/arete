use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::super::{
    models::category::{Dao as CategoryDao, Item as Tag},
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
    pub position: i16,
    pub parent: Option<i64>,
}

#[get("/admin/categories")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Tag>> {
    let db = db.deref();
    let it = CategoryDao::all(db)?;
    Ok(Json(it))
}

#[get("/admin/categories/<id>")]
pub fn show(_user: Administrator, id: i64, db: Database) -> JsonResult<Tag> {
    let db = db.deref();
    let it = CategoryDao::by_id(db, &id)?;
    Ok(Json(it))
}

#[post("/admin/categories", format = "json", data = "<form>")]
pub fn create(_user: Administrator, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CategoryDao::create(
        db,
        &form.parent,
        &form.name,
        &form.icon,
        &form.color,
        form.position,
    )?;
    Ok(Json(()))
}

#[post("/admin/categories/<id>", format = "json", data = "<form>")]
pub fn update(_user: Administrator, id: i64, form: Json<Form>, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CategoryDao::update(
        db,
        &id,
        &form.parent,
        &form.name,
        &form.icon,
        &form.color,
        form.position,
    )?;
    Ok(Json(()))
}

#[delete("/admin/categories/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, Error, _>(|| {
        CategoryDao::delete(db, &id)?;
        Ok(())
    })?;
    Ok(Json(()))
}
