use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;
use clap::{App, SubCommand};

use super::super::super::{env::NAME, errors::Result};

pub const COMMAND_NAME: &str = "generate:nginx";
pub const COMMAND_ABOUT: &str = "Generate nginx.conf";
pub const ARG_HTTPS: &str = "https";
pub const ARG_SERVER_NAME: &str = "server_name";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about(COMMAND_ABOUT)
        .arg(
            clap::Arg::with_name(ARG_HTTPS)
                .short("s")
                .long("ssl")
                .value_name("HTTPS")
                .help("Enable https?")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name(ARG_SERVER_NAME)
                .required(true)
                .short("n")
                .long("name")
                .value_name("SERVER_NAME")
                .help("HTTP server name")
                .takes_value(true),
        )
}

#[derive(Template)]
#[template(path = "nginx.conf", escape = "none")]
struct Config<'a> {
    name: &'a str,
    domain: &'a str,
    root: &'a str,
    port: u16,
    ssl: bool,
}

pub fn run(domain: String, port: u16, ssl: bool) -> Result<()> {
    let cur = current_dir()?;
    let tpl = Config {
        domain: &domain,
        name: NAME,
        port,
        ssl,
        root: &format!("{}", cur.display()),
    }
    .render()?;

    let file = Path::new("tmp").join("nginx.conf");
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    fd.write_all(tpl.as_bytes())?;

    Ok(())
}
