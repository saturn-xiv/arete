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
        access_key_id: "x".to_string(),
        secret_access_key: "x".to_string(),
    };
    println!("{:?}", record);
    let cli = aws::sqs::sqs::new(
        record,
        Region::Custom {
            name: "elasticmq".to_string(),
            endpoint: "http://localhost:9324".to_string(),
        },
    )
    .unwrap();
}
