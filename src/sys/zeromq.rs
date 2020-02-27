use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use super::super::errors::Result;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum Queue {
    Tcp(Option<String>, u16),
    Ipc(String),
}

impl Queue {
    pub fn server(&self) -> String {
        match self {
            Self::Tcp(ref host, port) => format!(
                "tcp://{}:{}",
                match host {
                    Some(v) => v,
                    None => "*",
                },
                port
            ),
            Self::Ipc(path) => format!(
                "ipc://tmp/{}.sock",
                percent_encode(path.as_bytes(), NON_ALPHANUMERIC)
            ),
        }
    }
    pub fn client(&self) -> String {
        match self {
            Self::Tcp(ref host, port) => format!(
                "tcp://{}:{}",
                match host {
                    Some(v) => v,
                    None => "127.0.0.1",
                },
                port
            ),
            Self::Ipc(path) => format!(
                "ipc://tmp/{}.sock",
                percent_encode(path.as_bytes(), NON_ALPHANUMERIC)
            ),
        }
    }
}

impl Queue {
    pub const SNDHWM: i32 = 1 << 12;
    pub fn sub(&self, topic: Option<String>) -> Result<zmq::Socket> {
        let url = self.client();
        info!("open sub socket to {}", url);
        let ctx = zmq::Context::new();
        let sck = ctx.socket(zmq::SUB)?;
        if let Self::Tcp(_, _) = self {
            // http://api.zeromq.org/3-2:zmq-setsockopt
            sck.set_tcp_keepalive(1)?;
            // TCP_KEEPINTVL 多久没有发送数据时，开始发送Keep-Alive包的时间，也就是链路空闲时间
            sck.set_tcp_keepalive_intvl(30)?;
            // TCP_KEEPCNT 连续发送多少次keep alive probe，对方没有回应，认为连接已经失效的重试次数
            sck.set_tcp_keepalive_cnt(2)?;
            // TCP_KEEPIDLE 发送Keep-Alive probe后，对方多久没有回应，然后重新再发送keep alive probe的时间间隔
            sck.set_tcp_keepalive_idle(5)?;
        }
        sck.connect(&url)?;
        sck.set_subscribe(match topic {
            Some(ref topic) => topic.as_bytes(),
            None => b"",
        })?;
        Ok(sck)
    }

    pub fn pub_(&self) -> Result<zmq::Socket> {
        let url = self.server();
        info!("open pub socket to {}", url);
        let ctx = zmq::Context::new();
        let sck = ctx.socket(zmq::PUB)?;
        sck.set_sndhwm(Self::SNDHWM)?;
        sck.bind(&url)?;
        Ok(sck)
    }
    pub fn push(&self) -> Result<zmq::Socket> {
        let url = self.client();
        info!("open push socket to {}", url);
        let ctx = zmq::Context::new();
        let sck = ctx.socket(zmq::PUSH)?;
        sck.set_sndhwm(Self::SNDHWM)?;
        sck.connect(&url)?;
        Ok(sck)
    }
    pub fn pull(&self) -> Result<zmq::Socket> {
        let url = self.server();
        info!("open pull socket to {}", url);
        let ctx = zmq::Context::new();
        let sck = ctx.socket(zmq::PULL)?;
        sck.bind(&url)?;
        Ok(sck)
    }
}
