use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::survey_responses;

#[derive(Queryable)]
pub struct Item {
    pub id: ID,
    pub form_id: ID,
    pub email: String,
    pub username: String,
    pub ip: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        form: ID,
        email: &String,
        username: &String,
        ip: &String,
        content: &String,
    ) -> Result<()>;
    fn by_form(&self, id: ID) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        form: ID,
        email: &String,
        username: &String,
        ip: &String,
        content: &String,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(survey_responses::dsl::survey_responses)
            .values((
                survey_responses::dsl::form_id.eq(form),
                survey_responses::dsl::ip.eq(ip),
                survey_responses::dsl::email.eq(&email),
                survey_responses::dsl::username.eq(&username),
                survey_responses::dsl::content.eq(content),
                survey_responses::dsl::created_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn by_form(&self, id: ID) -> Result<Vec<Item>> {
        let items = survey_responses::dsl::survey_responses
            .filter(survey_responses::dsl::form_id.eq(id))
            .order(survey_responses::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
