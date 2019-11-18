use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::{
    errors::JsonResult,
    orm::{Database, ID},
    request::ClientIp,
};
use super::super::models::leave_word::{Dao as LeaveWordDao, Item as LeaveWord};
use super::users::Administrator;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(length(min = 1))]
    pub media_type: String,
}

#[post("/leave-words", data = "<form>")]
pub fn create(
    _user: Administrator,
    remote: ClientIp,
    db: Database,
    form: Json<Form>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    LeaveWordDao::add(db, &remote.0, &form.body, &form.media_type.parse()?)?;
    Ok(Json(()))
}

#[get("/leave-words?<limit>")]
pub fn index(_user: Administrator, limit: i64, db: Database) -> JsonResult<Vec<LeaveWord>> {
    let db = db.deref();
    let items = LeaveWordDao::all(db, limit)?;
    Ok(Json(items))
}

#[delete("/leave-words/<id>")]
pub fn destroy(_user: Administrator, id: ID, db: Database) -> JsonResult<()> {
    let db = db.deref();
    LeaveWordDao::delete(db, id)?;
    Ok(Json(()))
}
