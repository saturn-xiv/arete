use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::attachments;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub title: String,
    pub size: i64,
    pub mime_type: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: ID) -> Result<Item>;
    fn create(
        &self,
        user: ID,
        title: &String,
        mime_type: &String,
        url: &String,
        size: i64,
    ) -> Result<()>;
    fn update(
        &self,
        id: ID,
        title: &String,
        mime_type: &String,
        url: &String,
        size: i64,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_user(&self, user: ID) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        user: ID,
        title: &String,
        mime_type: &String,
        url: &String,
        size: i64,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::mime_type.eq(mime_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: ID,
        title: &String,
        mime_type: &String,
        url: &String,
        size: i64,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id)))
            .set((
                attachments::dsl::title.eq(title),
                attachments::dsl::mime_type.eq(mime_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn by_user(&self, user: ID) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: ID) -> Result<()> {
        delete(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
