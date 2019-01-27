use std::net::SocketAddr;
use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::{models::leave_word::Dao as LeaveWordDao, MediaType};

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct New {
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

#[post("/leave-words", format = "json", data = "<form>")]
pub fn create(form: Json<New>, remote: SocketAddr, db: Database) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let ip = remote.ip();
    LeaveWordDao::add(db, &ip, &form.body, &form.media_type.parse::<MediaType>()?)?;
    Ok(Json(()))
}
