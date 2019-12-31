use std::collections::HashMap;

use reqwest::Client;

use super::super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub locale: String,
}

impl User {
    pub const ACTION: &'static str = "https://openidconnect.googleapis.com/v1/userinfo";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationCode {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
}

impl AuthorizationCode {
    pub fn token_info(&self) -> String {
        format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            self.id_token
        )
    }
}

impl super::Web {
    pub async fn exchange_authorization_code(
        &self,
        redirect_uri: &str,
        code: &str,
    ) -> Result<AuthorizationCode> {
        let mut body = HashMap::new();
        body.insert("code", code);
        body.insert("client_id", &self.client_id);
        body.insert("client_secret", &self.client_secret);
        body.insert("redirect_uri", redirect_uri);
        body.insert("grant_type", "authorization_code");

        let res = Client::new()
            .post("https://www.googleapis.com/oauth2/v4/token")
            .form(&body)
            .send()
            .await?;
        if res.status().is_success() {
            return Ok(res.json().await?);
        }

        Err(format_err!("{:?}", res))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdToken {
    pub iss: String,
    pub at_hash: Option<String>,
    pub email_verified: bool,
    pub sub: String,
    pub azp: Option<String>,
    pub email: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub name: Option<String>,
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub nonce: Option<String>,
    pub hd: Option<String>,
}
