use std::ops::Deref;

use diesel::{delete, prelude::*};
use rocket_contrib::json::Json;

use super::super::super::super::super::super::{
    errors::Result,
    orm::{schema::leave_words, Database},
};
use super::super::super::super::{models::leave_words::Item as LeaveWord, request::Administrator};

#[get("/")]
pub fn index(_user: Administrator, db: Database) -> Result<Json<Vec<LeaveWord>>> {
    let db = db.deref();
    let items = leave_words::dsl::leave_words
        .order(leave_words::dsl::created_at.desc())
        .load::<LeaveWord>(db)?;
    Ok(Json(items))
}

#[delete("/leave-words/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> Result<Json<()>> {
    let db = db.deref();
    delete(leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(id))).execute(db)?;
    Ok(Json(()))
}
