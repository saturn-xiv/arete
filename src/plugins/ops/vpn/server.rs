use askama::Template;

#[derive(Template)]
#[template(path = "openvpn/server.conf", escape = "none")]
pub struct Config<'a> {
    pub port: u16,
    pub server: &'a Server<'a>,
    pub client: &'a Client<'a>,
}

pub struct Server<'a> {
    pub network: &'a str,
    pub netmask: &'a str,
    pub ip: &'a str,
}

pub struct Client<'a> {
    pub network: &'a str,
    pub netmask: &'a str,
    pub dns1: &'a str,
    pub dns2: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/dnsmasq.conf", escape = "none")]
pub struct Dnsmasq<'a> {
    pub ip: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/firewall.sh", escape = "none")]
pub struct Firewall<'a> {
    pub network: &'a str,
    pub interface: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/sysctl.conf", escape = "none")]
pub struct Sysctl;

#[derive(Template)]
#[template(path = "openvpn/readme.md", escape = "none")]
pub struct Readme;

#[derive(Template)]
#[template(path = "openvpn/script/auth.sh", escape = "none")]
pub struct Auth<'a> {
    pub token: &'a str,
    pub host: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/script/connect.sh", escape = "none")]
pub struct Connect<'a> {
    pub token: &'a str,
    pub host: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/script/disconnect.sh", escape = "none")]
pub struct Disconnect<'a> {
    pub token: &'a str,
    pub host: &'a str,
}
