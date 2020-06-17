use url::form_urlencoded;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub channel_id: String,
    pub channel_secret: String,
    pub callback_url: String,
}

impl Config {
    // https://developers.line.me/en/docs/line-login/login-button/
    pub fn authorization_url(&self, state: &str, nonce: &str) -> String {
        form_urlencoded::Serializer::new(String::from(
            "https://access.line.me/oauth2/v2.1/authorize",
        ))
        .append_pair("response_type", "code")
        .append_pair("client_id", &self.channel_id)
        .append_pair("redirect_uri", &self.callback_url)
        .append_pair("state", state)
        .append_pair("scope", "profile openid email")
        .append_pair("nonce", nonce)
        .finish()
    }
}
