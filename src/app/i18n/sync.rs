use std::ops::Deref;

use clap::{App, SubCommand};
use diesel::Connection as DieselConnection;
use failure::Error;

use super::super::super::{
    env::Config,
    errors::Result,
    i18n::locale::{Dao as LocaleDao, File},
};

pub const COMMAND_NAME: &'static str = "i18n:sync";
pub const COMMAND_ABOUT: &'static str = "Sync locale records to database";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about(COMMAND_ABOUT)
}

pub fn run(cfg: Config) -> Result<()> {
    let db = cfg.postgresql.open()?;
    let db = db.get()?;
    let db = db.deref();
    let (inserted, find) = db.transaction::<_, Error, _>(|| {
        LocaleDao::sync(
            db,
            &vec![
                File {
                    lang: "en-US",
                    body: include_str!("en-US.yml"),
                },
                File {
                    lang: "zh-Hans",
                    body: include_str!("zh-Hans.yml"),
                },
                File {
                    lang: "zh-Hant",
                    body: include_str!("zh-Hant.yml"),
                },
            ],
        )
    })?;
    info!("find {} records, insert {}", find, inserted);
    Ok(())
}
