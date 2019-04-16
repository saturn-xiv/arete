use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{errors::Result, orm::Connection};
use super::super::schema::{survey_fields, survey_forms, survey_logs, survey_responses};

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub type_: String,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
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
pub struct Type {
    pub public: bool,
    pub multiple: bool,
}

pub trait Dao {
    fn add(
        &self,
        user: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
        type_: &Type,
    ) -> Result<()>;

    fn update(
        &self,
        id: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()>;
    fn get(&self, id: &i64) -> Result<Item>;
    fn delete(&self, id: &i64) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
        type_: &Type,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(survey_forms::dsl::survey_forms)
            .values((
                survey_forms::dsl::user_id.eq(user),
                survey_forms::dsl::title.eq(title),
                survey_forms::dsl::description.eq(description),
                survey_forms::dsl::nbf.eq(nbf),
                survey_forms::dsl::exp.eq(exp),
                survey_forms::dsl::type_.eq(&serde_json::to_string(type_)?),
                survey_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: &i64,
        title: &String,
        description: &String,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = survey_forms::dsl::survey_forms.filter(survey_forms::dsl::id.eq(id));
        update(it)
            .set((
                survey_forms::dsl::title.eq(title),
                survey_forms::dsl::description.eq(description),
                survey_forms::dsl::nbf.eq(nbf),
                survey_forms::dsl::exp.eq(exp),
                survey_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn get(&self, id: &i64) -> Result<Item> {
        let it = survey_forms::dsl::survey_forms
            .filter(survey_forms::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }

    fn delete(&self, id: &i64) -> Result<()> {
        delete(survey_fields::dsl::survey_fields.filter(survey_fields::dsl::form_id.eq(id)))
            .execute(self)?;
        delete(
            survey_responses::dsl::survey_responses.filter(survey_responses::dsl::form_id.eq(id)),
        )
        .execute(self)?;
        delete(survey_logs::dsl::survey_logs.filter(survey_logs::dsl::form_id.eq(id)))
            .execute(self)?;
        delete(survey_forms::dsl::survey_forms.filter(survey_forms::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }

    fn latest(&self) -> Result<Vec<Item>> {
        let items = survey_forms::dsl::survey_forms
            .order(survey_forms::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
