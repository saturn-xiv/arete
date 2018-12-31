use std::path::Path;
use std::{fs, io::Read};

use encoding_rs;
use serde::de::DeserializeOwned;
use serde_xml_rs;
use toml;

use super::errors::Result;

pub fn from_xml<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let mut file = fs::File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let len = match encoding_rs::Encoding::for_bom(buf.as_slice()) {
        None => 0,
        Some((_, len)) => {
            debug!("find bom header {}", len);
            len
        }
    };
    let buf = &buf[len..];
    let it = serde_xml_rs::deserialize(buf)?;
    Ok(it)
}

pub fn from_toml<P: AsRef<Path>, T: DeserializeOwned>(file: P) -> Result<T> {
    let mut file = fs::File::open(file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let it = toml::from_slice(&buf)?;
    Ok(it)
}
