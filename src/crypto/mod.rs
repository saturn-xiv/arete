// https://github.com/RustCrypto/hashes

pub mod sodium;
pub mod ssha512;

use super::errors::Result;

pub trait Encryptor {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;

    fn sum(plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(cipher: &[u8], plain: &[u8]) -> bool;

    fn random(l: usize) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key(pub String);

impl Default for Key {
    fn default() -> Self {
        Self(base64::encode(&sodium::Encryptor::random(32)))
    }
}

impl Into<Result<Vec<u8>>> for Key {
    fn into(self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.0)?;
        Ok(buf)
    }
}
