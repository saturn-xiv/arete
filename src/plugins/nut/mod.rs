pub mod api;
pub mod html;
pub mod models;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgresql")]
pub mod postgresql;
pub mod seo;
#[cfg(feature = "sqlite")]
pub mod sqlite;
pub mod tasks;
pub mod themes;

use std::fmt;
use std::str::FromStr;

use failure::Error as FailureError;
use rocket::Route;

use super::super::errors::{Error, Result};

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;

pub enum MediaType {
    TEXT,
    HTML,
    MARKDOWN,
}

impl fmt::Display for MediaType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaType::TEXT => write!(fmt, "text"),
            MediaType::HTML => write!(fmt, "html"),
            MediaType::MARKDOWN => write!(fmt, "markdown"),
        }
    }
}

impl FromStr for MediaType {
    type Err = FailureError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(MediaType::TEXT),
            "markdown" => Ok(MediaType::MARKDOWN),
            "html" => Ok(MediaType::HTML),
            t => Err(Error::BadMediaType(t.to_string()).into()),
        }
    }
}

pub fn routes() -> (&'static str, Vec<Route>, Vec<Route>) {
    (
        "/",
        routes![
            api::oauth::google::get,
            api::oauth::google::post,
            api::oauth::google::get_sign_in,
            api::oauth::google::post_sign_in,
            api::users::sign_in,
            api::users::sign_up,
            api::users::confirm,
            api::users::confirm_token,
            api::users::unlock,
            api::users::unlock_token,
            api::users::forgot_password,
            api::users::reset_password,
            api::users::get_profile,
            api::users::set_profile,
            api::users::change_password,
            api::users::logs,
            api::users::sign_out,
            api::users::apply_authority,
            api::users::deny_authority,
            api::users::index_authority,
            api::users::show,
            api::users::index,
            api::attachments::show,
            api::attachments::index_by_administrator,
            api::attachments::index_by_owner,
            api::attachments::destroy,
            api::install,
            api::site::get_author,
            api::site::set_author,
            api::site::get_seo,
            api::site::set_seo,
            api::site::get_smtp,
            api::site::set_smtp,
            api::site::test_smtp,
            api::site::clear_cache,
            api::site::status::get,
            api::cards::create,
            api::cards::update,
            api::cards::show,
            api::cards::index,
            api::cards::destroy,
            api::links::create,
            api::links::update,
            api::links::show,
            api::links::index,
            api::links::destroy,
            api::categories::create,
            api::categories::update,
            api::categories::show,
            api::categories::index,
            api::categories::destroy,
            api::tags::create,
            api::tags::update,
            api::tags::show,
            api::tags::index,
            api::tags::destroy,
            api::votes::create,
            api::votes::index,
            api::votes::destroy,
            api::friend_links::create,
            api::friend_links::update,
            api::friend_links::show,
            api::friend_links::index,
            api::friend_links::destroy,
            api::leave_words::create,
            api::leave_words::index,
            api::leave_words::destroy,
            api::locales::create,
            api::locales::update,
            api::locales::show,
            api::locales::index,
            api::locales::destroy,
            api::locales::languages,
        ],
        routes![html::about, html::contact, html::index],
    )
}
