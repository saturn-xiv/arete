extern crate arete;
extern crate env_logger;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    if let Err(e) = arete::app::launch() {
        error!("{:?}", e);
    }
}
