pub mod models;
pub mod schema;

use super::super::orm::migration::Migration;

lazy_static! {
    static ref MIGRATION: Migration = Migration {
        name: "create-vip-members",
        version: "20190101053657",
        up: include_str!("up.sql"),
        down: include_str!("down.sql"),
    };
}
