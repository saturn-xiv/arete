use std::ops::Deref;

use rocket_contrib::json::Json;

use super::super::super::super::super::super::{errors::Result, orm::Database};
use super::super::super::super::{
    models::leave_word::{Dao as LeaveWordDao, Item as LeaveWord},
    request::Administrator,
};

#[get("/admin/leave-words")]
pub fn index(_user: Administrator, db: Database) -> Result<Json<Vec<LeaveWord>>> {
    let db = db.deref();
    let items = LeaveWordDao::all(db, 1 << 12)?;
    Ok(Json(items))
}

#[delete("/admin/leave-words/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> Result<Json<()>> {
    let db = db.deref();
    LeaveWordDao::delete(db, &id)?;
    Ok(Json(()))
}
