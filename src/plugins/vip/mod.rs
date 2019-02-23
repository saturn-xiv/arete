pub mod graphql;
pub mod models;
pub mod schema;

use super::super::orm::migration::New as Migration;

lazy_static! {
    pub static ref MIGRATION: Migration<'static> = Migration {
        name: "create-vip-members",
        version: "20190101053657",
        up: include_str!("up.sql"),
        down: include_str!("down.sql"),
    };
}
