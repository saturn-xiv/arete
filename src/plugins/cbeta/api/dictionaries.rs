use std::path::PathBuf;

use super::super::ROOT as CBETA;

lazy_static! {
    pub static ref ROOT: PathBuf = CBETA.join("GoldenDict").join("13Dicts");
}
