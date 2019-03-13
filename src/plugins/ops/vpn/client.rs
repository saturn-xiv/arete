use std::fs::{remove_dir_all, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use askama::Template;

use super::super::super::super::errors::Result;

lazy_static! {
    static ref ROOT: PathBuf = Path::new("/etc").join("openvpn").join("client");
}

#[derive(Template)]
#[template(path = "openvpn/client.conf", escape = "none")]
pub struct Config<'a> {
    pub host: &'a str,
    pub port: u16,
    pub tcp: bool,
}

impl<'a> Config<'a> {
    pub fn clean(&self) -> Result<()> {
        remove_dir_all(&ROOT.as_path())?;
        Ok(())
    }
    pub fn setup(&self, ca: &str, cert: &str, key: &str) -> Result<()> {
        let cfg = self.render()?;
        {
            let file = ROOT.join("client.conf");
            info!("generate file {}", file.display());
            let mut fd = OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o600)
                .open(file)?;
            fd.write_all(cfg.as_bytes())?;
        }
        {
            let file = ROOT.join("ca.crt");
            info!("generate file {}", file.display());
            let mut fd = OpenOptions::new()
                .write(true)
                .create(true)
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
                .mode(0o600)
                .open(file)?;
            fd.write_all(key.as_bytes())?;
        }
        Ok(())
    }
}
