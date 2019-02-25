use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*};

use super::super::super::super::{errors::Result, orm::Connection};
use super::super::schema::survey_logs;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub form_id: i64,
    pub user_id: Option<i64>,
    pub ip: Option<String>,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        form: &i64,
        user: &Option<i64>,
        ip: &Option<String>,
        message: &String,
    ) -> Result<i64>;
    fn by_form(&self, id: &i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        form: &i64,
        user: &Option<i64>,
        ip: &Option<String>,
        message: &String,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(survey_logs::dsl::survey_logs)
            .values((
                survey_logs::dsl::user_id.eq(user),
                survey_logs::dsl::form_id.eq(form),
                survey_logs::dsl::ip.eq(ip),
                survey_logs::dsl::message.eq(message),
                survey_logs::dsl::created_at.eq(&now),
            ))
            .returning(survey_logs::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn by_form(&self, id: &i64) -> Result<Vec<Item>> {
        let items = survey_logs::dsl::survey_logs
            .filter(survey_logs::dsl::form_id.eq(id))
            .order(survey_logs::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
