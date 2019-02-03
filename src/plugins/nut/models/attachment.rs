use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{schema::attachments, Connection},
};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub size: i64,
    pub mime_type: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: &i64) -> Result<Item>;
    fn create(
        &self,
        user: &i64,
        title: &String,
        mime_type: &String,
        url: &String,
        size: &i64,
    ) -> Result<Item>;
    fn update(
        &self,
        id: &i64,
        title: &String,
        mime_type: &String,
        url: &String,
        size: &i64,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_user(&self, user: &i64) -> Result<Vec<Item>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: &i64) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        user: &i64,
        title: &String,
        mime_type: &String,
        url: &String,
        size: &i64,
    ) -> Result<Item> {
        let now = Utc::now().naive_utc();
        let it = insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::mime_type.eq(mime_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
                attachments::dsl::updated_at.eq(&now),
            ))
            .get_result(self)?;
        Ok(it)
    }

    fn update(
        &self,
        id: &i64,
        title: &String,
        mime_type: &String,
        url: &String,
        size: &i64,
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

    fn by_user(&self, user: &i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: &i64) -> Result<()> {
        delete(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
