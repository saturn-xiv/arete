use super::super::super::errors::Result;

pub const NAME: &str = "routes";
pub const ABOUT: &str = "List of all of the available routes";

pub fn run() -> Result<()> {
    println!("{:6} {:4} URI", "METHOD", "RANK");
    Ok(())
}
