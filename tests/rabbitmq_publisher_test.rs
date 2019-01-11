// Long and nested future chains can quickly result in large generic types.
#![type_length_limit = "16777216"]

extern crate arete;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate tokio;

use failure::Error;
use futures::future::Future;
use lapin::channel::{
    BasicProperties, BasicPublishOptions, ConfirmSelectOptions, QueueDeclareOptions,
};
use lapin::client::ConnectionOptions;
use lapin::types::FieldTable;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

#[test]
fn it_publish() {
    for _it in 1..=10 {
        publish();
    }
}

fn publish() {
    let addr = "127.0.0.1:5672".parse().unwrap();

    Runtime::new()
        .unwrap()
        .block_on_all(
            TcpStream::connect(&addr)
                .map_err(Error::from)
                .and_then(|stream| {
                    // connect() returns a future of an AMQP Client
                    // that resolves once the handshake is done
                    lapin::client::Client::connect(
                        stream,
                        ConnectionOptions {
                            username: "guest".to_string(),
                            password: "guest".to_string(),
                            vhost: "/arete".to_string(),
                            frame_max: 65535,
                            ..Default::default()
                        },
                    )
                    .map_err(Error::from)
                })
                .and_then(|(client, _ /* heartbeat */)| {
                    // create_channel returns a future that is resolved
                    // once the channel is successfully created
                    client
                        .create_confirm_channel(ConfirmSelectOptions::default())
                        .map_err(Error::from)
                })
                .and_then(|channel| {
                    let id = channel.id;
                    println!("created channel with id: {}", id);

                    // we using a "move" closure to reuse the channel
                    // once the queue is declared. We could also clone
                    // the channel
                    channel
                        .queue_declare("hello", QueueDeclareOptions::default(), FieldTable::new())
                        .and_then(move |_| {
                            println!("channel {} declared queue {}", id, "hello");

                            channel
                                .basic_publish(
                                    "",
                                    "hello",
                                    b"hello from tokio".to_vec(),
                                    BasicPublishOptions::default(),
                                    BasicProperties::default(),
                                )
                                .map(|confirmation| {
                                    println!("publish got confirmation: {:?}", confirmation)
                                })
                                .and_then(move |_| channel.close(200, "Bye"))
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("runtime exited with error");
}
