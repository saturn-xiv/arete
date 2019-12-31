use std::fmt::Debug;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::errors::Result;

pub struct Publisher {
    socket: zmq::Socket,
}

impl Publisher {
    pub const ALL: &'static str = "*";
    pub fn new(host: &str, port: u16) -> Result<Self> {
        Ok(Self {
            socket: {
                let ctx = zmq::Context::new();
                let sck = ctx.socket(zmq::PUB)?;
                sck.bind(&format!("tcp://{}:{}", host, port))?;
                sck
            },
        })
    }

    pub fn send<K: Serialize + Debug>(&self, payload: &K, to: Vec<String>) -> Result<()> {
        debug!("publish {:?} to {:?}", payload, to);
        let payload = serde_json::to_vec(payload)?;
        for to in to {
            self.socket.send(&to.as_bytes(), zmq::SNDMORE)?;
            self.socket.send(&payload, 0)?;
        }
        Ok(())
    }
}

pub struct Subscriber {
    socket: zmq::Socket,
}

impl Subscriber {
    pub fn new(host: &str, port: u16, topic: &str) -> Result<Self> {
        info!("try to sub from {}:{}", host, port);
        let ctx = zmq::Context::new();
        let sck = ctx.socket(zmq::SUB)?;
        // http://api.zeromq.org/3-2:zmq-setsockopt
        sck.set_tcp_keepalive(1)?;
        sck.set_tcp_keepalive_intvl(30)?;
        sck.set_tcp_keepalive_cnt(3)?;
        sck.set_tcp_keepalive_idle(5)?;
        sck.connect(&format!("tcp://{}:{}", host, port))?;
        sck.set_subscribe(topic.as_bytes())?;

        Ok(Self { socket: sck })
    }

    pub fn receive<V: DeserializeOwned + Debug>(&self) -> Result<V> {
        let evp = self.socket.recv_bytes(0)?;
        let payload = serde_json::from_slice(&self.socket.recv_bytes(0)?)?;
        debug!("receive [{}] {:?}", std::str::from_utf8(&evp)?, payload);
        Ok(payload)
    }
}
