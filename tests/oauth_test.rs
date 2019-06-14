extern crate arete;
extern crate serde_json;

use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use arete::oauth::google;

const CALLBACK: &'static str = "http://localhost:8080/oauth/google/callback";

#[test]
fn it_google() {
    let file = File::open(resource("client_secret.json")).unwrap();
    let reader = BufReader::new(file);
    let client: google::ClientSecret = serde_json::from_reader(reader).unwrap();

    println!(
        "{}",
        client.web.oauth2(
            vec![
                google::Scope::Openid,
                google::Scope::Email,
                google::Scope::Profile,
                google::Scope::PhotosLibraryReadonly,
                google::Scope::YoutubeReadonly
            ],
            CALLBACK,
        )
    );

    let code = google::Code::from_str(&read_to_string(resource("callback.txt")).unwrap()).unwrap();
    println!("{}", code);

    // let token = client
    //     .web
    //     .exchange_authorization_code(CALLBACK, &code.0)
    //     .unwrap();
    // println!("{:?}", token);

    let file = File::open(resource("authorization_code.json")).unwrap();
    let reader = BufReader::new(file);
    let code: google::AuthorizationCode = serde_json::from_reader(reader).unwrap();
    println!("{:?}", code);

    let user: google::openid::User = client.web.get(&code.access_token).unwrap();
    println!("{:?}", user);
}

fn resource(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("oauth")
        .join("google")
        .join(file)
}
