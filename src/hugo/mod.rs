pub mod bootstrap;
pub mod bulma;
pub mod materialiaze;
pub mod semantic_ui;

use std::fmt::Debug;

use chrono::NaiveDateTime;
use serde::Serialize;

use super::errors::Result;

#[derive(Debug, Clone, Serialize)]
pub struct Site {
    pub title: String,
    pub copyright: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub language: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub updated_at: NaiveDateTime,
}

pub trait Template: Serialize + Debug + Clone {}

pub fn generate<T: Template>(_tpl: &T) -> Result<()> {
    Ok(())
}
