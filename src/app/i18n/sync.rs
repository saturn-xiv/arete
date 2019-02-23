use std::ops::Deref;

use clap::{App, SubCommand};
use diesel::Connection as DieselConnection;
use failure::Error;

use super::super::super::{env::Config, errors::Result, i18n::locale::Dao as LocaleDao};

pub const COMMAND_NAME: &'static str = "i18n:sync";
pub const COMMAND_ABOUT: &'static str = "Sync locales from directory";
pub const ARG_DIR_NAME: &'static str = "dir";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about(COMMAND_ABOUT)
        .arg(
            clap::Arg::with_name(ARG_DIR_NAME)
                .required(true)
                .short("d")
                .long("dir")
                .value_name("LOCALES_DIR")
                .help("Directory name")
                .takes_value(true),
        )
}

pub fn run(cfg: Config, dir: String) -> Result<()> {
    let db = cfg.postgresql.open()?;
    let db = db.get()?;
    let (inserted, find) = db.transaction::<_, Error, _>(|| LocaleDao::sync(db.deref(), &dir))?;
    info!("find {} records, insert {}", find, inserted);
    Ok(())
}
