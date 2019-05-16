use askama::Template;

#[derive(Template)]
#[template(path = "openvpn/client.conf", escape = "none")]
pub struct Config<'a> {
    pub port: u16,
    pub host: &'a str,
}

#[derive(Template)]
#[template(path = "openvpn/auth.txt", escape = "none")]
pub struct Auth<'a> {
    pub user: &'a str,
    pub password: &'a str,
}
