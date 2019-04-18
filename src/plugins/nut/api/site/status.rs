use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_query,
    sql_types::{BigInt, Text, Timestamp},
};
use r2d2_redis::redis::cmd;
use rocket_contrib::json::Json;

use super::super::super::super::super::{
    cache::Cache,
    errors::{JsonResult, Result},
    orm::{Connection as DbConnection, Database},
    sys,
};
use super::super::users::Administrator;

const MB: u64 = 1024 * 1024;

#[derive(Serialize)]
pub struct Status {
    os: Os,
    redis: String,
    database: Db,
    network: Vec<Network>,
}

#[derive(Serialize)]
pub struct Os {
    pub uts: Uts,
    pub uptime: String,
    pub process_count: i32,
    pub load_average: LoadAverage,
    pub ram: Ram,
    pub swap: Swap,
    pub pid: i32,
}

#[derive(Serialize)]
pub struct LoadAverage {
    pub l1: f64,
    pub l2: f64,
    pub l3: f64,
}
#[derive(Serialize)]
pub struct Uts {
    pub machine: String,
    pub node_name: String,
    pub release: String,
    pub sys_name: String,
    pub version: String,
}
#[derive(Serialize)]
pub struct Ram {
    pub unused: i32,
    pub total: i32,
}
#[derive(Serialize)]
pub struct Swap {
    pub unused: i32,
    pub total: i32,
}

impl Os {
    pub fn new() -> Result<Self> {
        let un = sys::uts_name();

        let uts = Uts {
            machine: un.machine().to_string(),
            node_name: un.nodename().to_string(),
            release: un.release().to_string(),
            sys_name: un.sysname().to_string(),
            version: un.version().to_string(),
        };

        let si = sys::sys_info()?;
        let uptime = format!("{:?}", si.uptime());
        let process_count = si.process_count() as i32;

        let (l1, l2, l3) = si.load_average();

        Ok(Os {
            pid: sys::pid() as i32,
            uts: uts,
            uptime: uptime,
            process_count: process_count,
            load_average: LoadAverage {
                l1: l1,
                l2: l2,
                l3: l3,
            },
            ram: Ram {
                unused: (si.ram_unused() / MB) as i32,
                total: (si.ram_total() / MB) as i32,
            },
            swap: Swap {
                unused: (si.swap_free() / MB) as i32,
                total: (si.swap_total() / MB) as i32,
            },
        })
    }
}

#[derive(Serialize)]
pub struct Network {
    pub name: String,
    pub ip4: Option<String>,
    pub ip6: Option<String>,
    pub mac: Option<String>,
}

impl Network {
    pub fn new() -> Result<Vec<Self>> {
        let mut items = Vec::new();
        for it in sys::network::interfaces()? {
            items.push(Self {
                ip4: match sys::network::ip4(&it)? {
                    Some(v) => Some(v.to_string()),
                    None => None,
                },
                ip6: match sys::network::ip6(&it)? {
                    Some(v) => Some(v.to_string()),
                    None => None,
                },
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

#[cfg(feature = "mysql")]
#[derive(Serialize)]
pub struct Db {}

#[cfg(feature = "sqlite")]
#[derive(Serialize)]
pub struct Db {}

#[cfg(feature = "postgresql")]
#[derive(Serialize)]
pub struct Db {
    pub status: Option<PgStatus>,
    pub databases: Vec<PgDatabase>,
}

#[derive(Serialize, QueryableByName)]
pub struct PgStatus {
    #[sql_type = "Text"]
    pub version: String,
    #[sql_type = "Timestamp"]
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize, QueryableByName)]
pub struct PgDatabase {
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "BigInt"]
    pub size: i64,
}

impl Db {
    #[cfg(feature = "mysql")]
    pub fn new(_db: &DbConnection) -> Result<Self> {
        Ok(Self {})
    }
    #[cfg(feature = "sqlite")]
    pub fn new(_db: &DbConnection) -> Result<Self> {
        Ok(Self {})
    }
    #[cfg(feature = "postgresql")]
    pub fn new(db: &DbConnection) -> Result<Self> {
        let status = match sql_query(r###"SELECT version() as "version", now() as "timestamp""###)
            .load::<PgStatus>(db)?
            .first()
        {
            Some(it) => Some(PgStatus {
                version: it.version.clone(),
                timestamp: it.timestamp,
            }),
            None => None,
        };
        Ok(Self {
            status: status,
            databases: sql_query(r###"SELECT pg_database.datname as "name", pg_database_size(pg_database.datname)/1024/1024 AS "size" FROM pg_database ORDER by "size" DESC"###).load::<PgDatabase>(db)?,
        })
    }
}

#[get("/site/status")]
pub fn get(db: Database, _user: Administrator, cache: Cache) -> JsonResult<Status> {
    Ok(Json(Status {
        os: Os::new()?,
        network: Network::new()?,
        redis: cmd("info").query::<String>(cache.deref())?,
        #[cfg(feature = "postgresql")]
        database: Db::new(db.deref())?,
    }))
}
