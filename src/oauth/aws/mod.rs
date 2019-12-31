pub mod s3;
pub mod sqs;

use rusoto_credential::StaticProvider;

/// https://console.aws.amazon.com/iam/home
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Credentials {
    #[serde(rename = "Access key ID")]
    pub access_key_id: String,
    #[serde(rename = "Secret access key")]
    pub secret_access_key: String,
}

impl Credentials {
    pub fn provider(self) -> StaticProvider {
        StaticProvider::new_minimal(self.access_key_id, self.secret_access_key)
    }
}
