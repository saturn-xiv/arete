use std::ops::Deref;

use diesel::{delete, insert_into, prelude::*};
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{
    errors::Result,
    orm::{schema::leave_words, Database},
};
use super::super::super::{
    models::leave_words::Item as LeaveWord, request::Administrator, MediaType,
};

#[get("/")]
pub fn index(_user: Administrator, db: Database) -> Result<Json<Vec<LeaveWord>>> {
    let db = db.deref();
    let items = leave_words::dsl::leave_words
        .order(leave_words::dsl::created_at.desc())
        .load::<LeaveWord>(db)?;
    Ok(Json(items))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct New {
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

#[post("/", format = "json", data = "<form>")]
pub fn create(form: Json<New>, db: Database) -> Result<Json<()>> {
    form.validate()?;
    let db = db.deref();
    form.media_type.parse::<MediaType>()?;
    insert_into(leave_words::dsl::leave_words)
        .values((
            leave_words::dsl::body.eq(&form.body),
            leave_words::dsl::media_type.eq(&form.media_type),
        ))
        .execute(db)?;
    Ok(Json(()))
}

#[delete("/<id>")]
pub fn destory(_user: Administrator, id: i64, db: Database) -> Result<Json<()>> {
    let db = db.deref();
    delete(leave_words::dsl::leave_words.filter(leave_words::dsl::id.eq(id))).execute(db)?;
    Ok(Json(()))
}
