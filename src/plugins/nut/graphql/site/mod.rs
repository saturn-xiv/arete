pub mod status;

use std::ops::Deref;

use validator::Validate;

use super::super::super::super::{
    cache::Cache,
    crypto::sodium::Encryptor as Sodium,
    errors::Result,
    graphql::{context::Context, session::Session, Handler},
    settings::Dao as SettingDao,
};

#[derive(GraphQLInputObject, Validate, Serialize, Deserialize)]
pub struct Author {
    #[validate(length(min = "1"))]
    pub name: String,
    #[validate(email, length(min = "1"))]
    pub email: String,
}

impl Author {
    const KEY: &'static str = "site.author";
}

impl Default for Author {
    fn default() -> Self {
        Self {
            name: "who-am-i".to_string(),
            email: "change-me@gmail.com".to_string(),
        }
    }
}

impl Handler for Author {
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        SettingDao::set::<String, Author, Sodium>(
            db,
            &c.encryptor,
            &Self::KEY.to_string(),
            &self,
            false,
        )?;
        Ok(None)
    }
}

#[derive(Validate)]
pub struct GetAuthor {}

impl Handler for GetAuthor {
    type Item = Author;
    fn handle(&self, c: &Context, _s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let enc = c.encryptor.deref();
        let it: Author = match SettingDao::get(db, enc, &Author::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => Author::default(),
        };
        Ok(it)
    }
}

#[derive(GraphQLInputObject, Validate, Serialize, Deserialize)]
pub struct Seo {
    pub google: Option<Google>,
    pub baidu: Option<Baidu>,
}

#[derive(GraphQLInputObject, Validate, Serialize, Deserialize)]
pub struct Google {
    #[validate(length(min = "1"))]
    pub verify_id: String,
}

#[derive(GraphQLInputObject, Validate, Serialize, Deserialize)]
pub struct Baidu {
    #[validate(length(min = "1"))]
    pub verify_id: String,
}

impl Seo {
    const KEY: &'static str = "site.seo";
}

impl Default for Seo {
    fn default() -> Self {
        Self {
            google: None,
            baidu: None,
        }
    }
}

impl Handler for Seo {
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let enc = c.encryptor.deref();
        s.administrator(db)?;
        SettingDao::set::<String, Seo, Sodium>(db, enc, &Self::KEY.to_string(), &self, false)?;
        Ok(None)
    }
}

#[derive(Validate)]
pub struct GetSeo {}

impl Handler for GetSeo {
    type Item = Seo;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        let enc = c.encryptor.deref();
        s.administrator(db)?;
        let it: Seo = match SettingDao::get(db, enc, &Seo::KEY.to_string()) {
            Ok(v) => v,
            Err(_) => Seo::default(),
        };
        Ok(it)
    }
}

#[derive(Validate)]
pub struct ClearCache {}

impl Handler for ClearCache {
    type Item = Option<String>;
    fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
        let db = c.db.deref();
        s.administrator(db)?;
        c.cache.clear()?;
        Ok(None)
    }
}
