use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::schema::vip_members;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    Male,
    Female,
    Others,
}

impl fmt::Display for Gender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => fmt.write_str("male"),
            Gender::Female => fmt.write_str("female"),
            Gender::Others => fmt.write_str("others"),
        }
    }
}

impl FromStr for Gender {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "others" => Ok(Gender::Others),
            v => Err(format_err!("bad gender {}", v)),
        }
    }
}

#[derive(Queryable)]
pub struct Item {
    pub id: ID,
    pub nick_name: String,
    pub real_name: String,
    pub gender: String,
    pub birthday: NaiveDate,
    pub contact: String,
    pub point: i64,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn age(&self) -> i32 {
        Utc::now().year() - self.birthday.year()
    }
    pub fn contact(&self) -> Result<Contact> {
        let it = serde_json::from_str(&self.contact)?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
}

pub trait Dao {
    fn add(
        &self,
        nick_name: &str,
        real_name: &str,
        gender: &Gender,
        birthday: NaiveDate,
        contact: &Contact,
    ) -> Result<()>;
    fn get(&self, id: ID) -> Result<Item>;
    fn update(
        &self,
        id: ID,
        real_name: &str,
        gender: &Gender,
        birthday: NaiveDate,
        contact: &Contact,
    ) -> Result<()>;
    fn list(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: ID) -> Result<()>;
    fn point(&self, id: ID, v: i64) -> Result<()>;
}

impl Dao for Connection {
    fn add(
        &self,
        nick_name: &str,
        real_name: &str,
        gender: &Gender,
        birthday: NaiveDate,
        contact: &Contact,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(vip_members::dsl::vip_members)
            .values((
                vip_members::dsl::nick_name.eq(nick_name),
                vip_members::dsl::real_name.eq(real_name),
                vip_members::dsl::birthday.eq(&birthday),
                vip_members::dsl::gender.eq(&gender.to_string()),
                vip_members::dsl::contact.eq(&serde_json::to_string(contact)?),
                vip_members::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn get(&self, id: ID) -> Result<Item> {
        let it = vip_members::dsl::vip_members
            .filter(vip_members::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(
        &self,
        id: ID,
        real_name: &str,
        gender: &Gender,
        birthday: NaiveDate,
        contact: &Contact,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = vip_members::dsl::vip_members.filter(vip_members::dsl::id.eq(id));
        update(it)
            .set((
                vip_members::dsl::real_name.eq(real_name),
                vip_members::dsl::birthday.eq(&birthday),
                vip_members::dsl::gender.eq(&gender.to_string()),
                vip_members::dsl::contact.eq(&serde_json::to_string(contact)?),
                vip_members::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        Ok(())
    }
    fn list(&self) -> Result<Vec<Item>> {
        let items = vip_members::dsl::vip_members
            .order(vip_members::dsl::nick_name.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn point(&self, id: ID, v: i64) -> Result<()> {
        let now = Utc::now().naive_utc();
        let point = vip_members::dsl::vip_members
            .select(vip_members::dsl::point)
            .filter(vip_members::dsl::id.eq(id))
            .first::<i64>(self)?;
        let it = vip_members::dsl::vip_members.filter(vip_members::dsl::id.eq(id));
        update(it)
            .set((
                vip_members::dsl::point.eq(point + v),
                vip_members::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn delete(&self, id: ID) -> Result<()> {
        delete(vip_members::dsl::vip_members.filter(vip_members::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
