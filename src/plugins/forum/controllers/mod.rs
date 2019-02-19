pub mod api;
pub mod html;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push((
            "/api/forum",
            routes![
                api::posts::index,
                api::posts::create,
                api::posts::show,
                api::posts::update,
                api::posts::destory,
                api::topics::index,
                api::topics::create,
                api::topics::show,
                api::topics::update,
                api::topics::destory,
            ],
        ));

        items.push(("/forum", routes![]));
        items
    };
}
