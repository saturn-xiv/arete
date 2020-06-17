use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::{categories, category_resources};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub parent_id: Option<ID>,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub position: i16,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: ID) -> Result<Item>;
    fn create(
        &self,
        parent: Option<ID>,
        name: &str,
        icon: &str,
        color: &str,
        position: i16,
    ) -> Result<()>;
    fn update(
        &self,
        id: ID,
        parent: Option<ID>,
        name: &str,
        icon: &str,
        color: &str,
        position: i16,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
    fn bind(&self, categories: &[ID], rty: &str, rid: ID) -> Result<()>;
    fn unbind(&self, rty: &str, rid: ID) -> Result<()>;
    fn resources(&self, category: ID, rty: &str) -> Result<Vec<ID>>;
    fn children(&self, category: Option<ID>) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = categories::dsl::categories
            .filter(categories::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        parent: Option<ID>,
        name: &str,
        icon: &str,
        color: &str,
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
        id: ID,
        parent: Option<ID>,
        name: &str,
        icon: &str,
        color: &str,
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

    fn delete(&self, id: ID) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(categories::dsl::categories.filter(categories::dsl::parent_id.eq(&Some(id))))
            .set((
                categories::dsl::parent_id.eq(&None::<ID>),
                categories::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        delete(
            category_resources::dsl::category_resources.filter(category_resources::dsl::id.eq(id)),
        )
        .execute(self)?;
        delete(categories::dsl::categories.filter(categories::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }

    fn bind(&self, categories: &[ID], rty: &str, rid: ID) -> Result<()> {
        let now = Utc::now().naive_utc();
        for it in categories {
            insert_into(category_resources::dsl::category_resources)
                .values((
                    category_resources::dsl::category_id.eq(it),
                    category_resources::dsl::resource_id.eq(&rid),
                    category_resources::dsl::resource_type.eq(&rty),
                    category_resources::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn unbind(&self, rty: &str, rid: ID) -> Result<()> {
        delete(
            category_resources::dsl::category_resources
                .filter(category_resources::dsl::resource_type.eq(rty))
                .filter(category_resources::dsl::resource_id.eq(rid)),
        )
        .execute(self)?;
        Ok(())
    }
    fn resources(&self, category: ID, rty: &str) -> Result<Vec<ID>> {
        let items = category_resources::dsl::category_resources
            .select(category_resources::dsl::resource_id)
            .filter(category_resources::dsl::category_id.eq(category))
            .filter(category_resources::dsl::resource_type.eq(rty))
            .order(category_resources::dsl::created_at.desc())
            .load::<ID>(self)?;
        Ok(items)
    }
    fn children(&self, parent: Option<ID>) -> Result<Vec<Item>> {
        let items = if parent == None {
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
