use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;
use clap::{App, SubCommand};
use nix::unistd::{Gid, Uid};

use super::super::super::{
    env::{DESCRIPTION, NAME},
    errors::Result,
};

pub const COMMAND_NAME: &str = "generate:systemd";
pub const COMMAND_ABOUT: &str = "Generate systemd service.conf";

#[derive(Template)]
#[template(path = "systemd/service.conf", escape = "none")]
struct Config<'a> {
    user: &'a str,
    group: &'a str,
    name: &'a str,
    root: &'a str,
    description: &'a str,
}

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about(COMMAND_ABOUT)
}

pub fn run() -> Result<()> {
    let cur = current_dir()?;
    let tpl = Config {
        user: &Uid::current().to_string(),
        group: &Gid::current().to_string(),
        name: NAME,
        description: DESCRIPTION,
        root: &format!("{}", cur.display()),
    }
    .render()?;

    let file = Path::new("tmp").join("systemd.service");
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    fd.write_all(tpl.as_bytes())?;
    info!("please copy it into /lib/systemd/system/ folder.");
    Ok(())
}
