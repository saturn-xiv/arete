#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate encoding_rs;
extern crate eui48;
extern crate failure;
extern crate futures;
extern crate git2;
extern crate hex;
extern crate hyper;
extern crate ipnetwork;
extern crate jsonwebtoken;
extern crate language_tags;
extern crate lapin_futures as lapin;
extern crate lettre;
extern crate lettre_email;
extern crate log4rs;
extern crate md5;
extern crate mime;
extern crate mustache;
extern crate nix;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate sodiumoxide;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate ssh2;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate validator;
extern crate xml;

pub mod app;
pub mod cache;
pub mod crypto;
pub mod env;
pub mod errors;
pub mod i18n;
pub mod jwt;
pub mod oauth;
pub mod orm;
pub mod parser;
pub mod plugins;
pub mod queue;
pub mod redis;
pub mod rfc;
pub mod settings;

#[cfg(target_os = "linux")]
pub mod sys;
