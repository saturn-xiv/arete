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

#[derive(Serialize, Deserialize, Debug)]
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
        let mut ifs = File::create(&INTERFACES.as_path())?;
        write!(
            &mut ifs,
            r#"
auto lo
iface lo inet loopback
"#
        )?;

        if let Some((ref name, ref ether)) = self.ether {
            match ether {
                Ether::Dhcp => {
                    write!(
                        &mut ifs,
                        r#"
auto {name}
allow-hotplug {name}
iface {name} inet dhcp
"#,
                        name = name
                    )?;
                }
                Ether::Static {
                    address,
                    netmask,
                    gateway,
                    dns1,
                    dns2,
                } => {
                    write!(
                        &mut ifs,
                        r#"
auto {name}
allow-hotplug {name}
iface {name} inet static
  address {address}
  netmask {netmask}
  gateway {gateway}
  dns-nameservers {dns1} {dns2}
"#,
                        name = name,
                        address = address,
                        netmask = netmask,
                        gateway = gateway,
                        dns1 = dns1,
                        dns2 = dns2,
                    )?;
                }
            }
        }

        if let Some((ref name, ref wifi)) = self.wifi {
            let mut wpa = File::create(&WPA_SUPPLICANT.as_path())?;
            write!(
                &mut ifs,
                r#"
auto {name}
allow-hotplug {name}
iface {name} inet dhcp
  wpa-conf {wpa}
"#,
                name = name,
                wpa = WPA_SUPPLICANT.display(),
            )?;

            write!(
                &mut wpa,
                r#"
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=wheel
"#
            )?;

            match wifi {
                Wifi::Open { ssid } => {
                    write!(
                        &mut wpa,
                        r#"
network={{
  ssid = "{ssid}"
  scan_ssid=1
}}
"#,
                        ssid = ssid,
                    )?;
                }
                Wifi::Psk { ssid, password } => {
                    write!(
                        &mut wpa,
                        r#"
network={{
  ssid = "{ssid}"
  scan_ssid=1
  key_mgmt=WPA-PSK
  psk = "{password}"
}}
"#,
                        ssid = ssid,
                        password = password,
                    )?;
                }
                Wifi::Eap {
                    ssid,
                    identity,
                    password,
                } => {
                    write!(
                        &mut wpa,
                        r#"
network={{
  ssid = "{ssid}"
  key_mgmt=WPA-EAP
  pairwise=CCMP TKIP
  group=CCMP TKIP
  eap=TLS
  identity="{identity}"
  ca_cert="/etc/cert/ca.pem"
  client_cert="/etc/cert/user.pem"
  private_key="/etc/cert/user.prv"
  private_key_passwd="{password}"
}}
"#,
                        ssid = ssid,
                        identity = identity,
                        password = password,
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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
