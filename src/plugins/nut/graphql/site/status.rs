use std::ops::Deref;

use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_query,
    sql_types::{BigInt, Text, Timestamp},
};
use r2d2_redis::redis::cmd;
use validator::Validate;

use super::super::super::super::super::{
    errors::Result,
    graphql::{context::Context, session::Session, Handler, I64},
    orm::Connection as DbConnection,
    sys,
};

const MB: u64 = 1024 * 1024;

#[derive(GraphQLObject)]
pub struct Status {
    os: Os,
    redis: String,
    postgresql: PostgreSql,
    network: Vec<Network>,
    routes: Vec<Route>,
}

#[derive(GraphQLObject)]
pub struct Route {
    pub path: String,
    pub method: String,
}

#[derive(GraphQLObject)]
pub struct Os {
    pub uts: Uts,
    pub uptime: String,
    pub process_count: i32,
    pub load_average: LoadAverage,
    pub ram: Ram,
    pub swap: Swap,
    pub pid: i32,
}

#[derive(GraphQLObject)]
pub struct LoadAverage {
    pub l1: f64,
    pub l2: f64,
    pub l3: f64,
}
#[derive(GraphQLObject)]
pub struct Uts {
    pub machine: String,
    pub node_name: String,
    pub release: String,
    pub sys_name: String,
    pub version: String,
}
#[derive(GraphQLObject)]
pub struct Ram {
    pub unused: i32,
    pub total: i32,
}
#[derive(GraphQLObject)]
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

#[derive(GraphQLObject)]
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

#[derive(GraphQLObject)]
pub struct PostgreSql {
    pub status: Option<PgStatus>,
    pub databases: Vec<PgDatabaseM>,
}

#[derive(GraphQLObject, QueryableByName)]
pub struct PgStatus {
    #[sql_type = "Text"]
    pub version: String,
    #[sql_type = "Timestamp"]
    pub timestamp: NaiveDateTime,
}
#[derive(GraphQLObject)]
pub struct PgDatabaseM {
    pub name: String,
    pub size: I64,
}

#[derive(QueryableByName)]
pub struct PgDatabase {
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "BigInt"]
    pub size: i64,
}

impl PostgreSql {
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
            databases: sql_query(r###"SELECT pg_database.datname as "name", pg_database_size(pg_database.datname)/1024/1024 AS "size" FROM pg_database ORDER by "size" DESC"###).load::<PgDatabase>(db)?.into_iter().map(|x|{
                PgDatabaseM{
                    name:x.name,
                    size: I64(x.size)
                }
            }).collect(),
        })
    }
}

#[derive(Validate)]
pub struct Get {}

impl Handler for Get {
    type Item = Status;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db()?;
        let db = db.deref();
        s.administrator(db)?;

        let ch = c.cache()?;
        Ok(Status {
            os: Os::new()?,
            network: Network::new()?,
            redis: cmd("info").query::<String>(ch.deref())?,
            postgresql: PostgreSql::new(db.deref())?,
            routes: Vec::new(),
            // routes: ROUTER
            //     .routes
            //     .iter()
            //     .map(|(m, p, _)| Route {
            //         path: p.to_string(),
            //         method: m.to_string(),
            //     })
            //     .collect(),
        })
    }
}
