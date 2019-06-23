use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use super::super::super::storage::fs::FileSystem;
use super::themes::{About, Contact, Index};

#[get("/")]
pub fn index() -> Index {
    Index {}
}

#[get("/about")]
pub fn about() -> About {
    About {}
}

#[get("/contact")]
pub fn contact() -> Contact {
    Contact {}
}

#[get("/upload/<file..>")]
pub fn upload(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(FileSystem::root().join(file)).ok()
}

#[get("/3rd/<file..>")]
pub fn third(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("node_modules").join(file)).ok()
}

#[get("/assets/<file..>")]
pub fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join(file)).ok()
}
