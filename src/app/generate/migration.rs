use clap::{App, SubCommand};

pub const COMMAND_NAME: &'static str = "generate:migration";
pub const COMMAND_ABOUT: &'static str = "Generate database migration files";
pub const ARG_SERVICE_NAME: &'static str = "name";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about(COMMAND_ABOUT)
        .arg(
            clap::Arg::with_name(ARG_SERVICE_NAME)
                .required(true)
                .short("n")
                .long("name")
                .value_name("MIGRATION_NAME")
                .help("Migration's name")
                .takes_value(true),
        )
}
