use std::fs::File;
use std::io::prelude::*;

use encoding_rs::UTF_16LE;
use failure::SyncFailure;

use super::super::super::super::errors::Result;
use super::{Src, Text, ROOT};

#[derive(Deserialize, Debug)]
#[serde(rename = "metadata")]
pub struct Metadata {
    pub id: Text,
    pub title: Text,
    pub language: Text,
    pub identifier: Identifier,
    pub subject: Text,
    pub publisher: Text,
    pub date: Text,
    pub cover: Src,
    pub catalog: Src,
    pub nav: Src,
    pub spine: Src,
    pub bookdata: Src,
    pub javascript: Src,
    pub version: Text,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: String,
    #[serde(rename = "$value")]
    pub value: String,
}

impl Metadata {
    pub fn new() -> Result<Self> {
        let file = File::open(ROOT.join("index.xml"))?;
        let it = serde_xml_rs::from_reader(file).map_err(SyncFailure::new)?;
        Ok(it)
    }
    pub fn bookdata(&self) -> Result<Vec<super::bookdata::Item>> {
        let mut items = Vec::new();
        let mut file = File::open(ROOT.join(&self.bookdata.value))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let (buf, _, _) = UTF_16LE.decode(&buf.as_slice());

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .from_reader(buf.as_bytes());
        for it in rdr.deserialize() {
            items.push(it?);
        }
        Ok(items)
    }

    pub fn catalog(&self) -> Result<Vec<super::catalog::Item>> {
        let mut items = Vec::new();
        let file = File::open(ROOT.join(&self.catalog.value))?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .from_reader(file);
        for it in rdr.deserialize() {
            items.push(it?);
        }
        Ok(items)
    }

    pub fn spine(&self) -> Result<Vec<super::spine::Item>> {
        let mut items = Vec::new();
        let file = File::open(super::ROOT.join(&self.spine.value))?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .trim(csv::Trim::All)
            .from_reader(file);
        for it in rdr.deserialize() {
            items.push(it?);
        }
        Ok(items)
    }
}
