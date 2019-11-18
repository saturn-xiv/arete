use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
};
use super::super::models::category::{Dao as CategoryDao, Item as Category};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub icon: String,
    #[validate(length(min = 1))]
    pub color: String,
    pub position: i16,
    pub parent: Option<ID>,
}

#[post("/categories", data = "<form>")]
pub fn create(_user: Administrator, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CategoryDao::create(
        db,
        form.parent,
        &form.name,
        &form.icon,
        &form.color,
        form.position,
    )?;
    Ok(Json(()))
}

#[post("/categories/<id>", data = "<form>")]
pub fn update(_user: Administrator, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    CategoryDao::update(
        db,
        id,
        form.parent,
        &form.name,
        &form.icon,
        &form.color,
        form.position,
    )?;
    Ok(Json(()))
}

#[get("/categories/<id>")]
pub fn show(id: ID, _user: Administrator, db: Database) -> JsonResult<Category> {
    let db = db.deref();
    let it = CategoryDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/categories")]
pub fn index(db: Database) -> JsonResult<Vec<Category>> {
    let db = db.deref();
    let items = CategoryDao::all(db)?;
    Ok(Json(items))
}

#[delete("/categories/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, Error, _>(|| {
        CategoryDao::delete(db, id)?;
        Ok(())
    })?;
    Ok(Json(()))
}
