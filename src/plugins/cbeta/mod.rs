pub mod html;
pub mod models;

use std::path::{Path, PathBuf};

lazy_static! {
    // pub static ref ROOT: PathBuf = Path::new(&Component::RootDir).join("mnt").join("cbeta");
    pub static ref ROOT: PathBuf = Path::new("tmp").join("cbeta");
}
