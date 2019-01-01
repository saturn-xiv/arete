use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use chrono::Utc;
use clap::{App, SubCommand};

use super::super::super::errors::Result;

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

pub fn run(name: String) -> Result<()> {
    let dir = Path::new("db").join("migrations").join(format!(
        "{}-{}",
        Utc::now().format("%Y%m%d%H%M%S").to_string(),
        name
    ));
    fs::create_dir(&dir)?;
    for it in vec!["up", "down"] {
        let mut file = dir.join(it);
        file.set_extension("sql");
        info!("generate file {}", file.display());
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
    }

    Ok(())
}
