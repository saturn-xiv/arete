use std::ops::Deref;

use diesel::Connection as DieselConnection;
use failure::Error;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
};
use super::super::models::{
    user::Item as User,
    vote::{Dao as VoteDao, Item as Vote},
};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
pub struct Form {
    #[validate(length(min = "1"))]
    pub resource_type: String,
    pub resource_id: ID,
    pub like: bool,
}

#[post("/votes", data = "<form>")]
pub fn create(_user: User, db: Database, form: Json<Form>) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    VoteDao::like(db, &form.resource_type, form.resource_id, form.like)?;
    Ok(Json(()))
}

#[get("/votes")]
pub fn index(db: Database) -> JsonResult<Vec<Vote>> {
    let db = db.deref();
    let items = VoteDao::all(db)?;
    Ok(Json(items))
}

#[delete("/votes/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    db.transaction::<_, Error, _>(|| {
        VoteDao::delete(db, id)?;
        Ok(())
    })?;
    Ok(Json(()))
}
