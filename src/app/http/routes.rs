use super::super::super::errors::Result;

pub const NAME: &'static str = "routes";
pub const ABOUT: &'static str = "List of all of the available routes";

pub fn run() -> Result<()> {
    println!("METHOD\tRANK\tURI");
    // app.routes()
    //     .for_each(|it| println!("{}\t{}\t{}", it.method, it.rank, it.uri));
    Ok(())
}
