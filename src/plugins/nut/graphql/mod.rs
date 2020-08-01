pub mod admin;
pub mod users;

use juniper::GraphQLObject;
use nix::sys::utsname::uname;

use super::super::super::{env::VERSION, errors::Result, graphql::context::Context, orm::Dao};

#[derive(GraphQLObject)]
pub struct About {
    version: String,
    db: String,
    os: String,
}

impl About {
    pub fn new(ctx: &Context) -> Result<Self> {
        let uts = uname();
        Ok(Self {
            version: VERSION.to_string(),
            db: ctx.db.version()?,
            os: format!(
                "{} {} {} {} {}",
                uts.sysname(),
                uts.nodename(),
                uts.release(),
                uts.version(),
                uts.machine()
            ),
        })
    }
}
