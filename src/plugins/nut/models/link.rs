use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::links;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub href: String,
    pub label: String,
    pub loc: String,
    pub lang: String,
    pub x: i16,
    pub y: i16,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: ID) -> Result<Item>;
    fn create(&self, lang: &str, label: &str, href: &str, loc: &str, x: i16, y: i16) -> Result<()>;
    fn update(
        &self,
        id: ID,
        lang: &str,
        label: &str,
        href: &str,
        loc: &str,
        x: i16,
        y: i16,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
    fn loc_by_lang(&self, lang: &str) -> Result<Vec<String>>;
    fn by_lang_loc_x(&self, lang: &str, loc: &str, x: i16) -> Result<Vec<Item>>;
    fn by_lang_loc_y(&self, lang: &str, loc: &str, y: i16) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn by_lang_loc_x(&self, lang: &str, loc: &str, x: i16) -> Result<Vec<Item>> {
        let items = links::dsl::links
            .filter(links::dsl::lang.eq(lang))
            .filter(links::dsl::loc.eq(loc))
            .filter(links::dsl::x.eq(x))
            .order(links::dsl::y.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_lang_loc_y(&self, lang: &str, loc: &str, y: i16) -> Result<Vec<Item>> {
        let items = links::dsl::links
            .filter(links::dsl::lang.eq(lang))
            .filter(links::dsl::loc.eq(loc))
            .filter(links::dsl::y.eq(y))
            .order(links::dsl::y.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn loc_by_lang(&self, lang: &str) -> Result<Vec<String>> {
        Ok(links::dsl::links
            .select(links::dsl::loc)
            .filter(links::dsl::lang.eq(lang))
            .distinct()
            .load::<String>(self)?)
    }
    fn by_id(&self, id: ID) -> Result<Item> {
        let it = links::dsl::links
            .filter(links::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&self, lang: &str, label: &str, href: &str, loc: &str, x: i16, y: i16) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(links::dsl::links)
            .values((
                links::dsl::lang.eq(lang),
                links::dsl::loc.eq(loc),
                links::dsl::href.eq(href),
                links::dsl::label.eq(label),
                links::dsl::x.eq(x),
                links::dsl::y.eq(y),
                links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: ID,
        lang: &str,
        label: &str,
        href: &str,
        loc: &str,
        x: i16,
        y: i16,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(links::dsl::links.filter(links::dsl::id.eq(id)))
            .set((
                links::dsl::lang.eq(lang),
                links::dsl::loc.eq(loc),
                links::dsl::href.eq(href),
                links::dsl::label.eq(label),
                links::dsl::x.eq(x),
                links::dsl::y.eq(y),
                links::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = links::dsl::links
            .order(links::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: ID) -> Result<()> {
        delete(links::dsl::links.filter(links::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
