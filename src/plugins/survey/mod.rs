pub mod controllers;
pub mod models;
pub mod schema;

use super::super::orm::migration::Migration;

lazy_static! {
    static ref MIGRATION: Migration = Migration {
        name: "create-survey",
        version: "20190101053114",
        up: include_str!("up.sql"),
        down: include_str!("down.sql"),
    };
}
