use std::fs::File;
use std::io::prelude::*;
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub wifi: Option<(String, Wifi)>,
    pub ether: Option<(String, Ether)>,
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
