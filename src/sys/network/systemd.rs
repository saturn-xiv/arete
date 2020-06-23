use askama::Template;
use ipnet::Ipv4Net;

/*

https://wiki.debian.org/SystemdNetworkd

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

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/wpa.service", escape = "none")]
#[serde(rename_all = "camelCase")]
pub struct Wpa;

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/dhcp.network", escape = "none")]
#[serde(rename_all = "camelCase")]
pub struct Dhcp {
    pub name: String,
    pub metric: u8,
    pub options: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Template)]
#[template(path = "systemd/static.network", escape = "none")]
#[serde(rename_all = "camelCase")]
pub struct Static {
    pub name: String,
    pub metric: u8,
    pub address: Ipv4Net,
    pub gateway: String,
    pub dns1: String,
    pub dns2: Option<String>,
}
