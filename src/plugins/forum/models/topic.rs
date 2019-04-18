use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::super::nut::{
    models::{category::Dao as CategoryDao, tag::Dao as TagDao},
    MediaType,
};
use super::super::schema::forum_topics;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        user: ID,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<ID>,
        categories: &Vec<ID>,
    ) -> Result<()>;
    fn get(&self, id: ID) -> Result<Item>;
    fn update(
        &self,
        id: ID,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<ID>,
        categories: &Vec<ID>,
    ) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
    fn by_user(&self, id: ID) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: ID,
        title: &String,
        body: &String,
        media_type: &MediaType,
        _tags: &Vec<ID>,
        _categories: &Vec<ID>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(forum_topics::dsl::forum_topics)
            .values((
                forum_topics::dsl::user_id.eq(user),
                forum_topics::dsl::title.eq(title),
                forum_topics::dsl::body.eq(body),
                forum_topics::dsl::media_type.eq(&media_type.to_string()),
                forum_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        // TODO add tags & categories
        Ok(())
    }
    fn get(&self, id: ID) -> Result<Item> {
        let it = forum_topics::dsl::forum_topics
            .filter(forum_topics::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(
        &self,
        id: ID,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<ID>,
        categories: &Vec<ID>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(id));
        update(it)
            .set((
                forum_topics::dsl::title.eq(title),
                forum_topics::dsl::body.eq(body),
                forum_topics::dsl::media_type.eq(&media_type.to_string()),
                forum_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        CategoryDao::unbind(self, &RESOURCE_TYPE.to_string(), id)?;
        CategoryDao::bind(self, tags, &RESOURCE_TYPE.to_string(), id)?;
        TagDao::unbind(self, &RESOURCE_TYPE.to_string(), id)?;
        TagDao::bind(self, &categories, &RESOURCE_TYPE.to_string(), id)?;

        Ok(())
    }
    fn latest(&self) -> Result<Vec<Item>> {
        let items = forum_topics::dsl::forum_topics
            .order(forum_topics::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_user(&self, id: ID) -> Result<Vec<Item>> {
        let items = forum_topics::dsl::forum_topics
            .filter(forum_topics::dsl::user_id.eq(id))
            .order(forum_topics::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn delete(&self, id: ID) -> Result<()> {
        CategoryDao::unbind(self, &RESOURCE_TYPE.to_string(), id)?;
        TagDao::unbind(self, &RESOURCE_TYPE.to_string(), id)?;
        delete(forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}

pub const RESOURCE_TYPE: &'static str = "forum.topic";
