pub mod api;
pub mod html;
pub mod seo;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push((
            "/api/users",
            routes![
                api::users::sign_in,
                api::users::sign_up,
                api::users::confirm,
                api::users::confirm_token,
                api::users::forgot_password,
                api::users::unlock,
                api::users::unlock_token,
                api::users::reset_password,
                api::users::logs,
                api::users::get_profile,
                api::users::post_profile,
                api::users::change_password,
                api::users::sign_out
            ],
        ));
        items.push(("/api/locales", routes![api::locales::index]));
        items.push(("/api/leave-words", routes![api::leave_words::create]));
        items.push((
            "/api/admin",
            routes![
                api::admin::locales::update,
                api::admin::locales::destory,
                api::admin::leave_words::index,
                api::admin::leave_words::destory
            ],
        ));
        items.push(("/api", routes![api::about]));
        items.push((
            "/",
            routes![
                seo::robots::txt,
                seo::sitemap::xml_gz,
                seo::rss::atom,
                html::ueditor,
                html::home,
            ],
        ));
        items
    };
}
