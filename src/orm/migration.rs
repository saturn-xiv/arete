use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use chrono::{NaiveDateTime, Utc};
use diesel::{connection::SimpleConnection, delete, insert_into, prelude::*, update};

use super::super::{errors::Result, rfc::RFC822};
use super::{schema::schema_migrations, Connection};

pub struct Migration {
    pub version: &'static str,
    pub name: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}

pub const UP: &'static str = include_str!("up.sql");

pub fn new(name: String) -> Result<()> {
    let dir = root_dir().join(format!(
        "{}-{}",
        Utc::now().format("%Y%m%d%H%M%S").to_string(),
        name
    ));
    fs::create_dir(&dir)?;
    for it in vec!["up", "down"] {
        let mut file = dir.join(it);
        file.set_extension("sql");
        info!("generate file {}", file.display());
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
    }

    Ok(())
}

fn root_dir() -> PathBuf {
    Path::new("db").join("migrations")
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub version: String,
    pub name: String,
    pub up: String,
    pub down: String,
    pub run_at: Option<NaiveDateTime>,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<14} {:<32} {}",
            self.version,
            match self.run_at {
                Some(v) => v.to_rfc822(),
                None => "N/A".to_string(),
            },
            self.name,
        )
    }
}

#[derive(Insertable, Eq)]
#[table_name = "schema_migrations"]
pub struct New<'a> {
    pub version: &'a str,
    pub name: &'a str,
    pub up: &'a str,
    pub down: &'a str,
}

impl<'a> fmt::Display for New<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.version, self.name)
    }
}

impl<'a> Ord for New<'a> {
    fn cmp(&self, other: &New) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl<'a> PartialOrd for New<'a> {
    fn partial_cmp(&self, other: &New) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for New<'a> {
    fn eq(&self, other: &New) -> bool {
        self.version == other.version
    }
}

pub trait Dao {
    fn load(&self) -> Result<()>;
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn versions(&self) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn load(&self) -> Result<()> {
        self.batch_execute(UP)?;
        for it in fs::read_dir(root_dir())? {
            let it = it?.path();
            if let Some(name) = it.file_name() {
                if let Some(name) = name.to_str() {
                    info!("find migration: {}", it.display());
                    let it = New {
                        version: &name[..14],
                        name: &name[15..],
                        up: &fs::read_to_string(it.join("up.sql"))?,
                        down: &fs::read_to_string(it.join("down.sql"))?,
                    };
                    let c: i64 = schema_migrations::dsl::schema_migrations
                        .filter(schema_migrations::dsl::version.eq(it.version))
                        .filter(schema_migrations::dsl::name.eq(it.name))
                        .count()
                        .get_result(self)?;
                    if c == 0 {
                        info!("migration {} not exist, insert it", it);
                        insert_into(schema_migrations::dsl::schema_migrations)
                            .values(&it)
                            .execute(self)?;
                    }
                }
            }
        }

        Ok(())
    }
    fn migrate(&self) -> Result<()> {
        let now = Utc::now().naive_utc();
        for it in schema_migrations::dsl::schema_migrations
            .filter(schema_migrations::dsl::run_at.is_null())
            .order(schema_migrations::dsl::version.asc())
            .load::<Item>(self)?
        {
            info!("run migrate {}", it.up);
            self.batch_execute(&it.up)?;

            let it = schema_migrations::dsl::schema_migrations
                .filter(schema_migrations::dsl::id.eq(&it.id));
            update(it)
                .set(schema_migrations::dsl::run_at.eq(&now))
                .execute(self)?;
        }

        Ok(())
    }
    fn rollback(&self) -> Result<()> {
        match schema_migrations::dsl::schema_migrations
            .filter(schema_migrations::dsl::run_at.is_not_null())
            .order(schema_migrations::dsl::version.desc())
            .first::<Item>(self)
        {
            Ok(it) => {
                info!("rollback {}", it.down);
                self.batch_execute(&it.down)?;
                delete(
                    schema_migrations::dsl::schema_migrations
                        .filter(schema_migrations::dsl::id.eq(it.id)),
                )
                .execute(self)?;
            }
            Err(_) => warn!("database is empty"),
        };

        Ok(())
    }
    fn versions(&self) -> Result<Vec<Item>> {
        let items = schema_migrations::dsl::schema_migrations
            .order(schema_migrations::dsl::version.asc())
            .load(self)?;
        Ok(items)
    }
}
