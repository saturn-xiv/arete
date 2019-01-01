use clap::{App, SubCommand};
use diesel::Connection as DieselConnection;

use super::super::super::{
    errors::{Error, Result},
    orm::{migration::Migration, Connection},
};

pub const COMMAND_NAME: &'static str = "database:migrate";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about("Migrate database to latest migration")
}

pub fn run(db: &Connection) -> Result<()> {
    db.transaction::<_, Error, _>(|| {
        db.load()?;
        db.migrate()
    })
}
