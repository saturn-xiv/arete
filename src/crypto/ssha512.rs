// https://wiki.dovecot.org/Authentication/PasswordSchemes
// https://www.tunnelsup.com/using-salted-sha-hashes-with-dovecot-authentication/
// doveadm pw -t {SSHA256.hex}4a847fefc4f9ab450f16783c5025d64313942a1ceb2599707cdb65940ba901e513fa442f -p pass

use base64;
use sodiumoxide::crypto::hash::sha512::{hash, DIGESTBYTES};

pub fn sum(plain: &[u8], salt: &[u8]) -> String {
    base64::encode(&[hash(&[plain, salt].concat()).as_ref(), salt].concat())
}

pub fn verify(cipher: String, plain: &[u8]) -> bool {
    match base64::decode(&cipher) {
        Ok(buf) => cipher == sum(plain, &buf[DIGESTBYTES..]),
        Err(_) => false,
    }
}
