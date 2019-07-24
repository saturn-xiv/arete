pub mod bookdata;
pub mod catalog;
pub mod metadata;
pub mod spine;

use std::path::PathBuf;

use super::ROOT as CBETA;

lazy_static! {
    pub static ref ROOT: PathBuf = CBETA.join("Bookcase").join("CBETA");
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Src {
    #[serde(rename = "src")]
    pub value: String,
}
