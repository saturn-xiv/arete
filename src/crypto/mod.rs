// https://github.com/RustCrypto/hashes

#[cfg(feature = "sodium")]
pub mod sodium;

use super::errors::Result;

#[cfg(feature = "sodium")]
pub use self::sodium::*;

// https://wiki.dovecot.org/Authentication/PasswordSchemes
// https://www.tunnelsup.com/using-salted-sha-hashes-with-dovecot-authentication/
// doveadm pw -t {SSHA256.hex}4a847fefc4f9ab450f16783c5025d64313942a1ceb2599707cdb65940ba901e513fa442f -p pass
pub trait SSha512 {
    fn sum(plain: &[u8], salt: &[u8]) -> String;
    fn verify(cipher: String, plain: &[u8]) -> bool;
}

pub trait Random {
    fn bytes(l: usize) -> Vec<u8>;
}

pub trait Password {
    fn sum(plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(cipher: &[u8], plain: &[u8]) -> bool;
}

pub trait Secret {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key(pub String);

impl Default for Key {
    fn default() -> Self {
        Key(base64::encode(&Crypto::bytes(32)))
    }
}

impl Into<Result<Vec<u8>>> for Key {
    fn into(self) -> Result<Vec<u8>> {
        let buf = base64::decode(&self.0)?;
        Ok(buf)
    }
}
