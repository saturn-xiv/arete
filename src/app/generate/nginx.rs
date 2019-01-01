use std::env::current_dir;
use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use clap::{App, SubCommand};
use mustache;

use super::super::super::errors::Result;

pub const COMMAND_NAME: &'static str = "generate:nginx";
pub const COMMAND_ABOUT: &'static str = "Generate nginx.conf";
pub const ARG_HTTPS: &'static str = "https";
pub const ARG_SERVER_NAME: &'static str = "server_name";

#[derive(Serialize)]
struct Config {
    name: String,
    port: u16,
    ssl: bool,
    root: String,
}

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

pub fn run(name: String, port: u16, ssl: bool) -> Result<()> {
    let tpl = mustache::compile_str(include_str!("nginx.conf.mu"))?;
    let cur = current_dir()?;

    let file = Path::new("tmp").join("nginx.conf");
    info!("generate file {}", file.display());
    let mut fd = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(file)?;
    tpl.render(
        &mut fd,
        &Config {
            name: name,
            port: port,
            ssl: ssl,
            root: format!("{}", cur.display()),
        },
    )?;
    Ok(())
}
