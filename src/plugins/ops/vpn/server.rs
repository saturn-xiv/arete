use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use askama::Template;

use super::super::super::super::errors::Result;

lazy_static! {
    static ref ROOT: PathBuf = Path::new("/etc").join("openvpn").join("server");
}

#[derive(Template)]
#[template(path = "openvpn/server.conf", escape = "none")]
pub struct Config<'a> {
    pub port: u16,
    pub server: &'a ServerConfig<'a>,
    pub client: &'a ClientConfig<'a>,
}

pub struct ServerConfig<'a> {
    pub network: &'a str,
    pub ip: &'a str,
}

pub struct ClientConfig<'a> {
    pub network: &'a str,
    pub dns1: &'a str,
    pub dns2: &'a str,
}

impl<'a> Config<'a> {
    pub fn setup(&self) -> Result<()> {
        let cfg = self.render()?;
        {
            let file = ROOT.join("server.conf");
            info!("generate file {}", file.display());
            let mut fd = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(file)?;
            fd.write_all(cfg.as_bytes())?;
        }
        Ok(())
    }
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
