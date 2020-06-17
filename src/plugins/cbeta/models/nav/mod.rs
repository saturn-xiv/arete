pub mod menu;
pub mod simple;

use super::Text;

#[derive(Deserialize, Debug)]
pub struct Head {
    pub meta: Meta,
    pub title: Text,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub charset: String,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub href: String,
    #[serde(rename = "$value")]
    pub title: String,
}
