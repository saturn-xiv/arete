// pub mod controllers;
pub mod models;
pub mod schema;
pub mod themes;

use super::super::orm::migration::New as Migration;

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-forum",
        version: "20190101053108",
        up: include_str!("up.sql"),
        down: include_str!("down.sql"),
    };
}
