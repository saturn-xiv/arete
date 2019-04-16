use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::super::nut::MediaType;
use super::super::schema::forum_posts;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub topic_id: ID,
    pub post_id: Option<ID>,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        user: ID,
        topic: ID,
        post: Option<ID>,
        body: &String,
        media_type: &MediaType,
    ) -> Result<()>;
    fn get(&self, id: ID) -> Result<Item>;
    fn update(&self, id: ID, body: &String, media_type: &MediaType) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
    fn by_user(&self, id: ID) -> Result<Vec<Item>>;
    fn by_topic(&self, id: ID) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: ID,
        topic: ID,
        post: Option<ID>,
        body: &String,
        media_type: &MediaType,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(forum_posts::dsl::forum_posts)
            .values((
                forum_posts::dsl::user_id.eq(user),
                forum_posts::dsl::topic_id.eq(topic),
                forum_posts::dsl::post_id.eq(post),
                forum_posts::dsl::body.eq(body),
                forum_posts::dsl::media_type.eq(&media_type.to_string()),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn get(&self, id: ID) -> Result<Item> {
        let it = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(&self, id: ID, body: &String, media_type: &MediaType) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(id));
        update(it)
            .set((
                forum_posts::dsl::body.eq(body),
                forum_posts::dsl::media_type.eq(&media_type.to_string()),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        Ok(())
    }
    fn latest(&self) -> Result<Vec<Item>> {
        let items = forum_posts::dsl::forum_posts
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_user(&self, id: ID) -> Result<Vec<Item>> {
        let items = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::user_id.eq(id))
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_topic(&self, id: ID) -> Result<Vec<Item>> {
        let items = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::topic_id.eq(id))
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn delete(&self, id: ID) -> Result<()> {
        delete(forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
