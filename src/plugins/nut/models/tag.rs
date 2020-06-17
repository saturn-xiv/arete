use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::{tag_resources, tags};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: ID) -> Result<Item>;
    fn create(&self, name: &str, icon: &str, color: &str) -> Result<()>;
    fn update(&self, id: ID, name: &str, icon: &str, color: &str) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
    fn bind(&self, tags: &[ID], rty: &str, rid: ID) -> Result<()>;
    fn unbind(&self, rty: &str, rid: ID) -> Result<()>;
    fn resources(&self, tag: ID) -> Result<Vec<(String, ID)>>;
}

impl Dao for Connection {
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = tags::dsl::tags
            .filter(tags::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&self, name: &str, icon: &str, color: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(tags::dsl::tags)
            .values((
                tags::dsl::name.eq(name),
                tags::dsl::icon.eq(icon),
                tags::dsl::color.eq(color),
                tags::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(&self, id: ID, name: &str, icon: &str, color: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(tags::dsl::tags.filter(tags::dsl::id.eq(id)))
            .set((
                tags::dsl::name.eq(name),
                tags::dsl::icon.eq(icon),
                tags::dsl::color.eq(color),
                tags::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = tags::dsl::tags
            .order(tags::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: ID) -> Result<()> {
        delete(tag_resources::dsl::tag_resources.filter(tag_resources::dsl::id.eq(id)))
            .execute(self)?;
        delete(tags::dsl::tags.filter(tags::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }

    fn bind(&self, tags: &[ID], rty: &str, rid: ID) -> Result<()> {
        let now = Utc::now().naive_utc();
        for it in tags {
            insert_into(tag_resources::dsl::tag_resources)
                .values((
                    tag_resources::dsl::tag_id.eq(it),
                    tag_resources::dsl::resource_id.eq(rid),
                    tag_resources::dsl::resource_type.eq(rty),
                    tag_resources::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        Ok(())
    }

    fn unbind(&self, rty: &str, rid: ID) -> Result<()> {
        delete(
            tag_resources::dsl::tag_resources
                .filter(tag_resources::dsl::resource_type.eq(rty))
                .filter(tag_resources::dsl::resource_id.eq(rid)),
        )
        .execute(self)?;
        Ok(())
    }
    fn resources(&self, tag: ID) -> Result<Vec<(String, ID)>> {
        let items = tag_resources::dsl::tag_resources
            .select((
                tag_resources::dsl::resource_type,
                tag_resources::dsl::resource_id,
            ))
            .filter(tag_resources::dsl::tag_id.eq(tag))
            .order(tag_resources::dsl::created_at.desc())
            .load::<(String, ID)>(self)?;
        Ok(items)
    }
}
