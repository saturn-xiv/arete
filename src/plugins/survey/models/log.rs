use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::survey_logs;

#[derive(Queryable)]
pub struct Item {
    pub id: ID,
    pub form_id: ID,
    pub user_id: Option<ID>,
    pub ip: String,
    pub message: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(&self, form: ID, user: Option<ID>, ip: &str, message: &str) -> Result<()>;
    fn by_form(&self, id: ID) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(&self, form: ID, user: Option<ID>, ip: &str, message: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(survey_logs::dsl::survey_logs)
            .values((
                survey_logs::dsl::user_id.eq(user),
                survey_logs::dsl::form_id.eq(form),
                survey_logs::dsl::ip.eq(ip),
                survey_logs::dsl::message.eq(message),
                survey_logs::dsl::created_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_form(&self, id: ID) -> Result<Vec<Item>> {
        let items = survey_logs::dsl::survey_logs
            .filter(survey_logs::dsl::form_id.eq(id))
            .order(survey_logs::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
