/// https://wiki.archlinux.org/index.php/OpenVPN
/// https://help.ubuntu.com/lts/serverguide/openvpn.html.en
/// https://zshmobi.com/2018/06/11/fast-reverse-proxy-and-l2tp-vpn-setting/
/// https://zshmobi.com/2018/06/14/openvpn-deployment/
pub mod api;
pub mod client;
pub mod models;
pub mod server;

use std::path::{Component, Path, PathBuf};

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new(&Component::RootDir).join("etc").join("openvpn");
}
