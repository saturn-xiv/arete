// Long and nested future chains can quickly result in large generic types.
#![type_length_limit = "16777216"]

extern crate arete;
extern crate env_logger;

use arete::{errors::Result, queue::Queue};

struct Printer {}

impl arete::queue::Handler for Printer {
    fn handle(&self, id: String, payload: Vec<u8>) -> Result<()> {
        println!("receive message {} {}", id, String::from_utf8(payload)?);
        Ok(())
    }
}

#[test]
fn it_publish() {
    env_logger::init();

    let mq = arete::queue::rabbitmq::Config::default().open().unwrap();
    mq.consume(
        "arete-test".to_string(),
        "test".to_string(),
        Box::new(Printer {}),
    )
    .unwrap();
}
