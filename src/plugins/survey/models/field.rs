use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::survey_fields;

#[derive(Queryable)]
pub struct Item {
    pub id: ID,
    pub form_id: ID,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub type_: String,
    pub position: i16,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn type_(&self) -> Result<Type> {
        let it = serde_json::from_str(&self.type_)?;
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
        form: ID,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: bool,
        type_: &Type,
        position: i16,
    ) -> Result<()>;

    fn update(
        &self,
        id: ID,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: bool,
        type_: &Type,
        position: i16,
    ) -> Result<()>;
    fn get(&self, id: ID) -> Result<Item>;
    fn delete(&self, id: ID) -> Result<()>;
    fn by_form(&self, id: ID) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        form: ID,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: bool,
        type_: &Type,
        position: i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(survey_fields::dsl::survey_fields)
            .values((
                survey_fields::dsl::form_id.eq(form),
                survey_fields::dsl::key.eq(key),
                survey_fields::dsl::title.eq(title),
                survey_fields::dsl::description.eq(description),
                survey_fields::dsl::required.eq(required),
                survey_fields::dsl::type_.eq(&serde_json::to_string(type_)?),
                survey_fields::dsl::position.eq(position),
                survey_fields::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: ID,
        key: &String,
        title: &String,
        description: &Option<String>,
        required: bool,
        type_: &Type,
        position: i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = survey_fields::dsl::survey_fields.filter(survey_fields::dsl::id.eq(id));
        update(it)
            .set((
                survey_fields::dsl::key.eq(key),
                survey_fields::dsl::title.eq(title),
                survey_fields::dsl::description.eq(description),
                survey_fields::dsl::required.eq(required),
                survey_fields::dsl::type_.eq(&serde_json::to_string(type_)?),
                survey_fields::dsl::position.eq(position),
                survey_fields::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn get(&self, id: ID) -> Result<Item> {
        let it = survey_fields::dsl::survey_fields
            .filter(survey_fields::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn delete(&self, id: ID) -> Result<()> {
        delete(survey_fields::dsl::survey_fields.filter(survey_fields::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
    fn by_form(&self, id: ID) -> Result<Vec<Item>> {
        let items = survey_fields::dsl::survey_fields
            .filter(survey_fields::dsl::id.eq(id))
            .order(survey_fields::dsl::position.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
