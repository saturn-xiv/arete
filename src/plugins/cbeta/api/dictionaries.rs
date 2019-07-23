use super::super::ROOT as CBETA;
use std::path::PathBuf;

lazy_static! {
    pub static ref ROOT: PathBuf = CBETA.join("GoldenDict").join("13Dicts");
}
