use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{
        schema::{forum_topics, forum_topics_categories, forum_topics_tags},
        Connection,
    },
};
use super::super::super::nut::MediaType;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        user: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<i64>,
        categories: &Vec<i64>,
    ) -> Result<i64>;
    fn get(&self, id: &i64) -> Result<Item>;
    fn update(
        &self,
        id: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<i64>,
        categories: &Vec<i64>,
    ) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
    fn by_user(&self, id: &i64) -> Result<Vec<Item>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<i64>,
        categories: &Vec<i64>,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(forum_topics::dsl::forum_topics)
            .values((
                forum_topics::dsl::user_id.eq(user),
                forum_topics::dsl::title.eq(title),
                forum_topics::dsl::body.eq(body),
                forum_topics::dsl::media_type.eq(&media_type.to_string()),
                forum_topics::dsl::updated_at.eq(&now),
            ))
            .returning(forum_topics::dsl::id)
            .get_result(self)?;
        for it in tags {
            insert_into(forum_topics_tags::dsl::forum_topics_tags)
                .values((
                    forum_topics_tags::dsl::tag_id.eq(&it),
                    forum_topics_tags::dsl::topic_id.eq(&id),
                    forum_topics_tags::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        for it in categories {
            insert_into(forum_topics_categories::dsl::forum_topics_categories)
                .values((
                    forum_topics_categories::dsl::category_id.eq(&it),
                    forum_topics_categories::dsl::topic_id.eq(&id),
                    forum_topics_categories::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        Ok(id)
    }
    fn get(&self, id: &i64) -> Result<Item> {
        let it = forum_topics::dsl::forum_topics
            .filter(forum_topics::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(
        &self,
        id: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        tags: &Vec<i64>,
        categories: &Vec<i64>,
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

        delete(
            forum_topics_tags::dsl::forum_topics_tags
                .filter(forum_topics_tags::dsl::topic_id.eq(id)),
        )
        .execute(self)?;
        for it in tags {
            insert_into(forum_topics_tags::dsl::forum_topics_tags)
                .values((
                    forum_topics_tags::dsl::tag_id.eq(&it),
                    forum_topics_tags::dsl::topic_id.eq(id),
                    forum_topics_tags::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }

        delete(
            forum_topics_categories::dsl::forum_topics_categories
                .filter(forum_topics_categories::dsl::topic_id.eq(id)),
        )
        .execute(self)?;
        for it in categories {
            insert_into(forum_topics_categories::dsl::forum_topics_categories)
                .values((
                    forum_topics_categories::dsl::category_id.eq(&it),
                    forum_topics_categories::dsl::topic_id.eq(id),
                    forum_topics_categories::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn latest(&self) -> Result<Vec<Item>> {
        let items = forum_topics::dsl::forum_topics
            .order(forum_topics::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_user(&self, id: &i64) -> Result<Vec<Item>> {
        let items = forum_topics::dsl::forum_topics
            .filter(forum_topics::dsl::user_id.eq(id))
            .order(forum_topics::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(
            forum_topics_tags::dsl::forum_topics_tags
                .filter(forum_topics_tags::dsl::topic_id.eq(id)),
        )
        .execute(self)?;
        delete(
            forum_topics_categories::dsl::forum_topics_categories
                .filter(forum_topics_categories::dsl::topic_id.eq(id)),
        )
        .execute(self)?;
        delete(forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
