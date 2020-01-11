pub mod vpn;

use std::path::{Component, Path, PathBuf};

lazy_static! {
    pub static ref ROOT: PathBuf = Path::new(&Component::RootDir).join("etc").join("openvpn");
}
