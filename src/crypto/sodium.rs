use sodiumoxide::{
    crypto::{pwhash, secretbox},
    randombytes,
};

use super::super::errors::{Error, Result};
use super::Key;

pub struct Encryptor {
    key: secretbox::Key,
}

impl Encryptor {
    pub fn new(key: Key) -> Result<Self> {
        let key: Result<Vec<u8>> = key.into();
        match secretbox::Key::from_slice(&key?) {
            Some(key) => Ok(Self { key: key }),
            None => Err(Error::SodiumBadKey.into()),
        }
    }
}
impl super::Encryptor for Encryptor {
    fn random(l: usize) -> Vec<u8> {
        randombytes::randombytes(l)
    }

    fn sum(plain: &[u8]) -> Result<Vec<u8>> {
        match pwhash::pwhash(
            plain,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(cip) => Ok(cip[..].to_vec()),
            Err(_) => Err(Error::SodiumHash.into()),
        }
    }

    fn verify(cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let cipher = secretbox::seal(plain, &nonce, &self.key);
        (cipher, nonce[..].to_vec())
    }

    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        match secretbox::Nonce::from_slice(nonce) {
            Some(nonce) => match secretbox::open(cipher, &nonce, &self.key) {
                Ok(buf) => Ok(buf),
                Err(_) => Err(Error::SodiumDecrypt.into()),
            },
            None => Err(Error::SodiumBadNonce.into()),
        }
    }
}
