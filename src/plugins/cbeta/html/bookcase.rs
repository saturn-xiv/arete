use std::path::PathBuf;

lazy_static! {
    pub static ref ROOT: PathBuf = super::super::ROOT
        .join("CBReader")
        .join("Bookcase")
        .join("CBETA");
}
