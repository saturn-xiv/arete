use base64;
use sodiumoxide::{
    crypto::{
        hash::sha512::{hash, DIGESTBYTES},
        pwhash, secretbox,
    },
    randombytes,
};

use super::super::errors::Result;
use super::Key;

#[derive(Clone)]
pub struct Crypto {
    key: secretbox::Key,
}

impl Crypto {
    pub fn new(key: Key) -> Result<Self> {
        let key: Result<Vec<u8>> = key.into();
        match secretbox::Key::from_slice(&key?) {
            Some(key) => Ok(Self { key }),
            None => Err(format_err!("bad secret key")),
        }
    }
}
impl super::Random for Crypto {
    fn bytes(l: usize) -> Vec<u8> {
        randombytes::randombytes(l)
    }
}

impl super::Password for Crypto {
    fn sum(plain: &[u8]) -> Result<Vec<u8>> {
        match pwhash::pwhash(
            plain,
            pwhash::OPSLIMIT_INTERACTIVE,
            pwhash::MEMLIMIT_INTERACTIVE,
        ) {
            Ok(cip) => Ok(cip[..].to_vec()),
            Err(_) => Err(format_err!("generate password")),
        }
    }

    fn verify(cipher: &[u8], plain: &[u8]) -> bool {
        match pwhash::HashedPassword::from_slice(cipher) {
            Some(cipher) => pwhash::pwhash_verify(&cipher, plain),
            None => false,
        }
    }
}

impl super::Secret for Crypto {
    fn encrypt(&self, plain: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let cipher = secretbox::seal(plain, &nonce, &self.key);
        (cipher, nonce[..].to_vec())
    }

    fn decrypt(&self, cipher: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        match secretbox::Nonce::from_slice(nonce) {
            Some(nonce) => match secretbox::open(cipher, &nonce, &self.key) {
                Ok(buf) => Ok(buf),
                Err(_) => Err(format_err!("bad nonce")),
            },
            None => Err(format_err!("empty nonce")),
        }
    }
}

impl super::SSha512 for Crypto {
    fn sum(plain: &[u8], salt: &[u8]) -> String {
        base64::encode(&[hash(&[plain, salt].concat()).as_ref(), salt].concat())
    }

    fn verify(cipher: &str, plain: &[u8]) -> bool {
        match base64::decode(cipher) {
            Ok(buf) => cipher == Self::sum(plain, &buf[DIGESTBYTES..]),
            Err(_) => false,
        }
    }
}
