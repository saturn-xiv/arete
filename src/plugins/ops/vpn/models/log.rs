use std::fmt;

use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*};

use super::super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::vpn_logs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    Connection,
    Disconnect,
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Connection => fmt.write_str("Connect"),
            Type::Disconnect => fmt.write_str("Disconnect"),
        }
    }
}

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub type_: String,
    pub trusted_ip: String,
    pub trusted_port: i32,
    pub remote_ip: String,
    pub remote_port: i32,
    pub received: f64,
    pub send: f64,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn all(&self) -> Result<Vec<Item>>;
    fn add(
        &self,
        user: ID,
        type_: &Type,
        trusted_ip: &String,
        trusted_port: i32,
        remote_ip: &String,
        remote_port: i32,
        received: f64,
        send: f64,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self) -> Result<Vec<Item>> {
        let items = vpn_logs::dsl::vpn_logs
            .order(vpn_logs::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn add(
        &self,
        user: ID,
        type_: &Type,
        trusted_ip: &String,
        trusted_port: i32,
        remote_ip: &String,
        remote_port: i32,
        received: f64,
        send: f64,
    ) -> Result<()> {
        insert_into(vpn_logs::dsl::vpn_logs)
            .values((
                vpn_logs::dsl::user_id.eq(user),
                vpn_logs::dsl::type_.eq(&type_.to_string()),
                vpn_logs::dsl::trusted_ip.eq(trusted_ip),
                vpn_logs::dsl::trusted_port.eq(trusted_port),
                vpn_logs::dsl::remote_ip.eq(remote_ip),
                vpn_logs::dsl::remote_port.eq(remote_port),
                vpn_logs::dsl::received.eq(received),
                vpn_logs::dsl::send.eq(send),
            ))
            .execute(self)?;
        Ok(())
    }
}
