#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;

extern crate actix_files;
extern crate actix_web;
extern crate amq_protocol_uri;
extern crate askama;
extern crate base64;
extern crate bytesize;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate csv;
extern crate encoding_rs;
extern crate eui48;
extern crate futures;
extern crate futures_executor;
extern crate futures_util;
extern crate git2;
extern crate hex;
extern crate humantime;
extern crate ipnetwork;
extern crate jsonwebtoken;
extern crate juniper;
extern crate lettre;
extern crate lettre_email;
extern crate md5;
extern crate mime;
extern crate mustache;
extern crate nix;
extern crate paho_mqtt;
extern crate percent_encoding;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate rusoto_sqs;
extern crate serde;
extern crate serde_xml_rs;
extern crate sodiumoxide;
extern crate ssh2;
extern crate tempfile;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate validator;
extern crate xml;
extern crate yaml_rust;
extern crate zmq;

#[macro_use]
pub mod macros;

pub mod app;
pub mod cache;
pub mod crypto;
pub mod dict;
pub mod env;
pub mod errors;
pub mod graphql;
pub mod hugo;
pub mod i18n;
pub mod jwt;
pub mod nav;
pub mod oauth;
pub mod orm;
pub mod parser;
pub mod plugins;
pub mod queue;
pub mod request;
pub mod rfc;
pub mod settings;
pub mod storage;
pub mod sys;
pub mod theme;

use chrono::{NaiveDateTime, Utc};

lazy_static! {
    pub static ref STARTUP: NaiveDateTime = Utc::now().naive_local();
}

pub const XML_HEADER: &str = r###"<?xml version="1.0" encoding="utf-8" ?>"###;
pub const VIEWS_ROOT: &str = "themes";
