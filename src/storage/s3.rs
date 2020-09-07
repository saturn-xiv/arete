use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{
    CreateBucketRequest, DeleteObjectRequest, GetBucketLocationRequest, HeadBucketRequest,
    PutObjectRequest, S3Client, S3,
};

use super::super::errors::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub region: String,
    pub endpoint: Option<String>,
    pub access_key: String,
    pub secret_key: String,
}

impl Config {
    fn open(&self) -> Result<S3Client> {
        let it = S3Client::new_with(
            HttpClient::new()?,
            StaticProvider::new(self.access_key.clone(), self.access_key.clone(), None, None),
            match self.endpoint {
                Some(ref v) => Region::Custom {
                    name: self.region.clone(),
                    endpoint: v.to_string(),
                },
                None => self.region.parse()?,
            },
        );
        Ok(it)
    }

    pub async fn put(&self, bucket: &str, name: &str, body: Vec<u8>) -> Result<()> {
        let client = self.open()?;
        if client
            .head_bucket(HeadBucketRequest {
                bucket: bucket.to_string(),
            })
            .await
            .is_err()
        {
            client
                .create_bucket(CreateBucketRequest {
                    bucket: bucket.to_string(),
                    ..Default::default()
                })
                .await?;
        }
        client
            .put_object(PutObjectRequest {
                key: name.to_string(),
                body: Some(body.into()),
                ..Default::default()
            })
            .await?;
        Ok(())
    }

    // https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region
    pub async fn get(&self, bucket: &str, name: &str) -> Result<String> {
        if let Some(ref endpoint) = self.endpoint {
            return Ok(format!("{}/{}/{}", endpoint, bucket, name));
        }
        let client = self.open()?;
        let val = client
            .get_bucket_location(GetBucketLocationRequest {
                bucket: bucket.to_string(),
            })
            .await?;
        Ok(format!(
            "https://s3-{}.amazonaws.com/{}/{}",
            match val.location_constraint {
                Some(ref v) => v,
                None => "",
            },
            bucket,
            name
        ))
    }

    pub async fn delete(&self, bucket: &str, name: &str) -> Result<()> {
        let client = self.open()?;
        client
            .delete_object(DeleteObjectRequest {
                bucket: bucket.to_string(),
                key: name.to_string(),
                ..Default::default()
            })
            .await?;
        Ok(())
    }
}
