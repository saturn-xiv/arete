use clap::{App, SubCommand};

use super::super::super::{
    errors::Result,
    orm::{migration::Dao, Connection},
};

pub const COMMAND_NAME: &'static str = "database:status";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about("Show database schema status")
}

pub fn run(db: &Connection) -> Result<()> {
    println!("{:14} {:32} {}", "VERSION", "RUN ON", "NAME");
    for it in db.versions()? {
        println!("{}", it);
    }
    Ok(())
}
