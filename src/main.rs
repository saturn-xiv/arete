extern crate arete;
#[macro_use]
extern crate log;

fn main() {
    if let Err(e) = arete::app::launch() {
        error!("{:?}", e);
    }
}
