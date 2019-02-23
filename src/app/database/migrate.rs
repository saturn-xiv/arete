use clap::{App, SubCommand};
use diesel::Connection as DieselConnection;
use failure::Error;

use super::super::super::{
    errors::Result,
    i18n,
    orm::{migration::Dao, Connection},
    plugins::{forum, nut, survey, vip},
    settings,
};

pub const COMMAND_NAME: &'static str = "database:migrate";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about("Migrate database to latest migration")
}

pub fn run(db: &Connection) -> Result<()> {
    db.transaction::<_, Error, _>(|| {
        db.load(&vec![
            i18n::locale::MIGRATION.clone(),
            settings::MIGRATION.clone(),
            nut::AUTH.clone(),
            nut::SITE.clone(),
            forum::MIGRATION.clone(),
            survey::MIGRATION.clone(),
            vip::MIGRATION.clone(),
        ])?;
        db.migrate()
    })
}
