pub mod admin;
pub mod users;

use juniper::GraphQLObject;

use super::super::super::{env::VERSION, errors::Result, graphql::context::Context, orm::Dao};

#[derive(GraphQLObject)]
#[graphql(description = "Site info")]
pub struct About {
    version: String,
    db: String,
}

impl About {
    pub fn new(ctx: &Context) -> Result<Self> {
        Ok(Self {
            version: VERSION.to_string(),
            db: ctx.db.version()?,
        })
    }
}
