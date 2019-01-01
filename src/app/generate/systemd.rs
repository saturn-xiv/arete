use std::env::current_dir;
use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use clap::{App, SubCommand};
use nix::unistd::{Gid, Uid};

use super::super::super::errors::Result;

pub const COMMAND_NAME: &'static str = "generate:systemd";
pub const COMMAND_ABOUT: &'static str = "Generate systemd service.conf";
pub const ARG_SERVICE_NAME: &'static str = "name";

#[derive(Serialize)]
struct Config {
    user: String,
    group: String,
    name: String,
    root: String,
    description: String,
}

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about(COMMAND_ABOUT)
        .arg(
            clap::Arg::with_name(ARG_SERVICE_NAME)
                .required(true)
                .short("n")
                .long("name")
                .value_name("SERVICE_NAME")
                .help("HTTP server name")
                .takes_value(true),
        )
}

pub fn run(name: String, description: String) -> Result<()> {
    let tpl = mustache::compile_str(include_str!("systemd.conf.mu"))?;
    let cur = current_dir()?;

    let file = Path::new("tmp").join(format!("{}.service", name));
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    tpl.render(
        &mut fd,
        &Config {
            user: Uid::current().to_string(),
            group: Gid::current().to_string(),
            name: name.clone(),
            description: description,
            root: format!("{}", cur.display()),
        },
    )?;
    Ok(())
}
