pub mod fs;
pub mod s3;

use super::errors::Result;

pub trait Storage: Send + Sync {
    fn save(&self, name: &str, body: &[u8]) -> Result<String>;
}
