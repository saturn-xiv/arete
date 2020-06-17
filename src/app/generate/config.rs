use std::fs;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use serde::ser::Serialize;

use super::super::super::errors::Result;

pub const NAME: &str = "generate:config";

pub fn help<P: AsRef<Path>>(file: P) -> String {
    format!("Generate {}", file.as_ref().display())
}

pub fn run<P: AsRef<Path>, V: Serialize + Default>(file: P) -> Result<()> {
    let buf = toml::to_vec(&V::default())?;

    info!("generate file {}", file.as_ref().display());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(file)?;
    file.write_all(&buf)?;
    Ok(())
}
