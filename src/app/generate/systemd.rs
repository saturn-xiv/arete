use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;
use clap::{App, SubCommand};
use nix::unistd::{Gid, Uid};

use super::super::super::errors::Result;

pub const COMMAND_NAME: &'static str = "generate:systemd";
pub const COMMAND_ABOUT: &'static str = "Generate systemd service.conf";
pub const ARG_SERVICE_NAME: &'static str = "name";

#[derive(Template)]
#[template(path = "systemd.conf", escape = "none")]
struct Config<'a> {
    user: &'a str,
    group: &'a str,
    name: &'a str,
    root: &'a str,
    description: &'a str,
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
    let cur = current_dir()?;
    let tpl = Config {
        user: &Uid::current().to_string(),
        group: &Gid::current().to_string(),
        name: &name,
        description: &description,
        root: &format!("{}", cur.display()),
    }
    .render()?;

    let file = Path::new("tmp").join(format!("{}.service", name));
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    fd.write_all(tpl.as_bytes())?;
    Ok(())
}
