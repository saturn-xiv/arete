#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
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

extern crate amq_protocol_uri;
extern crate askama;
extern crate base64;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate cookie;
extern crate csv;
extern crate encoding_rs;
extern crate eui48;
extern crate futures;
extern crate git2;
extern crate hex;
extern crate hyper;
extern crate jsonwebtoken;
extern crate language_tags;
extern crate lapin_futures as lapin;
extern crate lettre;
extern crate lettre_email;
extern crate log4rs;
extern crate md5;
extern crate mime;
extern crate multipart;
extern crate mustache;
extern crate nix;
extern crate ntp;
extern crate paho_mqtt as mqtt;
extern crate r2d2;
#[cfg(feature = "redis")]
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
#[cfg(feature = "sodium")]
extern crate sodiumoxide;
extern crate ssh2;
extern crate tempfile;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
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
pub mod catchers;
pub mod crypto;
pub mod dict;
pub mod env;
pub mod errors;
pub mod i18n;
pub mod jwt;
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
pub mod themes;
