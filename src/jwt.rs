use std::ops::Add;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::errors::Result;

// https://www.ibm.com/support/knowledgecenter/zh/SSEQTP_8.5.5/com.ibm.websphere.wlp.doc/ae/cwlp_jwttoken.html
// https://jwt.io/
// https://tools.ietf.org/html/rfc7519
pub struct Jwt {
    key: String,
}

impl Jwt {
    pub fn new(key: String) -> Self {
        return Self { key: key };
    }
    pub fn timestamps(ttl: Duration) -> (i64, i64) {
        let nbf = Utc::now().naive_utc();
        let exp = nbf.add(ttl);
        (nbf.timestamp(), exp.timestamp())
    }
    pub fn sum<T: Serialize>(&self, kid: Option<String>, claims: &T) -> Result<String> {
        let mut header = Header::default();
        header.kid = kid;
        header.alg = Algorithm::HS512;
        let token = encode(&header, claims, self.key.as_bytes())?;
        Ok(token)
    }
    pub fn parse<T: DeserializeOwned>(&self, token: &String) -> Result<TokenData<T>> {
        let mut vat = Validation::new(Algorithm::HS512);
        vat.leeway = 60;
        let val = decode(token, self.key.as_bytes(), &vat)?;
        Ok(val)
    }
}
