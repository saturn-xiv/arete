extern crate arete;
extern crate csv;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use arete::oauth::aws;

#[test]
fn it_s3() {
    let file = File::open(resource("accessKeys.csv")).unwrap();
    let reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.deserialize() {
        let record: aws::Credentials = result.unwrap();
        println!("{:?}", record);
        let cli = aws::s3::S3::new(record, "us-west-2").unwrap();
        for bucket in cli.list_buckets().unwrap() {
            println!("find bucket: {}", bucket);
            for obj in cli.list_objects(bucket, None).unwrap() {
                println!("find object: {}", obj);
            }
        }
    }
}

fn resource(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("oauth")
        .join("aws")
        .join(file)
}
