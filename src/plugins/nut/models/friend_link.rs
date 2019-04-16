use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::friend_links;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub title: String,
    pub home: String,
    pub logo: String,
    pub position: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: ID) -> Result<Item>;
    fn create(&self, title: &String, home: &String, logo: &String, position: &i16) -> Result<()>;
    fn update(
        &self,
        id: ID,
        title: &String,
        home: &String,
        logo: &String,
        position: &i16,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = friend_links::dsl::friend_links
            .filter(friend_links::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&self, title: &String, home: &String, logo: &String, position: &i16) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(friend_links::dsl::friend_links)
            .values((
                friend_links::dsl::title.eq(title),
                friend_links::dsl::home.eq(home),
                friend_links::dsl::logo.eq(logo),
                friend_links::dsl::position.eq(position),
                friend_links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: ID,
        title: &String,
        home: &String,
        logo: &String,
        position: &i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(friend_links::dsl::friend_links.filter(friend_links::dsl::id.eq(id)))
            .set((
                friend_links::dsl::title.eq(title),
                friend_links::dsl::home.eq(home),
                friend_links::dsl::logo.eq(logo),
                friend_links::dsl::position.eq(position),
                friend_links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = friend_links::dsl::friend_links
            .order(friend_links::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: ID) -> Result<()> {
        delete(friend_links::dsl::friend_links.filter(friend_links::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}
