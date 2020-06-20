extern crate arete;
extern crate serde_xml_rs;

use std::fs::read_to_string;
use std::path::PathBuf;

use arete::sys::nmap::{Cidr, Run};

#[test]
fn it_cidr() {
    for i in 0..32 + 1 {
        let it = Cidr(i);
        let mask = it.to_string();
        println!("{} {}", i, mask);
        assert_eq!(i, mask.parse::<Cidr>().unwrap().0)
    }
}

#[test]
fn it_map() {
    let it: Run = serde_xml_rs::from_str(
        &read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("nmap.xml"),
        )
        .unwrap(),
    )
    .unwrap();

    println!("{:?}", it);
}
