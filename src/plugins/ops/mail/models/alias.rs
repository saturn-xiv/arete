use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::super::{
    crypto::Password,
    errors::Result,
    orm::{Connection, ID},
};
// use super::super::schema::{vpn_logs, vpn_users};

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub domain_id: ID,
    pub source: String,
    pub destination: String,
    pub created_at: NaiveDateTime,
}
