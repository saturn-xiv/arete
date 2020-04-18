use askama::Template;

#[derive(Template)]
#[template(path = "dovecot/dovecot.conf", escape = "none")]
pub struct Main<'a> {
    pub domain: &'a str,
}

#[derive(Template)]
#[template(path = "dovecot/dovecot-sql.conf.ext", escape = "none")]
pub struct Sql<'a> {
    pub host: &'a str,
    pub port: u16,
    pub name: &'a str,
    pub user: &'a str,
    pub password: &'a str,
}

#[derive(Template)]
#[template(path = "dovecot/conf.d/10-auth.conf", escape = "none")]
pub struct Auth {}

#[derive(Template)]
#[template(path = "dovecot/conf.d/10-mail.conf", escape = "none")]
pub struct Mail {}

#[derive(Template)]
#[template(path = "dovecot/conf.d/10-master.conf", escape = "none")]
pub struct Master {}

#[derive(Template)]
#[template(path = "dovecot/conf.d/10-ssl.conf", escape = "none")]
pub struct Ssl<'a> {
    pub domain: &'a str,
}

#[derive(Template)]
#[template(path = "dovecot/conf.d/auth-sql.conf.ext", escape = "none")]
pub struct AuthSql {}

#[derive(Template)]
#[template(path = "dovecot/conf.d/auth-system.conf.ext", escape = "none")]
pub struct AuthSystem {}
