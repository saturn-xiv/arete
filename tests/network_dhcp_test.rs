extern crate arete;

use std::path::PathBuf;

use arete::sys::network::dhcp::Lease;

#[test]
fn isc() {
    let items = Lease::isc(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("dhclient.eth0.leases"),
    )
    .unwrap();
    println!("{:?}", items);
}
