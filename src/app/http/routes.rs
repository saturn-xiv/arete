use rocket::config::{ConfigBuilder, Environment, LoggingLevel};

use super::super::super::errors::Result;

pub const NAME: &'static str = "routes";
pub const ABOUT: &'static str = "List of all of the available routes";

pub fn run() -> Result<()> {
    let app = super::rocket(
        ConfigBuilder::new(Environment::Production)
            .log_level(LoggingLevel::Off)
            .finalize()?,
    );
    println!("METHOD\tRANK\tURI");
    app.routes()
        .for_each(|it| println!("{}\t{}\t{}", it.method, it.rank, it.uri));
    Ok(())
}
