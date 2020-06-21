extern crate arete;

use arete::sys::network;

#[test]
fn interfaces() {
    for it in network::interfaces().unwrap() {
        println!("{}", it);
    }
}
