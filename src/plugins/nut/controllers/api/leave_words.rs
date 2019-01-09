use std::ops::Deref;

use diesel::{insert_into, prelude::*};
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::{
    errors::Result,
    orm::{schema::leave_words, Database},
};
use super::super::super::MediaType;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct New {
    #[validate(length(min = "1"))]
    pub body: String,
    #[validate(length(min = "1"))]
    pub media_type: String,
}

#[post("/leave-words", format = "json", data = "<form>")]
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
