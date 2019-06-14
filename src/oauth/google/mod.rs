pub mod openid;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use failure::Error as FailureError;
use rand::Rng;
use reqwest::Client;
use rocket::http::Status;
use serde::de::DeserializeOwned;
use url::{form_urlencoded, Url};

use super::super::errors::{Error, Result};

/// https://developers.google.com/identity/protocols/OAuth2WebServer
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientSecret {
    pub web: Web,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Web {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub javascript_origins: Vec<String>,
}

/// https://developers.google.com/photos/library/guides/authentication-authorization
/// https://developers.google.com/identity/protocols/googlescopes
pub enum Scope {
    YoutubeReadonly,
    PhotosLibraryReadonly,
    Profile,
    Openid,
    Email,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Scope::YoutubeReadonly => "https://www.googleapis.com/auth/youtube.readonly",
                Scope::PhotosLibraryReadonly => {
                    "https://www.googleapis.com/auth/photoslibrary.readonly"
                }
                Scope::Profile => "profile",
                Scope::Openid => "openid",
                Scope::Email => "email",
            }
        )
    }
}

pub enum AccessType {
    Online,
    Offline,
}

impl Default for AccessType {
    fn default() -> Self {
        AccessType::Online
    }
}

impl fmt::Display for AccessType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                AccessType::Online => "online",
                AccessType::Offline => "offline",
            }
        )
    }
}

pub trait Get: DeserializeOwned {
    fn url() -> &'static str;
}

impl Web {
    pub fn oauth2(&self, scope: Vec<Scope>, redirect_uri: &str) -> String {
        let mut rng = rand::thread_rng();

        form_urlencoded::Serializer::new(
            "https://accounts.google.com/o/oauth2/v2/auth?".to_string(),
        )
        .append_pair("client_id", &self.client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair(
            "scope",
            &scope
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
        .append_pair("access_type", &AccessType::default().to_string())
        .append_pair("state", &rng.gen::<u32>().to_string())
        .append_pair("include_granted_scopes", &true.to_string())
        .append_pair("response_type", Code::CODE)
        .finish()
    }

    pub fn get<Q: Get>(&self, token: &str) -> Result<Q> {
        let mut res = Client::new().get(Q::url()).bearer_auth(token).send()?;
        if res.status().is_success() {
            return Ok(res.json()?);
        }

        Err(format_err!("{:?}", res))
    }

    pub fn exchange_authorization_code(
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

        let mut res = Client::new()
            .post("https://www.googleapis.com/oauth2/v4/token")
            .form(&body)
            .send()?;
        if res.status().is_success() {
            return Ok(res.json()?);
        }

        Err(format_err!("{:?}", res))
    }
}

pub struct Code(pub String);

impl Code {
    const CODE: &'static str = "code";
    const ERROR: &'static str = "error";
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Code {
    type Err = FailureError;

    fn from_str(s: &str) -> Result<Self> {
        let it = Url::parse(s)?;
        let query: HashMap<_, _> = it.query_pairs().into_owned().collect();
        if let Some(v) = query.get(Self::CODE) {
            return Ok(Self(v.to_string()));
        }
        if let Some(v) = query.get(Self::ERROR) {
            return Err(format_err!("{}", v));
        }
        Err(Error::Http(Status::BadRequest).into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationCode {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
}
