use super::super::super::errors::Result;

// https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol

pub struct Discover {}

impl Discover {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
