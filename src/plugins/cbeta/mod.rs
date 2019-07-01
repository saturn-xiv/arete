pub mod api;
pub mod html;
pub mod models;
pub mod themes;

use std::path::{Component, Path, PathBuf};

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new(&Component::RootDir).join("mnt").join("cbeta");
}
