pub mod html;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push(("/wiki", routes![html::index, html::show]));

        items
    };
}
