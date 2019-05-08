extern crate arete;
extern crate askama;

use arete::sys::network::setup::{Ether, Interface, Wifi};
use askama::Template;

const ETH: &'static str = "eth0";
const WLAN: &'static str = "wlan0";

#[test]
fn interfaces() {
    for it in vec![
        Interface {
            ether: None,
            wifi: None,
        },
        Interface {
            ether: Some((ETH.to_string(), Ether::Dhcp)),
            wifi: Some((
                WLAN.to_string(),
                Wifi::Open {
                    ssid: "open".to_string(),
                },
            )),
        },
        Interface {
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
            wifi: Some((
                WLAN.to_string(),
                Wifi::Psk {
                    ssid: "psk".to_string(),
                    password: "password".to_string(),
                },
            )),
        },
        Interface {
            ether: None,
            wifi: Some((
                WLAN.to_string(),
                Wifi::Eap {
                    ssid: "eap".to_string(),
                    identity: "username".to_string(),
                    password: "password".to_string(),
                },
            )),
        },
    ] {
        println!("---- model ----\n{:?}\n--------", it);
        println!(
            "---- netplan.yaml  ----\n{}\n--------",
            it.netplan().unwrap()
        );
        println!("---- network.sh  ----\n{}\n--------", it.render().unwrap());
    }
}
