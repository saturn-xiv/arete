use std::fmt;
use std::process::Command;
use std::str::FromStr;

use failure::{Error, SyncFailure};

use super::super::errors::Result;

/// https://nmap.org/book/man-output.html
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "nmaprun")]
pub struct Run {
    pub verbose: Verbose,
    #[serde(rename = "host", default)]
    pub hosts: Vec<Host>,
    pub scanner: String,
    pub args: String,
    #[serde(rename = "startstr")]
    pub start: String,
    pub version: f32,
    #[serde(rename = "xmloutputversion")]
    pub xml_output_version: f32,
}

impl Run {
    pub fn scan(network: &str, cidr: u8) -> Result<Self> {
        let buf = Command::new("nmap")
            .arg("-dd")
            .arg("-n")
            .arg("-sn")
            .arg("-oX")
            .arg("-")
            .arg(format!("{}/{}", network, cidr))
            .output()?
            .stdout;
        let it = serde_xml_rs::from_str(std::str::from_utf8(&buf)?).map_err(SyncFailure::new)?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Verbose {
    pub level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    #[serde(rename = "address", default)]
    pub addresses: Vec<Address>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "addrtype")]
    pub type_: String,
    #[serde(rename = "addr")]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub state: String,
    pub reason: String,
    #[serde(rename = "reason_ttl")]
    pub ttl: u32,
}

/// https://kb.wisc.edu/page.php?id=3493
/// https://www.forease.net/docs/handbook/appendix-netmask.html
pub struct Cidr(pub u8);

impl Cidr {
    pub const MAX: u8 = 32;
    pub const MIN: u8 = 0;
}

impl FromStr for Cidr {
    type Err = Error;

    fn from_str(mask: &str) -> Result<Self> {
        for i in Self::MIN..Self::MAX + 1 {
            let it = Self(i);
            if it.to_string() == mask {
                return Ok(it);
            }
        }

        Err(format_err!("bad netmask {}", mask))
    }
}

impl fmt::Display for Cidr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 > Self::MAX {
            return Err(fmt::Error);
        }
        let mask: u64 = (0xffffffff >> (Self::MAX - self.0)) << (Self::MAX - self.0);
        write!(
            f,
            "{}.{}.{}.{}",
            (0xff000000 & mask) >> 24,
            (0x00ff0000 & mask) >> 16,
            (0x0000ff00 & mask) >> 8,
            (0x000000ff & mask)
        )
    }
}
