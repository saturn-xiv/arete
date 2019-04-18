pub mod html;
pub mod models;
pub mod themes;

use rocket::Route;

pub fn routes() -> (&'static str, Vec<Route>, Vec<Route>) {
    ("wiki", routes![], routes![html::index, html::show])
}
