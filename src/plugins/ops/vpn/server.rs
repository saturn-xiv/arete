use std::fs;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;

use super::super::super::super::errors::Result;

#[derive(Template)]
#[template(path = "openvpn/server.conf", escape = "none")]
pub struct Config<'a> {
    pub port: u16,
    pub tcp: bool,
    pub network: &'a str,
    pub netmask: &'a str,
}

pub fn setup<'a>(cfg: &'a Config) -> Result<()> {
    let root = Path::new("/etc").join("openvpn").join("server");
    let cfg = cfg.render()?;
    {
        let file = root.join("server.conf");
        info!("generate file {}", file.display());
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(cfg.as_bytes())?;
    }
    Ok(())
}

pub fn create() -> Result<()> {
    Ok(())
}

pub fn update() -> Result<()> {
    Ok(())
}

pub fn delete() -> Result<()> {
    Ok(())
}

pub fn list() -> Result<()> {
    Ok(())
}
