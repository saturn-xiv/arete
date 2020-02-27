extern crate arete;
extern crate env_logger;

use std::thread;
use std::time::Duration;

#[test]
fn it_pub() {
    env_logger::init();
    for i in 0..std::u8::MAX {
        let cli = arete::queue::mosquitto::Client::new("localhost", None).unwrap();
        let msg = format!("Hello, {}", i);
        let to = vec!["hi".to_string()];
        println!("send {} to {:?}", msg, to);
        cli.send(&msg, &to).unwrap();
        thread::sleep(Duration::from_secs(5));
    }
}
