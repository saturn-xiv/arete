use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::path::{Component, Path, PathBuf};

use askama::Template;
use ipnetwork::Ipv4Network;

use super::super::super::errors::Result;

lazy_static! {
    pub static ref SYSTEM: PathBuf = Path::new(&Component::RootDir)
        .join("etc")
        .join("systemd")
        .join("system");
    pub static ref NETWORK: PathBuf = Path::new(&Component::RootDir)
        .join("etc")
        .join("systemd")
        .join("network");
    pub static ref WPA_SUPPLICANT: PathBuf = Path::new(&Component::RootDir)
        .join("etc")
        .join("wpa_supplicant");
}

/*

https://wiki.debian.org/SystemdNetworkd
https://www.freedesktop.org/software/systemd/man/systemd.network.html
https://tools.ietf.org/html/rfc4632

wpa_passphrase MyNetwork SuperSecretPassphrase > /etc/wpa_supplicant/wpa_supplicant-wlan0.conf

echo "@reboot root /sbin/wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant-wlan0.conf" > /etc/cron.d/wifi

systemctl enable systemd-networkd
systemctl enable systemd-resolved
ln -sf /run/systemd/resolve/resolv.conf /etc/resolv.conf

/etc/systemd/network/00-wireless-dhcp.network
*/
#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/wpa_supplicant.conf", escape = "none")]
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

impl Wifi {
    pub fn save(&self, name: &str) -> Result<()> {
        let fd = Self::file(name);
        debug!("generate {}", fd.display());
        let buf = self.render()?;
        let mut fd = File::create(fd)?;
        write!(fd, "{}", buf)?;
        Ok(())
    }

    pub fn remove(name: &str) -> Result<()> {
        let fd = Self::file(name);
        if fd.exists() {
            debug!("remove {}", fd.display());
            remove_file(fd)?;
        }
        Ok(())
    }

    fn file(name: &str) -> PathBuf {
        WPA_SUPPLICANT.join(format!("wpa_supplicant-{}.conf", name))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/wpa.service", escape = "none")]
#[serde(rename_all = "camelCase")]
pub struct Wpa;

impl Wpa {
    pub fn save(&self, name: &str) -> Result<()> {
        let fd = SYSTEM.join(format!("wpa_supplicant@{}.service", name));
        debug!("generate {}", fd.display());
        let buf = self.render()?;
        let mut fd = File::create(fd)?;
        write!(fd, "{}", buf)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/dhcp.network", escape = "none")]
#[serde(rename_all = "camelCase")]
pub struct Dhcp {
    pub name: String,
    pub metric: u8,
    pub options: Vec<u8>,
}

impl Dhcp {
    pub const WWW: u8 = 72;

    pub fn save(&self, vendor: &str) -> Result<()> {
        let fd = NETWORK.join(&format!("00-{}-{}.network", vendor, self.name));
        debug!("generate {}", fd.display());
        let buf = self.render()?;
        let mut fd = File::create(&fd)?;
        write!(fd, "{}", buf)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/static.network", escape = "none")]
#[serde(rename_all = "camelCase")]
pub struct Static {
    pub name: String,
    pub metric: u8,
    pub address: Ipv4Network,
    pub gateway: Ipv4Addr,
    pub dns1: Ipv4Addr,
    pub dns2: Option<Ipv4Addr>,
}

impl Static {
    pub fn save(&self, vendor: &str) -> Result<()> {
        let fd = NETWORK.join(&format!("00-{}-{}.network", vendor, self.name));
        debug!("generate {}", fd.display());
        let buf = self.render()?;
        let mut fd = File::create(&fd)?;
        write!(fd, "{}", buf)?;
        Ok(())
    }
    pub fn new(
        name: &str,
        metric: u8,
        address: &str,
        netmask: &str,
        gateway: &str,
        dns1: &str,
        dns2: Option<&str>,
    ) -> Result<Self> {
        let it = Self {
            name: name.to_string(),
            metric,
            address: Ipv4Network::with_netmask(address.parse()?, netmask.parse()?)?,
            gateway: gateway.parse()?,
            dns1: dns1.parse()?,
            dns2: match dns2 {
                Some(v) => Some(v.parse()?),
                None => None,
            },
        };
        Ok(it)
    }
}
