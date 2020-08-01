use std::fmt;
use std::ops::Add;
use std::str::FromStr;

use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use failure::Error;

use super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::policies;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub role: String,
    pub resource: Option<String>,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
    pub version: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn enable(&self) -> bool {
        let today = Utc::now().naive_utc().date();
        today.ge(&self.nbf) && today.le(&self.exp)
    }
    pub fn weeks(d: i64) -> (NaiveDate, NaiveDate) {
        let nbf = Utc::now().naive_utc();
        let exp = nbf.add(Duration::weeks(d));
        (nbf.date(), exp.date())
    }
}

#[derive(Insertable)]
#[table_name = "policies"]
pub struct New<'a> {
    pub user_id: &'a ID,
    pub role: &'a str,
    pub resource: Option<&'a str>,
    pub nbf: &'a NaiveDate,
    pub exp: &'a NaiveDate,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn all(&self, user: ID) -> Result<Vec<Item>>;
    fn is(&self, user: ID, role: &Role) -> bool;
    fn can(&self, user: ID, role: &Role, resource: &Option<String>) -> bool;
    fn deny(&self, user: ID, role: &Role, resource: &Option<String>) -> Result<()>;
    fn forbidden(&self, user: ID) -> Result<()>;
    fn apply(
        &self,
        user: ID,
        role: &Role,
        resource: &Option<String>,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self, user: ID) -> Result<Vec<Item>> {
        let items = policies::dsl::policies
            .filter(policies::dsl::user_id.eq(user))
            .load::<Item>(self)?;
        Ok(items.into_iter().filter(|x| x.enable()).collect::<_>())
    }
    fn is(&self, user: ID, role: &Role) -> bool {
        self.can(user, role, &None)
    }
    fn can(&self, user: ID, role: &Role, resource: &Option<String>) -> bool {
        let it = match resource {
            Some(_) => policies::dsl::policies
                .filter(policies::dsl::user_id.eq(user))
                .filter(policies::dsl::role.eq(&role.to_string()))
                .filter(policies::dsl::resource.eq(resource))
                .first::<Item>(self),
            None => policies::dsl::policies
                .filter(policies::dsl::user_id.eq(user))
                .filter(policies::dsl::role.eq(&role.to_string()))
                .filter(policies::dsl::resource.is_null())
                .first::<Item>(self),
        };
        if let Ok(it) = it {
            return it.enable();
        }
        false
    }

    fn deny(&self, user: ID, role: &Role, resource: &Option<String>) -> Result<()> {
        match resource {
            Some(_) => delete(
                policies::dsl::policies
                    .filter(policies::dsl::user_id.eq(user))
                    .filter(policies::dsl::role.eq(&role.to_string()))
                    .filter(policies::dsl::resource.eq(resource)),
            )
            .execute(self),
            None => delete(
                policies::dsl::policies
                    .filter(policies::dsl::user_id.eq(user))
                    .filter(policies::dsl::role.eq(&role.to_string()))
                    .filter(policies::dsl::resource.is_null()),
            )
            .execute(self),
        }?;
        Ok(())
    }

    fn forbidden(&self, user: ID) -> Result<()> {
        delete(policies::dsl::policies.filter(policies::dsl::user_id.eq(user))).execute(self)?;
        Ok(())
    }

    fn apply(
        &self,
        user: ID,
        role: &Role,
        resource: &Option<String>,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();

        let it = match resource {
            Some(_) => policies::dsl::policies
                .filter(policies::dsl::user_id.eq(user))
                .filter(policies::dsl::role.eq(&role.to_string()))
                .filter(policies::dsl::resource.eq(resource))
                .first::<Item>(self),
            None => policies::dsl::policies
                .filter(policies::dsl::user_id.eq(user))
                .filter(policies::dsl::role.eq(&role.to_string()))
                .filter(policies::dsl::resource.is_null())
                .first::<Item>(self),
        };

        match it {
            Ok(it) => {
                let it = policies::dsl::policies.filter(policies::dsl::id.eq(&it.id));
                update(it)
                    .set((
                        policies::dsl::exp.eq(exp),
                        policies::dsl::nbf.eq(nbf),
                        policies::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(policies::dsl::policies)
                    .values(&New {
                        user_id: &user,
                        role: &role.to_string(),
                        resource: resource.as_ref().map(|x| x as _),
                        exp: &exp,
                        nbf: &nbf,
                        updated_at: &now,
                    })
                    .execute(self)?;
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Role {
    Root,
    Admin,
    Manager,
    Member,
    By(String),
}

impl fmt::Display for Role {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Root => write!(fmt, "root"),
            Role::Admin => write!(fmt, "admin"),
            Role::Manager => write!(fmt, "manager"),
            Role::Member => write!(fmt, "member"),
            Role::By(n) => write!(fmt, "{}", &n),
        }
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "root" => Role::Root,
            "admin" => Role::Admin,
            "manager" => Role::Manager,
            "member" => Role::Member,
            v => Role::By(v.to_string()),
        })
    }
}
