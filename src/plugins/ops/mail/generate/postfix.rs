use askama::Template;

#[derive(Template)]
#[template(path = "postfix/main.cf", escape = "none")]
pub struct Main<'a> {
    pub domain: &'a str,
    pub driver: &'a str,
}

#[derive(Template)]
#[template(path = "postfix/master.cf", escape = "none")]
pub struct Master {}

#[derive(Template)]
#[template(path = "postfix/virtual-alias-maps.cf", escape = "none")]
pub struct VirtualAliasMaps<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub password: &'a str,
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "postfix/virtual-mailbox-domains.cf", escape = "none")]
pub struct VirtualMailboxDomains<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub password: &'a str,
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "postfix/virtual-email2email.cf", escape = "none")]
pub struct VirtualEmail2Email<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub password: &'a str,
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "postfix/virtual-mailbox-maps.cf", escape = "none")]
pub struct VirtualMailboxMaps<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub password: &'a str,
    pub name: &'a str,
}
