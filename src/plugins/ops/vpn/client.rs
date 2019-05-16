use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;

use askama::Template;

use super::super::super::super::errors::Result;
use super::ROOT;

#[derive(Template)]
#[template(path = "openvpn/client.conf", escape = "none")]
pub struct Config<'a> {
    pub port: u16,
    pub host: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/auth.txt", escape = "none")]
pub struct Auth<'a> {
    pub user: &'a str,
    pub password: &'a str,
}

pub fn setup<'a>(
    cfg: &'a Config,
    auth: &'a Auth,
    ca: &'a str,
    cert: &'a str,
    key: &'a str,
) -> Result<()> {
    {
        let buf = cfg.render()?;
        let file = ROOT.join("client.conf");
        info!("generate file {}", file.display());
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(buf.as_bytes())?;
    }
    {
        let buf = auth.render()?;
        let file = ROOT.join("auth.txt");
        info!("generate file {}", file.display());
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(buf.as_bytes())?;
    }
    {
        let file = ROOT.join("ca.crt");
        info!("generate file {}", file.display());
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(ca.as_bytes())?;
    }
    {
        let file = ROOT.join("client.crt");
        info!("generate file {}", file.display());
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(cert.as_bytes())?;
    }
    {
        let file = ROOT.join("client.key");
        info!("generate file {}", file.display());
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(file)?;
        fd.write_all(key.as_bytes())?;
    }
    Ok(())
}
