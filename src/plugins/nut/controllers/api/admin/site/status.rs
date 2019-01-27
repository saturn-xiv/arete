use std::collections::BTreeMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_query,
    sql_types::{BigInt, Text, Timestamp},
};
use r2d2_redis::redis::cmd;
use rocket_contrib::json::Json;

use super::super::super::super::super::super::super::{
    errors::{JsonResult, Result},
    orm::{Connection, Database},
    redis::Redis,
    sys,
};
use super::super::super::super::super::request::Administrator;

const MB: u64 = 1024 * 1024;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    os: Os,
    redis: String,
    postgresql: PostgreSql,
    network: Vec<Network>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Os(BTreeMap<&'static str, String>);

impl Os {
    pub fn new() -> Result<Self> {
        let mut items = BTreeMap::new();

        let un = sys::uts_name();
        items.insert(
            "Name",
            format!(
                "{} {} {} {} {}",
                un.machine(),
                un.nodename(),
                un.release(),
                un.sysname(),
                un.version()
            ),
        );

        let si = sys::sys_info()?;
        items.insert("Uptime", format!("{:?}", si.uptime()));
        items.insert("Process Number", si.process_count().to_string());
        let (l1, l2, l3) = si.load_average();
        items.insert("Load Average", format!("{:.2} {:.2} {:.2}", l1, l2, l3));
        items.insert(
            "RAM(MB)",
            format!("{}/{}", si.ram_unused() / MB, si.ram_total() / MB,),
        );
        items.insert(
            "SWAP(MB)",
            format!("{}/{}", si.swap_free() / MB, si.swap_total() / MB),
        );
        items.insert("PID", sys::pid().to_string());

        Ok(Os(items))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub name: String,
    pub ip4: Option<Ipv4Addr>,
    pub ip6: Option<Ipv6Addr>,
    pub mac: Option<String>,
}

impl Network {
    pub fn new() -> Result<Vec<Self>> {
        let mut items = Vec::new();
        for it in sys::network::interfaces()? {
            items.push(Self {
                ip4: sys::network::ip4(&it)?,
                ip6: sys::network::ip6(&it)?,
                mac: match sys::network::mac(&it)? {
                    Some(v) => Some(v.to_hex_string()),
                    None => None,
                },
                name: it,
            });
        }
        Ok(items)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostgreSql {
    pub status: BTreeMap<&'static str, String>,
    pub databases: Vec<PgDatabase>,
}

#[derive(QueryableByName, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PgStatus {
    #[sql_type = "Text"]
    pub version: String,
    #[sql_type = "Timestamp"]
    pub timestamp: NaiveDateTime,
}

#[derive(QueryableByName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PgDatabase {
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "BigInt"]
    pub size: i64,
}

impl PostgreSql {
    pub fn new(db: &Connection) -> Result<Self> {
        let mut status = BTreeMap::new();
        if let Some(it) = sql_query(r###"SELECT version() as "version", now() as "timestamp""###)
            .load::<PgStatus>(db)?
            .first()
        {
            status.insert("Version", it.version.clone());
            status.insert("Timestamp", it.timestamp.to_string());
        }
        Ok(Self {
            status: status,
            databases: sql_query(r###"SELECT pg_database.datname as "name", pg_database_size(pg_database.datname)/1024/1024 AS "size" FROM pg_database ORDER by "size" DESC"###).load::<PgDatabase>(db)?,
        })
    }
}

#[get("/admin/site/status")]
pub fn get(_user: Administrator, redis: Redis, db: Database) -> JsonResult<Status> {
    Ok(Json(Status {
        os: Os::new()?,
        network: Network::new()?,
        redis: cmd("info").query::<String>(redis.deref())?,
        postgresql: PostgreSql::new(db.deref())?,
    }))
}
