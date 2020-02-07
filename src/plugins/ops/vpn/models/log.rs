use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};

use super::super::super::super::super::{
    errors::Result,
    orm::{Connection, ID},
};
use super::super::schema::vpn_logs;

#[derive(Queryable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ID,
    pub user_id: ID,
    pub trusted_ip: String,
    pub trusted_port: i32,
    pub remote_ip: String,
    pub remote_port: i32,
    pub received: Option<i64>,
    pub send: Option<i64>,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}

pub trait Dao {
    fn all(&self, limit: i64) -> Result<Vec<Item>>;
    fn connect(
        &self,
        user: ID,
        trusted_ip: &str,
        trusted_port: i32,
        remote_ip: &str,
        remote_port: i32,
    ) -> Result<()>;
    fn disconnect(
        &self,
        user: ID,
        trusted_ip: &str,
        trusted_port: i32,
        received: i64,
        send: i64,
    ) -> Result<()>;
}

impl Dao for Connection {
    fn all(&self, limit: i64) -> Result<Vec<Item>> {
        let items = vpn_logs::dsl::vpn_logs
            .order(vpn_logs::dsl::opened_at.desc())
            .limit(limit)
            .load::<Item>(self)?;
        Ok(items)
    }
    fn connect(
        &self,
        user: ID,
        trusted_ip: &str,
        trusted_port: i32,
        remote_ip: &str,
        remote_port: i32,
    ) -> Result<()> {
        insert_into(vpn_logs::dsl::vpn_logs)
            .values((
                vpn_logs::dsl::user_id.eq(user),
                vpn_logs::dsl::trusted_ip.eq(trusted_ip),
                vpn_logs::dsl::trusted_port.eq(trusted_port),
                vpn_logs::dsl::remote_ip.eq(remote_ip),
                vpn_logs::dsl::remote_port.eq(remote_port),
            ))
            .execute(self)?;
        Ok(())
    }

    fn disconnect(
        &self,
        user: ID,
        trusted_ip: &str,
        trusted_port: i32,
        received: i64,
        send: i64,
    ) -> Result<()> {
        let it = vpn_logs::dsl::vpn_logs
            .filter(vpn_logs::dsl::user_id.eq(user))
            .filter(vpn_logs::dsl::trusted_ip.eq(trusted_ip))
            .filter(vpn_logs::dsl::trusted_port.eq(trusted_port))
            .filter(vpn_logs::dsl::received.is_null())
            .filter(vpn_logs::dsl::send.is_null())
            .filter(vpn_logs::dsl::closed_at.is_null());

        update(it)
            .set((
                vpn_logs::dsl::received.eq(Some(received)),
                vpn_logs::dsl::send.eq(Some(send)),
                vpn_logs::dsl::closed_at.eq(&Some(Utc::now().naive_utc())),
            ))
            .execute(self)?;
        Ok(())
    }
}
