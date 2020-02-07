use clap::{App, SubCommand};
use diesel::Connection as DieselConnection;
use failure::Error;

use super::super::super::{
    errors::Result,
    i18n,
    orm::{
        migration::{Dao, New as Migration},
        Connection,
    },
    plugins::{forum, nut, ops::vpn, survey, vip},
    settings,
};

pub const COMMAND_NAME: &str = "database:migrate";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about("Migrate database to latest migration")
}

/// date +"%Y%m%d%H%M%S"
pub fn run(db: &Connection) -> Result<()> {
    db.transaction::<_, Error, _>(|| {
        db.load(&[
            Migration {
                name: "create-locales",
                version: "20190101053014",
                up: i18n::UP,
                down: i18n::DOWN,
            },
            Migration {
                name: "create-settings",
                version: "20190101053042",
                up: settings::UP,
                down: settings::DOWN,
            },
            Migration {
                name: "create-auth",
                version: "20190101053052",
                up: nut::AUTH_UP,
                down: nut::AUTH_DOWN,
            },
            Migration {
                name: "create-site",
                version: "20190101053059",
                up: nut::SITE_UP,
                down: nut::SITE_DOWN,
            },
            Migration {
                name: "create-forum",
                version: "20190101053108",
                up: forum::UP,
                down: forum::DOWN,
            },
            Migration {
                name: "create-survey",
                version: "20190101053114",
                up: survey::UP,
                down: survey::DOWN,
            },
            Migration {
                name: "create-vip-members",
                version: "20190101053657",
                up: vip::UP,
                down: vip::DOWN,
            },
            Migration {
                name: "create-ops-vpn",
                version: "20190514084629",
                up: vpn::UP,
                down: vpn::DOWN,
            },
        ])?;
        db.migrate()
    })
}
