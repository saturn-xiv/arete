extern crate arete;

fn main() {
    if let Err(e) = arete::app::launch() {
        error!("{:?}", e);
    }
}
