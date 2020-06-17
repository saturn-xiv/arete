/// https://wiki.debian.org/NetworkConfiguration
/// https://wiki.debian.org/WiFi/HowToUse
/// http://jorisvr.nl/wpapsk.html
/// http://manpages.ubuntu.com/manpages/disco/man5/wpa_supplicant.conf.5.html
/// https://netplan.io/examples

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
use std::fmt;
use std::fs::read_to_string;
use std::path::{Component, Path, PathBuf};

use askama::Template;
use yaml_rust::{yaml::Hash, Yaml, YamlEmitter};

use super::super::super::errors::Result;

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

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "network.sh", escape = "none")]
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
            if let Some(v) = super::ip4(&n)? {
                return Ok(format!("{}", v));
            }
        }
        if let Some((ref n, _)) = self.wifi {
            if let Some(v) = super::ip4(&n)? {
                return Ok(format!("{}", v));
            }
        }
        Err(format_err!("network isn't enable"))
    }

    pub fn escape(mut self) {
        if let Some((n, w)) = self.wifi {
            match w {
                Wifi::Psk { ssid, .. } => {
                    self.wifi = Some((
                        n,
                        Wifi::Psk {
                            ssid,
                            password: "".to_string(),
                        },
                    ))
                }
                Wifi::Eap { ssid, identity, .. } => {
                    self.wifi = Some((
                        n,
                        Wifi::Eap {
                            ssid,
                            identity,
                            password: "".to_string(),
                        },
                    ))
                }
                _ => {}
            }
        }
    }
}

pub enum Renderer {
    Networkd,
    NetworkManager,
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Networkd => "networkd",
                Self::NetworkManager => "NetworkManager",
            }
        )
    }
}

impl Interface {
    pub const ETHER_METRIC: i64 = 50;
    pub const WIFI_METRIC: i64 = 200;
    pub fn netplan(&self, renderer: &Renderer) -> Result<String> {
        let mut network = Hash::new();
        network.insert(Yaml::String("version".to_string()), Yaml::Integer(2));
        network.insert(
            Yaml::String("renderer".to_string()),
            Yaml::String(renderer.to_string()),
        );

        if let Some((ref n, ref e)) = self.ether {
            let mut ethernets = Hash::new();

            let mut eth = Hash::new();
            match e {
                Ether::Static {
                    address,
                    gateway,
                    dns1,
                    dns2,
                    ..
                } => {
                    eth.insert(
                        Yaml::String("addresses".to_string()),
                        // Yaml::Array(vec![Yaml::String(format!("{}/{}", address, netmask))]),
                        // FIXME
                        Yaml::Array(vec![Yaml::String(format!("{}/24", address))]),
                    );
                    eth.insert(
                        Yaml::String("gateway4".to_string()),
                        Yaml::String(gateway.clone()),
                    );
                    {
                        let mut nameservers = Hash::new();
                        nameservers.insert(
                            Yaml::String("addresses".to_string()),
                            Yaml::Array(vec![
                                Yaml::String(dns1.clone()),
                                Yaml::String(dns2.clone()),
                            ]),
                        );

                        eth.insert(
                            Yaml::String("nameservers".to_string()),
                            Yaml::Hash(nameservers),
                        );
                    }
                }
                Ether::Dhcp => {
                    eth.insert(Yaml::String("dhcp4".to_string()), Yaml::Boolean(true));
                    {
                        let mut overrides = Hash::new();
                        overrides.insert(
                            Yaml::String("route-metric".to_string()),
                            Yaml::Integer(Self::ETHER_METRIC),
                        );
                        eth.insert(
                            Yaml::String("dhcp4-overrides".to_string()),
                            Yaml::Hash(overrides),
                        );
                    }
                }
            };

            ethernets.insert(Yaml::String(n.clone()), Yaml::Hash(eth));
            network.insert(Yaml::String("ethernets".to_string()), Yaml::Hash(ethernets));
        }

        if let Some((ref n, ref w)) = self.wifi {
            let mut wifis = Hash::new();
            let mut wlan = Hash::new();

            let mut access_points = Hash::new();
            match w {
                Wifi::Open { ssid } => {
                    let it = Hash::new();
                    access_points.insert(Yaml::String(ssid.clone()), Yaml::Hash(it));
                }
                Wifi::Psk { ssid, password } => {
                    let mut it = Hash::new();
                    it.insert(
                        Yaml::String("password".to_string()),
                        Yaml::String(password.clone()),
                    );
                    access_points.insert(Yaml::String(ssid.clone()), Yaml::Hash(it));
                }
                Wifi::Eap {
                    ssid,
                    identity,
                    password,
                } => {
                    let mut it = Hash::new();

                    {
                        let mut auth = Hash::new();
                        auth.insert(
                            Yaml::String("key-management".to_string()),
                            Yaml::String("eap".to_string()),
                        );
                        auth.insert(
                            Yaml::String("method".to_string()),
                            Yaml::String("ttls".to_string()),
                        );
                        // auth.insert(
                        //     Yaml::String("anonymous-identity".to_string()),
                        //     Yaml::String("@internal.example.com".to_string()),
                        // );
                        auth.insert(
                            Yaml::String("identity".to_string()),
                            Yaml::String(identity.clone()),
                        );
                        auth.insert(
                            Yaml::String("password".to_string()),
                            Yaml::String(password.clone()),
                        );
                        it.insert(Yaml::String("auth".to_string()), Yaml::Hash(auth));
                    }
                    access_points.insert(Yaml::String(ssid.clone()), Yaml::Hash(it));
                }
            }
            wlan.insert(
                Yaml::String("access-points".to_string()),
                Yaml::Hash(access_points),
            );
            wlan.insert(Yaml::String("dhcp4".to_string()), Yaml::Boolean(true));
            {
                let mut overrides = Hash::new();
                overrides.insert(
                    Yaml::String("route-metric".to_string()),
                    Yaml::Integer(Self::WIFI_METRIC),
                );
                wlan.insert(
                    Yaml::String("dhcp4-overrides".to_string()),
                    Yaml::Hash(overrides),
                );
            }

            wifis.insert(Yaml::String(n.clone()), Yaml::Hash(wlan));
            network.insert(Yaml::String("wifis".to_string()), Yaml::Hash(wifis));
        }

        let mut doc = Hash::new();
        doc.insert(Yaml::String("network".to_string()), Yaml::Hash(network));

        let mut buf = String::new();
        let mut emitter = YamlEmitter::new(&mut buf);
        emitter.dump(&Yaml::Hash(doc))?;
        Ok(buf)
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
