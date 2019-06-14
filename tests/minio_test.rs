extern crate arete;
extern crate csv;
extern crate rusoto_core;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use arete::oauth::aws;
use rusoto_core::Region;

#[test]
fn it_minio() {
    let record = aws::Credentials {
        access_key_id: "access-key".to_string(),
        secret_access_key: "secret-key".to_string(),
    };
    println!("{:?}", record);
    let cli = aws::s3::S3::new(
        record,
        Region::Custom {
            name: "".to_string(),
            endpoint: "http://localhost:9000".to_string(),
        },
    )
    .unwrap();
    for bucket in cli.list_buckets().unwrap() {
        println!("find bucket: {}", bucket);
        for obj in cli.list_objects(bucket, None).unwrap() {
            println!("find object: {}", obj);
        }
    }
}
