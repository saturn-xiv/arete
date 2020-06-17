pub mod ol;

use std::fs::File;

use failure::SyncFailure;

use super::super::super::super::super::errors::Result;
use super::super::ROOT;
use super::Head;

#[derive(Deserialize, Debug)]
#[serde(rename = "html")]
pub struct Html {
    pub head: Head,
    pub body: Body,
}

impl Html {
    pub fn new(name: &str) -> Result<Self> {
        let file = File::open(ROOT.join(name))?;
        let it = serde_xml_rs::from_reader(file).map_err(SyncFailure::new)?;
        Ok(it)
    }
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub nav: Nav,
}
#[derive(Deserialize, Debug)]
pub struct Nav {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "$value")]
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub enum Item {
    #[serde(rename = "span")]
    Span {
        #[serde(rename = "$value")]
        value: String,
    },
    #[serde(rename = "ol")]
    Ol {
        #[serde(rename = "li")]
        items: Vec<ol::Item>,
    },
    #[serde(rename = "li")]
    Li {},
}

#[derive(Deserialize, Debug)]
pub enum Link {
    #[serde(rename = "cblink")]
    Cb {
        #[serde(rename = "href")]
        href: String,
        #[serde(rename = "$value")]
        title: String,
    },
    #[serde(rename = "a")]
    A {
        #[serde(rename = "href")]
        href: String,
        #[serde(rename = "$value")]
        title: String,
    },
}
