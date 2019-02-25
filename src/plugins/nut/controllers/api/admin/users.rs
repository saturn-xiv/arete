use std::ops::Deref;

use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use failure::Error;
use ipnetwork::IpNetwork;
use rocket_contrib::json::Json;
use validator::Validate;

use super::super::super::super::super::super::{
    errors::JsonResult,
    orm::{schema::users, Database},
};
use super::super::super::super::{
    models::{
        policy::{Dao as PolicyDao, Role},
        user::Dao as UserDao,
    },
    request::Administrator,
};


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


#[get("/admin/users/<id>/authority")]
pub fn get_authority(_user: Administrator, id: i64, db: Database) -> JsonResult<User> {
    let db = db.deref();
    let user = UserDao::by_id(db, &id)?;

    Ok(Json(User {
        nick_name: user.nick_name,
        real_name: user.real_name,
        policies: PolicyDao::all(db, &id)?,
    }))
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authority {
    pub policies: Vec<(String, Option<String>)>,
    #[validate(length(min = "1"))]
    pub nbf: String,
    #[validate(length(min = "1"))]
    pub exp: String,
}

#[post("/admin/users/<id>/authority", format = "json", data = "<form>")]
pub fn post_authority(
    _user: Administrator,
    id: i64,
    form: Json<Authority>,
    db: Database,
) -> JsonResult<()> {
    form.validate()?;

    let nbf = form.nbf.parse::<NaiveDate>()?;
    let exp = form.exp.parse::<NaiveDate>()?;
    let db = db.deref();
    UserDao::by_id(db, &id)?;

    if PolicyDao::can(db, &id, &Role::Root, &None) {
        return Err(format_err!("can't modify root").into());
    }

    db.transaction::<_, Error, _>(|| {
        for (ro, re) in PolicyDao::all(db, &id)? {
            PolicyDao::deny(db, &id, &ro, &re)?;
        }

        for (ro, re) in form.policies.iter() {
            let ro = ro.parse()?;
            if ro != Role::Root {
                PolicyDao::apply(db, &id, &ro, &re, &nbf, &exp)?;
            }
        }
        Ok(())
    })?;

    Ok(Json(()))
}
