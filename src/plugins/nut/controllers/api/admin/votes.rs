use std::ops::Deref;

use rocket_contrib::json::Json;

use super::super::super::super::super::super::{errors::JsonResult, orm::Database};
use super::super::super::super::{
    models::tag::{Dao as VoteDao, Item as Vote},
    request::Administrator,
};

#[get("/admin/votes")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Vote>> {
    let db = db.deref();
    let it = VoteDao::all(db)?;
    Ok(Json(it))
}

#[delete("/admin/votes/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> JsonResult<()> {
    let db = db.deref();
    VoteDao::delete(db, &id)?;
    Ok(Json(()))
}
