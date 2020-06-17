pub mod fs;

use super::errors::Result;

pub trait Storage: Send + Sync {
    fn save(&self, name: &str, body: &[u8]) -> Result<String>;
}
