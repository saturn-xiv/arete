// Long and nested future chains can quickly result in large generic types.
#![type_length_limit = "16777216"]

extern crate arete;
extern crate failure;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate tokio;
extern crate tokio_core;

use failure::Error;
use futures::future::Future;
use futures::Stream;
use lapin::channel::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::client::ConnectionOptions;
use lapin::types::FieldTable;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

#[test]
fn it_publish() {
    consumer();
}

fn consumer() {
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
                .and_then(|(client, heartbeat)| {
                    // The heartbeat future should be run in a dedicated thread so that nothing can prevent it from
                    // dispatching events on time.
                    // If we ran it as part of the "main" chain of futures, we might end up not sending
                    // some heartbeats if we don't poll often enough (because of some blocking task or such).
                    tokio::spawn(heartbeat.map_err(|_| ()));

                    // create_channel returns a future that is resolved
                    // once the channel is successfully created
                    client.create_channel().map_err(Error::from)
                })
                .and_then(|channel| {
                    let id = channel.id;
                    println!("created channel with id: {}", id);

                    let ch = channel.clone();
                    channel
                        .queue_declare("hello", QueueDeclareOptions::default(), FieldTable::new())
                        .and_then(move |queue| {
                            println!("channel {} declared queue {}", id, "hello");

                            // basic_consume returns a future of a message
                            // stream. Any time a message arrives for this consumer,
                            // the for_each method would be called
                            channel.basic_consume(
                                &queue,
                                "my_consumer",
                                BasicConsumeOptions::default(),
                                FieldTable::new(),
                            )
                        })
                        .and_then(|stream| {
                            println!("got consumer stream");

                            stream.for_each(move |message| {
                                println!("got message: {:?}", message);
                                println!(
                                    "decoded message: {:?}",
                                    std::str::from_utf8(&message.data).unwrap()
                                );
                                ch.basic_ack(message.delivery_tag, false)
                            })
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("runtime exited with error");
}
