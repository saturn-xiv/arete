// pub mod catchers;
// pub mod controllers;
pub mod models;
// pub mod request;
// pub mod tasks;
pub mod schema;

use std::fmt;
use std::str::FromStr;

use failure::Error as FailureError;

use super::super::{
    errors::{Error, Result},
    orm::migration::Migration,
};

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

lazy_static! {
    static ref AUTH: Migration = Migration {
        name: "create-auth",
        version: "20190101053052",
        up: include_str!("auth-up.sql"),
        down: include_str!("auth-down.sql"),
    };
}

lazy_static! {
    static ref SITE: Migration = Migration {
        name: "create-site",
        version: "20190101053059",
        up: include_str!("site-up.sql"),
        down: include_str!("site-down.sql"),
    };
}
