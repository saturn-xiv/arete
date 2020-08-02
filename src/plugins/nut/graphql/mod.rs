pub mod locales;
pub mod site;
pub mod users;

use std::ops::Deref;

use diesel::Connection;
use failure::Error;
use juniper::{GraphQLInputObject, GraphQLObject};
use validator::Validate;

use super::super::super::{
    env::VERSION,
    errors::Result,
    graphql::context::Context,
    i18n::{locale::Dao as LocaleDao, I18n},
};
use super::models::{
    log::Dao as LogDao,
    policy::{Dao as PolicyDao, Item as PolicyItem, Role},
    user::Dao as UserDao,
};

#[derive(GraphQLInputObject, Validate)]
pub struct Install {
    pub title: String,
    pub site: site::Info,
    pub administrator: users::SignUp,
}

impl Install {
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        self.validate()?;
        let db = ctx.db.deref();
        if UserDao::count(db)? > 0 {
            return Err(__i18n_e!(db, &ctx.locale, "nut.errors.db-not-empty"));
        }
        let user = self.administrator.save(ctx)?;

        let (nbf, exp) = PolicyItem::weeks(1 << 12);
        db.transaction::<_, Error, _>(move || {
            UserDao::confirm(db, user.id)?;
            __i18n_l!(
                db,
                user.id,
                &ctx.client_ip,
                &ctx.locale,
                "nut.logs.user.confirm"
            )?;
            for it in vec![Role::Admin, Role::Root] {
                PolicyDao::apply(db, user.id, &it, &None::<String>, &nbf, &exp)?;
                __i18n_l!(
                    db,
                    user.id,
                    &ctx.client_ip,
                    &ctx.locale,
                    "nut.logs.user.role.apply",
                    json!({ "name": it })
                )?;
            }
            Ok(())
        })?;
        self.site.save(db, &ctx.locale)?;
        Ok(())
    }
}

#[derive(GraphQLObject)]
pub struct About {
    version: String,
    languages: Vec<String>,
}

impl About {
    pub fn new(ctx: &Context) -> Result<Self> {
        let db = ctx.db.deref();
        Ok(Self {
            version: VERSION.to_string(),
            languages: LocaleDao::languages(db)?,
        })
    }
}
