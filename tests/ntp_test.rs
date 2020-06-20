extern crate arete;

#[test]
fn it_ntp() {
    println!("{:?}", arete::sys::ntp("0.us.pool.ntp.org:123").unwrap());
}
