use std::ops::Deref;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use askama::Template;
use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;
use validator::Validate;

use super::super::super::super::super::{
    crypto::Crypto,
    errors::{JsonResult, Result},
    orm::{Database, ID},
    settings::Dao as SettingDao,
};
use super::super::super::super::nut::api::users::Administrator;
use super::super::{client, models::user::Dao as UserDao, server, ROOT};
use super::Token;

#[derive(Serialize)]
pub struct File {
    pub path: PathBuf,
    pub mode: u32,
    pub content: String,
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub port: u16,
    #[validate(length(min = "1", max = "32"))]
    pub dns: String,
    #[validate(length(min = "1", max = "255"))]
    pub host: String,
    #[validate(length(min = "1", max = "16"))]
    pub ip: String,
    #[validate(length(min = "1", max = "16"))]
    pub interface: String,
    pub server: Server,
    pub client: Client,
}
impl Form {
    const KEY: &'static str = "site.author";

    pub fn server(&self, token: &String) -> Result<Vec<File>> {
        let mut items = Vec::new();

        items.push(File {
            path: Path::new(&Component::RootDir)
                .join("etc")
                .join("sysctl.d")
                .join("openvpn.conf"),
            mode: 0600,
            content: server::Sysctl.render()?,
        });
        items.push(File {
            path: Path::new(&Component::RootDir)
                .join("etc")
                .join("dnsmasq.conf"),
            mode: 0600,
            content: server::Dnsmasq { ip: &self.ip }.render()?,
        });
        items.push(File {
            path: ROOT.join("readme.me"),
            mode: 0600,
            content: server::Readme.render()?,
        });
        items.push(File {
            path: ROOT.join("server.conf"),
            mode: 0600,
            content: server::Config {
                port: self.port,
                server: &server::Server {
                    network: &self.server.network,
                    netmask: &self.server.netmask,
                    ip: &self.ip,
                },
                client: &server::Client {
                    netmask: &self.client.netmask,
                    network: &self.client.network,
                    dns1: &self.ip,
                    dns2: &self.dns,
                },
            }
            .render()?,
        });
        items.push(File {
            path: ROOT.join("script").join("firewall.sh"),
            mode: 0700,
            content: server::Firewall {
                network: &self.client.network,
                interface: &self.interface,
            }
            .render()?,
        });
        items.push(File {
            path: ROOT.join("script").join("auth.sh"),
            mode: 0700,
            content: server::Auth {
                host: &self.host,
                token: token,
            }
            .render()?,
        });
        items.push(File {
            path: ROOT.join("script").join("connect.sh"),
            mode: 0700,
            content: server::Connect {
                host: &self.host,
                token: token,
            }
            .render()?,
        });
        items.push(File {
            path: ROOT.join("script").join("disconnect.sh"),
            mode: 0700,
            content: server::Disconnect {
                host: &self.host,
                token: token,
            }
            .render()?,
        });
        Ok(items)
    }
    pub fn client(&self, user: &String, password: &String) -> Result<Vec<File>> {
        let mut items = Vec::new();
        items.push(File {
            path: ROOT.join("client.conf"),
            mode: 0600,
            content: client::Config {
                host: &self.host,
                port: self.port,
            }
            .render()?,
        });
        items.push(File {
            path: ROOT.join("auth.txt"),
            mode: 0600,
            content: client::Auth {
                user: user,
                password: password,
            }
            .render()?,
        });
        Ok(items)
    }
}

impl Default for Form {
    fn default() -> Self {
        Self {
            port: 1194,
            server: Server::default(),
            client: Client::default(),
            host: "vpn.change-me.com".to_string(),
            ip: "10.1.1.2".to_string(),
            interface: "eth0".to_string(),
            dns: "8.8.8.8".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[validate(length(min = "1", max = "16"))]
    pub network: String,
    #[validate(length(min = "1", max = "16"))]
    pub netmask: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            network: "10.1.1.0".to_string(),
            netmask: "255.255.255.0".to_string(),
        }
    }
}
#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    #[validate(length(min = "1", max = "16"))]
    pub network: String,
    #[validate(length(min = "1", max = "16"))]
    pub netmask: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            network: "192.168.6.0".to_string(),
            netmask: "255.255.255.0".to_string(),
        }
    }
}

#[get("/")]
pub fn get(db: Database, _user: Administrator, enc: State<Arc<Crypto>>) -> JsonResult<Form> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let it: Form = match SettingDao::get(db, enc, &Form::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => Form::default(),
    };
    Ok(Json(it))
}

#[post("/", data = "<form>")]
pub fn post(
    _user: Administrator,
    enc: State<Arc<Crypto>>,
    db: Database,
    form: Json<Form>,
) -> JsonResult<()> {
    form.validate()?;
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let form = form.deref();
    SettingDao::set::<String, Form, Crypto>(db, enc, &Form::KEY.to_string(), form, false)?;
    Ok(Json(()))
}

#[get("/server")]
pub fn server(
    _user: Administrator,
    enc: State<Arc<Crypto>>,
    db: Database,
) -> JsonResult<Vec<File>> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let cfg: Form = SettingDao::get(db, enc, &Form::KEY.to_string())?;
    let token: String = match SettingDao::get(db, enc, &Token::KEY.to_string()) {
        Ok(v) => v,
        Err(_) => {
            let v = Uuid::new_v4().to_string();
            SettingDao::set::<String, String, Crypto>(db, enc, &Token::KEY.to_string(), &v, true)?;
            v
        }
    };
    Ok(Json(cfg.server(&token)?))
}

#[get("/client/<id>")]
pub fn client(
    _user: Administrator,
    enc: State<Arc<Crypto>>,
    id: ID,
    db: Database,
) -> JsonResult<Vec<File>> {
    let db = db.deref();
    let enc = enc.deref();
    let enc = enc.deref();
    let cfg: Form = SettingDao::get(db, enc, &Form::KEY.to_string())?;
    let user = UserDao::by_id(db, id)?;
    Ok(Json(cfg.client(&user.email, &"change-me".to_string())?))
}
