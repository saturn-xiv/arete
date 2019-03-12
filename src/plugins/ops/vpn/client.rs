use std::fs;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use askama::Template;

use super::super::super::super::errors::Result;

#[derive(Template)]
#[template(path = "openvpn/client.conf", escape = "none")]
pub struct Config<'a> {
    pub host: &'a str,
    pub port: u16,
    pub tcp: bool,
}

pub fn setup<'a>(cfg: &'a Config, ca: &'a str, cert: &'a str, key: &'a str) -> Result<()> {
    let root = Path::new("/etc").join("openvpn").join("client");
    let cfg = cfg.render()?;
    {
        let file = root.join("client.conf");
        info!("generate file {}", file.display());
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(cfg.as_bytes())?;
    }
    {
        let file = root.join("ca.crt");
        info!("generate file {}", file.display());
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(ca.as_bytes())?;
    }
    {
        let file = root.join("client.crt");
        info!("generate file {}", file.display());
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(cert.as_bytes())?;
    }
    {
        let file = root.join("client.key");
        info!("generate file {}", file.display());
        let mut fd = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(key.as_bytes())?;
    }
    Ok(())
}
