use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use yaml_rust::{Yaml, YamlLoader};

use super::super::{
    errors::Result,
    orm::{schema::locales, Connection},
};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i64,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "locales"]
pub struct New<'a> {
    pub lang: &'a str,
    pub code: &'a str,
    pub message: &'a str,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn sync<P: AsRef<Path>>(&self, root: P) -> Result<(usize, usize)>;
    fn languages(&self) -> Result<Vec<String>>;
    fn count(&self, lang: &String) -> Result<i64>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_lang(&self, lang: &String) -> Result<Vec<Item>>;
    fn by_id(&self, id: &i64) -> Result<Item>;
    fn by_lang_and_code(&self, lang: &String, code: &String) -> Result<Item>;
    fn delete(&self, id: &i64) -> Result<()>;
    fn create(&self, lang: &String, code: &String, message: &String) -> Result<()>;
    fn update(&self, id: &i64, lang: &String, code: &String, message: &String) -> Result<()>;
}

fn loop_yaml(
    db: &Connection,
    lang: &str,
    prefix: Option<String>,
    node: Yaml,
) -> Result<(usize, usize)> {
    let mut finded = 0;
    let mut inserted = 0;
    let sep = ".";
    match node {
        Yaml::String(v) => {
            let k = match prefix {
                Some(p) => p,
                None => "".to_string(),
            };
            // debug!("find {} {} => {}", lang, k, v);
            finded += 1;

            let cnt: i64 = locales::dsl::locales
                .count()
                .filter(locales::dsl::lang.eq(lang))
                .filter(locales::dsl::code.eq(&k))
                .get_result(db)?;
            if cnt == 0 {
                inserted += 1;
                insert_into(locales::dsl::locales)
                    .values(&New {
                        lang: lang,
                        code: &k,
                        message: &v,
                        updated_at: &Utc::now().naive_utc(),
                    })
                    .execute(db)?;
            }
        }
        Yaml::Hash(h) => {
            for (k, v) in h {
                match k {
                    Yaml::String(k) => {
                        let (i, f) = loop_yaml(
                            db,
                            lang,
                            Some(match prefix {
                                Some(ref p) => p.clone() + sep + &k,
                                None => k,
                            }),
                            v,
                        )?;
                        inserted += i;
                        finded += f;
                    }
                    k => {
                        error!("bad key {:?}", k);
                    }
                }
            }
        }
        k => {
            error!("bad key {:?}", k);
        }
    };
    Ok((inserted, finded))
}

impl Dao for Connection {
    fn sync<P: AsRef<Path>>(&self, root: P) -> Result<(usize, usize)> {
        let mut finded = 0;
        let mut inserted = 0;
        let ext = "yml";

        for it in fs::read_dir(root)? {
            let it = it?.path();
            if Some(OsStr::new(ext)) == it.extension() {
                let buf = fs::read_to_string(&it)?;
                if let Some(name) = it.file_name() {
                    if let Some(name) = name.to_str() {
                        let lang = &name[..(name.len() - ext.len() - 1)];
                        info!("find locale {}", lang);
                        for it in YamlLoader::load_from_str(&buf)? {
                            let (i, f) = loop_yaml(&self, lang, None, it)?;
                            inserted += i;
                            finded += f;
                        }
                    }
                }
            }
        }

        Ok((inserted, finded))
    }

    fn languages(&self) -> Result<Vec<String>> {
        Ok(locales::dsl::locales
            .select(locales::dsl::lang)
            .distinct()
            .load::<String>(self)?)
    }

    fn count(&self, lang: &String) -> Result<i64> {
        let cnt: i64 = locales::dsl::locales
            .count()
            .filter(locales::dsl::lang.eq(lang))
            .get_result(self)?;
        Ok(cnt)
    }
    fn by_lang(&self, lang: &String) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .order(locales::dsl::code.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn all(&self) -> Result<Vec<Item>> {
        let items = locales::dsl::locales
            .order(locales::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_id(&self, id: &i64) -> Result<Item> {
        let it = locales::dsl::locales
            .filter(locales::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn by_lang_and_code(&self, lang: &String, code: &String) -> Result<Item> {
        let it = locales::dsl::locales
            .filter(locales::dsl::lang.eq(lang))
            .filter(locales::dsl::code.eq(code))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(&self, id: &i64, lang: &String, code: &String, message: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = locales::dsl::locales.filter(locales::dsl::id.eq(id));
        update(it)
            .set((
                locales::dsl::lang.eq(lang),
                locales::dsl::code.eq(code),
                locales::dsl::message.eq(message),
                locales::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn create(&self, lang: &String, code: &String, message: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(locales::dsl::locales)
            .values(&New {
                lang: lang,
                code: code,
                message: message,
                updated_at: &now,
            })
            .execute(self)?;
        Ok(())
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(locales::dsl::locales.filter(locales::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
