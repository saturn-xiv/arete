use std::path::PathBuf;

lazy_static! {
    pub static ref EPUB: PathBuf = super::super::ROOT.join("EPUB");
}
