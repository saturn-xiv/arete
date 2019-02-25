pub mod album;
pub mod cbeta;
pub mod forum;
pub mod nut;
pub mod ops;
pub mod survey;
pub mod vip;
pub mod wiki;

use super::http::Router;

lazy_static! {
    pub static ref ROUTER: Router = {
        let it = Router::new();
        it
    };
}
