use std::fmt;
use std::str::FromStr;

use rusoto_core::{request::HttpClient, Region};
use rusoto_s3::{
    CreateBucketRequest, DeleteBucketRequest, DeleteObjectRequest, ListObjectsV2Request,
    PutObjectRequest, S3Client, S3 as S3Provider,
};

use super::super::super::errors::Result;

pub struct S3 {
    client: S3Client,
}

pub enum Acl {
    PublicRead,
}

impl fmt::Display for Acl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Acl::PublicRead => "public-read",
            }
        )
    }
}

impl S3 {
    pub fn new(cred: super::Credentials, region: &str) -> Result<Self> {
        Ok(Self {
            client: S3Client::new_with(
                HttpClient::new()?,
                cred.provider(),
                Region::from_str(region)?,
            ),
        })
    }

    pub fn create_bucket(&self, name: String, acl: &Acl) -> Result<()> {
        self.client
            .create_bucket(CreateBucketRequest {
                acl: Some(acl.to_string()),
                bucket: name,
                ..Default::default()
            })
            .sync()?;
        Ok(())
    }
    pub fn delete_bucket(&self, name: String) -> Result<()> {
        self.client
            .delete_bucket(DeleteBucketRequest { bucket: name })
            .sync()?;
        Ok(())
    }
    pub fn list_buckets(&self) -> Result<Vec<String>> {
        let mut buckets = Vec::new();
        if let Some(items) = self.client.list_buckets().sync()?.buckets {
            for it in items {
                if let Some(it) = it.name {
                    buckets.push(it);
                }
            }
        }
        Ok(buckets)
    }

    pub fn list_objects(&self, bucket: String, after: Option<String>) -> Result<Vec<String>> {
        let mut objects = Vec::new();

        if let Some(items) = self
            .client
            .list_objects_v2(ListObjectsV2Request {
                bucket: bucket,
                start_after: after,
                ..Default::default()
            })
            .sync()?
            .contents
        {
            for it in items {
                if let Some(key) = it.key {
                    objects.push(key);
                }
            }
        }
        Ok(objects)
    }

    pub fn put_object(&self, bucket: String, name: String, body: Vec<u8>, acl: &Acl) -> Result<()> {
        self.client
            .put_object(PutObjectRequest {
                acl: Some(acl.to_string()),
                bucket: bucket,
                key: name,
                body: Some(body.into()),
                ..Default::default()
            })
            .sync()?;
        Ok(())
    }

    pub fn delete_object(&self, bucket: String, name: String) -> Result<()> {
        self.client
            .delete_object(DeleteObjectRequest {
                bucket: bucket,
                key: name,
                ..Default::default()
            })
            .sync()?;
        Ok(())
    }
}
