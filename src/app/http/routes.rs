use super::super::super::{errors::Result, plugins::ROUTER};

pub const NAME: &'static str = "routes";
pub const ABOUT: &'static str = "List of all of the available routes";

pub fn run() -> Result<()> {
    println!("{}", ROUTER.to_string());
    Ok(())
}
