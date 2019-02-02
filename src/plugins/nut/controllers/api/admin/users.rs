use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use rocket_contrib::json::Json;

use super::super::super::super::super::super::{
    errors::JsonResult,
    orm::{schema::users, Database},
};
use super::super::super::super::request::Administrator;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub real_name: String,
    pub nick_name: String,
    pub email: String,
    pub provider_type: String,
    pub sign_in_count: i64,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<IpNetwork>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_ip: Option<IpNetwork>,
}

#[get("/admin/users")]
pub fn index(_user: Administrator, db: Database) -> JsonResult<Vec<Item>> {
    let db = db.deref();

    let items = users::dsl::users
        .select((
            users::dsl::id,
            users::dsl::real_name,
            users::dsl::nick_name,
            users::dsl::email,
            users::dsl::provider_type,
            users::dsl::sign_in_count,
            users::dsl::current_sign_in_at,
            users::dsl::current_sign_in_ip,
            users::dsl::last_sign_in_at,
            users::dsl::last_sign_in_ip,
        ))
        .order(users::dsl::updated_at.desc())
        .load::<Item>(db)?;
    Ok(Json(items))
}
