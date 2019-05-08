/// https://wiki.debian.org/NetworkConfiguration
/// https://wiki.debian.org/WiFi/HowToUse
/// http://jorisvr.nl/wpapsk.html
/// http://manpages.ubuntu.com/manpages/disco/man5/wpa_supplicant.conf.5.html

/// Details of the calculation
///
/// For WPA-PSK encryption, the binary key is derived from the passphrase according to the following formula:
///
///   Key = PBKDF2(passphrase, ssid, 4096, 256)
///
/// The function PBKDF2 is a standardized method to derive a key from a passphrase. It is specified in RFC2898 with a clear explanation on how to compute it. The function needs an underlying pseudorandom function. In the case of WPA, the underlying function is HMAC-SHA1.
/// SHA1 is a function that computes a 160-bit hash from an arbitrary amount of input data. It is clearly explained in RFC3174. HMAC is a standardized method to turn a cryptographic hash function into a keyed message authentication function. It is specified in RFC2104.
///
/// To summarize, the key derivation process involves iterating a HMAC-SHA1 function 4096 times, and then doing that again to produce more key bits.
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::{Component, Path, PathBuf};

use askama::Template;

use super::super::errors::Result;

lazy_static! {
    pub static ref INTERFACES: PathBuf = Path::new(&Component::RootDir)
        .join("etc")
        .join("network")
        .join("interfaces");
    pub static ref WPA_SUPPLICANT: PathBuf = Path::new(&Component::RootDir)
        .join("etc")
        .join("wpa_supplicant")
        .join("wpa_supplicant.conf");
}

pub fn mac(n: &str) -> Result<String> {
    let it = read_to_string(
        Path::new(&Component::RootDir)
            .join("sys")
            .join("class")
            .join("net")
            .join(n)
            .join("address"),
    )?;
    Ok(it.trim().to_string())
}

#[derive(Template, Debug)]
#[template(path = "ubuntu/interfaces", escape = "none")]
pub struct Interfaces {
    pub ether: Option<(String, Ether)>,
    pub wifi: Option<String>,
}

#[derive(Template, Debug)]
#[template(path = "ubuntu/wpa_supplicant.conf", escape = "none")]
pub struct WpaSupplicant {
    pub wifi: Wifi,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub wifi: Option<(String, Wifi)>,
    pub ether: Option<(String, Ether)>,
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            wifi: Some((
                "wlan0".to_string(),
                Wifi::Open {
                    ssid: "open".to_string(),
                },
            )),
            ether: Some(("eth0".to_string(), Ether::Dhcp)),
        }
    }
}

impl Interface {
    pub fn mac(&self) -> Result<String> {
        if let Some((ref n, _)) = self.ether {
            return mac(n);
        }
        if let Some((ref n, _)) = self.wifi {
            return mac(n);
        }
        Err(format_err!("network isn't enable"))
    }

    pub fn ip4(&self) -> Result<String> {
        if let Some((ref n, _)) = self.ether {
            if let Some(v) = super::network::ip4(&n)? {
                return Ok(format!("{}", v));
            }
        }
        if let Some((ref n, _)) = self.wifi {
            if let Some(v) = super::network::ip4(&n)? {
                return Ok(format!("{}", v));
            }
        }
        Err(format_err!("network isn't enable"))
    }

    pub fn escape(mut self) {
        if let Some((n, w)) = self.wifi {
            match w {
                Wifi::Psk { ssid, password: _ } => {
                    self.wifi = Some((
                        n.clone(),
                        Wifi::Psk {
                            ssid: ssid.clone(),
                            password: "".to_string(),
                        },
                    ))
                }
                Wifi::Eap {
                    ssid,
                    identity,
                    password: _,
                } => {
                    self.wifi = Some((
                        n.clone(),
                        Wifi::Eap {
                            ssid: ssid.clone(),
                            identity: identity.clone(),
                            password: "".to_string(),
                        },
                    ))
                }
                _ => {}
            }
        }
    }
}

impl Interface {
    pub fn save(&self) -> Result<()> {
        let mut fd = File::create(&INTERFACES.as_path())?;
        write!(
            &mut fd,
            "{}",
            Interfaces {
                ether: self.ether.clone(),
                wifi: match self.wifi {
                    Some((ref n, _)) => Some(n.clone()),
                    None => None,
                },
            }
            .render()?
        )?;
        if let Some((_, ref w)) = self.wifi {
            let mut fd = File::create(&WPA_SUPPLICANT.as_path())?;
            write!(&mut fd, "{}", WpaSupplicant { wifi: w.clone() }.render()?)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Ether {
    Dhcp,
    Static {
        address: String,
        netmask: String,
        gateway: String,
        dns1: String,
        dns2: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Wifi {
    Open {
        ssid: String,
    },
    Psk {
        ssid: String,
        password: String,
    },
    Eap {
        ssid: String,
        identity: String,
        password: String,
    },
}
