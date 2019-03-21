// pub mod attachments;
// pub mod cards;
// pub mod categories;
// pub mod friend_links;
// pub mod leave_words;
// pub mod links;
// pub mod site;
// pub mod tags;
pub mod users;
// pub mod votes;

// use std::ops::Deref;

// use diesel::prelude::*;
// use failure::Error;
// use validator::Validate;

// use super::super::super::{
//     crypto::sodium::Encryptor as Sodium,
//     errors::Result,
//     graphql::{context::Context, session::Session, Handler},
//     i18n::I18n,
// };
// use super::models::{
//     log::Dao as LogDao,
//     policy::{Dao as PolicyDao, Item as Policy, Role},
//     user::Dao as UserDao,
// };

// #[derive(GraphQLInputObject, Validate)]
// pub struct Install {
//     #[validate(length(min = "1", max = "32"))]
//     pub real_name: String,
//     #[validate(email, length(min = "2", max = "64"))]
//     pub email: String,
//     #[validate(length(min = "6", max = "32"))]
//     pub password: String,
// }

// impl Handler for Install {
//     type Item = ();
//     fn handle(&self, c: &Context, s: &Session) -> Result<Self::Item> {
//         let db = c.db()?;
//         let db = db.deref();

//         db.transaction::<_, Error, _>(|| {
//             if UserDao::count(db)? > 0 {
//                 return __i18n_e!(db, &s.lang, "nut.errors.database-is-not-empty");
//             }
//             UserDao::sign_up::<Sodium>(
//                 db,
//                 &self.real_name,
//                 &"admin".to_string(),
//                 &self.email,
//                 &self.password,
//             )?;
//             let it = UserDao::by_email(db, &self.email)?;
//             UserDao::confirm(db, &it.id)?;
//             let (nbf, exp) = Policy::weeks(1 << 12);
//             PolicyDao::apply(db, &it.id, &Role::Root, &None::<String>, &nbf, &exp)?;
//             PolicyDao::apply(db, &it.id, &Role::Admin, &None::<String>, &nbf, &exp)?;
//             __i18n_l!(db, &it.id, &s.client_ip, &s.lang, "nut.logs.init-database")?;

//             Ok(())
//         })?;

//         Ok(())
//     }
// }
