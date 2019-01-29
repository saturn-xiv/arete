use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{
        schema::{categories, category_resources},
        Connection,
    },
};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub position: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: &i64) -> Result<Item>;
    fn create(
        &self,
        parent: &Option<i64>,
        name: &String,
        icon: &String,
        color: &String,
        position: i16,
    ) -> Result<()>;
    fn update(
        &self,
        id: &i64,
        parent: &Option<i64>,
        name: &String,
        icon: &String,
        color: &String,
        position: i16,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: &i64) -> Result<()>;
    fn bind(&self, category: &i64, rty: &String, rid: &i64) -> Result<()>;
    fn unbind(&self, category: &i64, rty: &String, rid: &i64) -> Result<()>;
    fn resources(&self, category: &i64, rty: &String) -> Result<Vec<i64>>;
    fn children(&self, category: &Option<i64>) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn by_id(&self, id: &i64) -> Result<Item> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        parent: &Option<i64>,
        name: &String,
        icon: &String,
        color: &String,
        position: i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(categories::dsl::categories)
            .values((
                categories::dsl::parent_id.eq(parent),
                categories::dsl::name.eq(name),
                categories::dsl::icon.eq(icon),
                categories::dsl::color.eq(color),
                categories::dsl::position.eq(&position),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: &i64,
        parent: &Option<i64>,
        name: &String,
        icon: &String,
        color: &String,
        position: i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(categories::dsl::categories.filter(categories::dsl::id.eq(id)))
            .set((
                categories::dsl::parent_id.eq(parent),
                categories::dsl::name.eq(name),
                categories::dsl::icon.eq(icon),
                categories::dsl::color.eq(color),
                categories::dsl::position.eq(&position),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = categories::dsl::categories
            .order(categories::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: &i64) -> Result<()> {
        delete(
            category_resources::dsl::category_resources.filter(category_resources::dsl::id.eq(id)),
        )
        .execute(self)?;
        delete(categories::dsl::categories.filter(categories::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }

    fn bind(&self, category: &i64, rty: &String, rid: &i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(category_resources::dsl::category_resources)
            .values((
                category_resources::dsl::category_id.eq(&category),
                category_resources::dsl::resource_id.eq(&rid),
                category_resources::dsl::resource_type.eq(&rty),
                category_resources::dsl::created_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn unbind(&self, category: &i64, rty: &String, rid: &i64) -> Result<()> {
        delete(
            category_resources::dsl::category_resources
                .filter(category_resources::dsl::category_id.eq(category))
                .filter(category_resources::dsl::resource_type.eq(rty))
                .filter(category_resources::dsl::resource_id.eq(rid)),
        )
        .execute(self)?;
        Ok(())
    }
    fn resources(&self, category: &i64, rty: &String) -> Result<Vec<i64>> {
        let items = category_resources::dsl::category_resources
            .select(category_resources::dsl::resource_id)
            .filter(category_resources::dsl::category_id.eq(category))
            .filter(category_resources::dsl::resource_type.eq(rty))
            .order(category_resources::dsl::created_at.desc())
            .load::<i64>(self)?;
        Ok(items)
    }
    fn children(&self, parent: &Option<i64>) -> Result<Vec<Item>> {
        let items = if *parent == None {
            categories::dsl::categories
                .filter(categories::dsl::parent_id.is_null())
                .order(categories::dsl::position.asc())
                .load::<Item>(self)?
        } else {
            categories::dsl::categories
                .filter(categories::dsl::parent_id.eq(parent))
                .order(categories::dsl::position.asc())
                .load::<Item>(self)?
        };

        Ok(items)
    }
}
