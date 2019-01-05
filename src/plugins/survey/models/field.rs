use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use serde_json::{from_value, to_value, Value};

use super::super::super::super::{
    errors::Result,
    orm::{schema::survey_fields, Connection},
};

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub form_id: i64,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub type_: Value,
    pub position: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn type_(self) -> Result<Type> {
        let it = from_value(self.type_)?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    TEXT(bool, String),
    SELECT(bool, Vec<String>),
}

pub trait Dao {
    fn add(
        &self,
        form: &i64,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: &bool,
        type_: &Type,
        position: &i16,
    ) -> Result<i64>;

    fn update(
        &self,
        id: &i64,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: &bool,
        type_: &Type,
        position: &i16,
    ) -> Result<()>;
    fn get(&self, id: &i64) -> Result<Item>;
    fn delete(&self, id: &i64) -> Result<()>;
    fn by_form(&self, id: &i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        form: &i64,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: &bool,
        type_: &Type,
        position: &i16,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(survey_fields::dsl::survey_fields)
            .values((
                survey_fields::dsl::form_id.eq(form),
                survey_fields::dsl::key.eq(key),
                survey_fields::dsl::title.eq(title),
                survey_fields::dsl::description.eq(description),
                survey_fields::dsl::required.eq(required),
                survey_fields::dsl::type_.eq(&to_value(type_)?),
                survey_fields::dsl::position.eq(position),
                survey_fields::dsl::updated_at.eq(&now),
            ))
            .returning(survey_fields::dsl::id)
            .get_result(self)?;
        Ok(id)
    }

    fn update(
        &self,
        id: &i64,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: &bool,
        type_: &Type,
        position: &i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = survey_fields::dsl::survey_fields.filter(survey_fields::dsl::id.eq(id));
        update(it)
            .set((
                survey_fields::dsl::key.eq(key),
                survey_fields::dsl::title.eq(title),
                survey_fields::dsl::description.eq(description),
                survey_fields::dsl::required.eq(required),
                survey_fields::dsl::type_.eq(&to_value(type_)?),
                survey_fields::dsl::position.eq(position),
                survey_fields::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn get(&self, id: &i64) -> Result<Item> {
        let it = survey_fields::dsl::survey_fields
            .filter(survey_fields::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(survey_fields::dsl::survey_fields.filter(survey_fields::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
    fn by_form(&self, id: &i64) -> Result<Vec<Item>> {
        let items = survey_fields::dsl::survey_fields
            .filter(survey_fields::dsl::id.eq(id))
            .order(survey_fields::dsl::position.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
