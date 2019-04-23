use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
};
use super::super::models::tag::{Dao as TagDao, Item as Tag};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(length(min = "1"))]
    pub icon: String,
    #[validate(length(min = "1"))]
    pub color: String,
}

#[post("/tags", data = "<form>")]
pub fn create(_user: Administrator, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    TagDao::create(db, &form.name, &form.icon, &form.color)?;
    Ok(Json(()))
}

#[post("/tags/<id>", data = "<form>")]
pub fn update(_user: Administrator, id: ID, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    TagDao::update(db, id, &form.name, &form.icon, &form.color)?;
    Ok(Json(()))
}

#[get("/tags/<id>")]
pub fn show(id: ID, _user: Administrator, db: Database) -> JsonResult<Tag> {
    let db = db.deref();
    let it = TagDao::by_id(db, id)?;
    Ok(Json(it))
}

#[get("/tags")]
pub fn index(db: Database) -> JsonResult<Vec<Tag>> {
    let db = db.deref();
    let items = TagDao::all(db)?;
    Ok(Json(items))
}

#[delete("/tags/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, Error, _>(|| {
        TagDao::delete(db, id)?;
        Ok(())
    })?;
    Ok(Json(()))
}
