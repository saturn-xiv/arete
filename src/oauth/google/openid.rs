#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub sub: String,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub locale: String,
}

impl super::Get for User {
    fn url() -> &'static str {
        "https://openidconnect.googleapis.com/v1/userinfo"
    }
}
