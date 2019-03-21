use rocket::config::{Config as RocketConfig, Environment, LoggingLevel};

use super::super::super::errors::Result;

pub const NAME: &'static str = "routes";
pub const ABOUT: &'static str = "List of all of the available routes";

pub fn run() -> Result<()> {
    let rt = super::rocket(
        RocketConfig::build(Environment::Production)
            .log_level(LoggingLevel::Off)
            .finalize()?,
    );
    println!("{:6} {:4} {}", "METHOD", "RANK", "URI");
    for it in rt.routes() {
        println!("{:6} {:4} {}", it.method, it.rank, it.uri);
    }
    Ok(())
}
