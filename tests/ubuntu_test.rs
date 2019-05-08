extern crate arete;
extern crate askama;

use arete::sys::ubuntu::{Ether, Interfaces, Wifi, WpaSupplicant};
use askama::Template;

const ETH: &'static str = "eth0";
const WLAN: &'static str = "wlan0";

#[test]
fn interfaces() {
    for it in vec![
        Interfaces {
            ether: None,
            wifi: None,
        },
        Interfaces {
            ether: Some((ETH.to_string(), Ether::Dhcp)),
            wifi: Some(WLAN.to_string()),
        },
        Interfaces {
            ether: Some((
                ETH.to_string(),
                Ether::Static {
                    address: "192.168.1.10".to_string(),
                    netmask: "255.255.255.0".to_string(),
                    gateway: "192.168.1.0".to_string(),
                    dns1: "8.8.8.8".to_string(),
                    dns2: "8.8.8.8".to_string(),
                },
            )),
            wifi: Some(WLAN.to_string()),
        },
    ] {
        println!(
            "---- interfaces {:?} ----\n{}\n--------",
            it,
            it.render().unwrap()
        )
    }
}

#[test]
fn wifi() {
    for it in vec![
        WpaSupplicant {
            wifi: Wifi::Open {
                ssid: "open".to_string(),
            },
        },
        WpaSupplicant {
            wifi: Wifi::Psk {
                ssid: "psk".to_string(),
                password: "password".to_string(),
            },
        },
        WpaSupplicant {
            wifi: Wifi::Eap {
                ssid: "eap".to_string(),
                identity: "username".to_string(),
                password: "password".to_string(),
            },
        },
    ] {
        println!(
            "---- wpa_supplicant.conf {:?} ----\n{}\n--------",
            it,
            it.render().unwrap()
        )
    }
}
