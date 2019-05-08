use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::{Component, Path};
use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use failure::Error;

use super::super::super::errors::Result;

/// https://man.openbsd.org/dhclient.leases.5
/// http://manpages.ubuntu.com/manpages/trusty/man5/dhclient.conf.5.html
/// https://www.ietf.org/assignments/bootp-dhcp-parameters/bootp-dhcp-parameters.xml
/// Append `also request www-server;` /etc/dhcp/dhclient.conf
#[derive(Serialize, Eq, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    pub interface: String,
    pub fixed_address: String,
    pub subnet_mask: Option<String>,
    pub routers: Option<Vec<String>>,
    pub dhcp_lease_time: Option<u64>,
    pub dhcp_message_type: Option<u32>,
    pub domain_name_servers: Option<Vec<String>>,
    pub dhcp_server_identifier: Option<String>,
    pub dhcp_renewal_time: Option<u64>,
    pub dhcp_rebinding_time: Option<u64>,
    pub interface_mtu: Option<u32>,
    pub broadcast_address: Option<String>,
    pub host_name: Option<String>,
    pub domain_name: Option<String>,
    pub www_server: Option<Vec<String>>,
    pub renew: NaiveDateTime,
    pub rebind: NaiveDateTime,
    pub expire: NaiveDateTime,
}

impl Ord for Lease {
    fn cmp(&self, other: &Lease) -> Ordering {
        self.renew.cmp(&other.renew)
    }
}

impl PartialOrd for Lease {
    fn partial_cmp(&self, other: &Lease) -> Option<Ordering> {
        self.renew.partial_cmp(&other.renew)
    }
}
impl PartialEq for Lease {
    fn eq(&self, other: &Lease) -> bool {
        self.renew == other.renew && self.interface == other.interface
    }
}

impl Default for Lease {
    fn default() -> Self {
        let now = Utc::now().naive_utc();
        Self {
            interface: "lo".to_string(),
            fixed_address: "127.0.0.1".to_string(),
            subnet_mask: None,
            routers: None,
            dhcp_lease_time: None,
            dhcp_message_type: None,
            domain_name_servers: None,
            dhcp_server_identifier: None,
            dhcp_rebinding_time: None,
            dhcp_renewal_time: None,
            interface_mtu: None,
            broadcast_address: None,
            host_name: None,
            domain_name: None,
            www_server: None,
            renew: now,
            rebind: now,
            expire: now,
        }
    }
}

impl Lease {
    pub fn new() -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let ext = OsStr::new("lease");

        // check for ubuntu 18.04
        let root = Path::new(&Component::RootDir)
            .join("var")
            .join("lib")
            .join("NetworkManager");
        if root.exists() {
            for entry in read_dir(root)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(v) = path.extension() {
                        if v == ext {
                            items.append(&mut Self::isc(&path)?);
                        }
                    }
                }
            }
        }

        items.sort_by(|a, b| b.cmp(a));
        Ok(items)
    }

    pub fn isc<P: AsRef<Path>>(file: P) -> Result<Vec<Lease>> {
        let file = file.as_ref();
        debug!("load leases from {}", file.display());
        let file = File::open(file)?;
        let file = BufReader::new(&file);
        let mut items = Vec::new();

        let mut it = Lease::default();
        for line in file.lines() {
            match line?.parse::<Line>()? {
                Line::Begin => {
                    it = Lease::default();
                }
                Line::End => {
                    items.push(it.clone());
                }
                Line::Interface(v) => {
                    it.interface = v;
                }
                Line::Expire(v) => {
                    it.expire = v;
                }
                Line::Rebind(v) => {
                    it.rebind = v;
                }
                Line::Renew(v) => {
                    it.renew = v;
                }
                Line::FixedAddress(v) => {
                    it.fixed_address = v;
                }
                Line::Option { name, value } => match &name[..] {
                    "routers" => {
                        it.routers =
                            Some(value.split(Line::DIVIDER).map(|x| x.to_string()).collect());
                    }
                    "subnet-mask" => {
                        it.subnet_mask = Some(value);
                    }
                    "dhcp-lease-time" => {
                        it.dhcp_lease_time = Some(value.parse()?);
                    }
                    "dhcp-message-type" => {
                        it.dhcp_message_type = Some(value.parse()?);
                    }
                    "domain-name-servers" => {
                        it.domain_name_servers =
                            Some(value.split(Line::DIVIDER).map(|x| x.to_string()).collect());
                    }
                    "dhcp-server-identifier" => {
                        it.dhcp_server_identifier = Some(value);
                    }
                    "interface-mtu" => {
                        it.dhcp_message_type = Some(value.parse()?);
                    }
                    "broadcast-address" => {
                        it.broadcast_address = Some(value);
                    }
                    "host-name" => {
                        it.host_name = Some(value);
                    }
                    "domain-name" => {
                        it.domain_name = Some(value);
                    }
                    "www-server" => {
                        it.www_server =
                            Some(value.split(Line::DIVIDER).map(|x| x.to_string()).collect());
                    }
                    _ => {
                        warn!("unknown dhcp option {} {}", name, value);
                    }
                },
            }
        }

        Ok(items)
    }
}

pub enum Line {
    Begin,
    End,
    Interface(String),
    FixedAddress(String),
    Option { name: String, value: String },
    Renew(NaiveDateTime),
    Rebind(NaiveDateTime),
    Expire(NaiveDateTime),
}

impl Line {
    pub const DATE_TIME_FORMAT: &'static str = "%w %Y/%m/%d %T";
    pub const DIVIDER: &'static str = ",";
    fn detect(s: &str, p: &str) -> Option<String> {
        let p = format!("{} ", p);
        if s.starts_with(&p) {
            let mut s = &s[p.len()..s.len() - 1];
            if s.starts_with('"') {
                s = &s[1..];
            }
            if s.ends_with('"') {
                s = &s[..s.len() - 1];
            }
            return Some(s.to_string());
        }
        None
    }
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self> {
        let line = line.trim();
        if line == "lease {" {
            return Ok(Line::Begin);
        }
        if line == "}" {
            return Ok(Line::End);
        }
        if let Some(it) = Self::detect(line, "interface") {
            return Ok(Line::Interface(it));
        }
        if let Some(it) = Self::detect(line, "fixed-address") {
            return Ok(Line::Interface(it));
        }
        if let Some(it) = Self::detect(line, "renew") {
            return Ok(Line::Renew(NaiveDateTime::parse_from_str(
                &it,
                Self::DATE_TIME_FORMAT,
            )?));
        }
        if let Some(it) = Self::detect(line, "rebind") {
            return Ok(Line::Rebind(NaiveDateTime::parse_from_str(
                &it,
                Self::DATE_TIME_FORMAT,
            )?));
        }
        if let Some(it) = Self::detect(line, "expire") {
            return Ok(Line::Expire(NaiveDateTime::parse_from_str(
                &it,
                Self::DATE_TIME_FORMAT,
            )?));
        }
        if let Some(it) = Self::detect(line, "option") {
            if let Some(i) = it.find(' ') {
                let k = &it[..i];
                let mut v = &it[i + 1..];
                if v.starts_with('"') {
                    v = &v[1..];
                }

                return Ok(Line::Option {
                    name: k.to_string(),
                    value: v.to_string(),
                });
            }
        }
        Err(format_err!("unknown line {}", line))
    }
}
