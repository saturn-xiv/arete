// Long and nested future chains can quickly result in large generic types.
#![type_length_limit = "16777216"]

extern crate arete;
extern crate env_logger;
extern crate uuid;

use arete::queue::Queue;

#[test]
fn it_publish() {
    env_logger::init();
    let mq = arete::queue::rabbitmq::Config::default().open().unwrap();
    for i in 1..=10 {
        let queue = "test".to_string();
        let id = uuid::Uuid::new_v4().to_string();
        println!("publish message {}@{}", id, queue);
        mq.publish(queue, id, format!("Hello, arete-{}", i))
            .unwrap();
    }
}
