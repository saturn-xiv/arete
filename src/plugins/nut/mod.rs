pub mod controllers;
pub mod models;

use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use actix::prelude::*;

use super::super::{
    env::Config,
    errors::{Error, Result},
    orm::DbExecutor,
};

pub struct State {
    pub config: Arc<Config>,
    pub db: Addr<DbExecutor>,
}

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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(MediaType::TEXT),
            "markdown" => Ok(MediaType::MARKDOWN),
            "html" => Ok(MediaType::HTML),
            t => Err(format!("unknown media type {}", t).into()),
        }
    }
}
