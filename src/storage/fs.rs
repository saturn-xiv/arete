use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::Utc;

use super::super::errors::Result;

pub struct FileSystem {}

impl FileSystem {
    pub fn root() -> PathBuf {
        Path::new("tmp").join("upload")
    }
}

impl super::Storage for FileSystem {
    fn save(&self, name: &str, body: &[u8]) -> Result<String> {
        let now = Utc::now();
        let mut it =
            Path::new(&now.format("%F").to_string()).join(&now.format("%H-%M-%S-%f").to_string());
        if let Some(ext) = Path::new(name).extension() {
            it = it.with_extension(ext);
        }
        let file = Self::root().join(&it);
        if let Some(dir) = file.parent() {
            create_dir_all(dir)?;
        }
        let mut file = File::create(file)?;
        file.write_all(body)?;

        Ok(format!("/upload/{}", it.display()))
    }
}
