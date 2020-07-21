pub mod users;

use juniper::GraphQLObject;

use super::super::super::{env::VERSION, errors::Result, graphql::context::Context};

#[derive(GraphQLObject)]
#[graphql(description = "Site info")]
pub struct About {
    version: String,
}

impl About {
    pub fn new(_ctx: &Context) -> Result<Self> {
        Ok(Self {
            version: VERSION.to_string(),
        })
    }
}
